use std::sync::{Arc, LazyLock};
use std::u32;
use futures::future::BoxFuture;
use regex::Regex;
use scraper::Selector;
use utilites::http::{HyperClient, Uri};
use crate::{create_parser, create_plugin};
use super::plugin_trait::Number;
use super::{signatory_authorites, OffSiteParser};
use super::NumberExtractorPlugin;

//Some("https://npa.bashkortostan.ru"),
const GLAVA_PATTERN: &'static str = r"(?<prefix>УГ-)(?<number>\d{1,4})";

create_plugin!(HeadPlugin,
    signatory_authorites::ГЛАВА_РЕСПУБЛИКИ_БАШКОРТОСТАН,
    GLAVA_PATTERN);

create_parser!(BashOffSiteParser,
    "https://npa.bashkortostan.ru",
    "https://npa.bashkortostan.ru",
    [
        GLAVA_PATTERN
    ], parse);

async fn parse(regexes: Arc<LazyLock<Vec<Regex>>>, api_url: &str, sa: &str, _act_type: &str, year: u32, sender: Option<tokio::sync::mpsc::Sender<String>>) 
-> Result<Vec<String>, crate::error::ExtractorError>
{
    let mut page = 1;
    let mut html = query(year, page, sa, api_url).await?;
    let mut numbers = Vec::new();
    while let Some(items) = scrap(&html)
    {
        for name in items
        {
            for re in regexes.iter()
            {
                if let Some(mch) = re.find(&name)
                {
                    numbers.push(mch.as_str().to_owned());
                }
            }
        }
        if let Some(s) = sender.as_ref()
        {
            let _ = s.send(["Получение данных с ".to_owned(), api_url.to_owned(), " стр. № ".to_owned(), page.to_string()].concat()).await;
        }
        page += 1;
        html = query(year, page, sa, api_url).await?;
    }
    Ok(numbers)
}
fn client(uri: &str) -> HyperClient
{
    let uri: Uri = uri.parse().unwrap();
    utilites::http::HyperClient::new(uri)
}

fn scrap(html: &String) -> Option<Vec<String>>
{
    let parsed = scraper::Html::parse_document(html);
    let doc_item_selector = Selector::parse("a[class=\"documents__item-name\"]").unwrap();
    let frag = parsed.select(&doc_item_selector);
    let items = frag.map(|f| f.inner_html()).collect::<Vec<String>>();
    if items.len() > 0
    {
        Some(items)
    }
    else 
    {
        None    
    }
}
async fn query(year: u32, page: u32, sa: &str, uri: &str) -> Result<String, super::super::error::ExtractorError>
{
    //https://npa.bashkortostan.ru/?filter_name=&filter_type=5&filter_organization=92&filter_reg_date_from=01.01.2024&filter_reg_date_to=31.12.2024&filter_pub_date_from=&filter_pub_date_to=&filter_reg_number=%D0%A3%D0%93-12
    let client = client(uri);
    //nav-documents=page-2
    let start_date = ["01.01.".to_owned(), year.to_string()].concat();
    let end_date = ["31.12.".to_owned(), year.to_string()].concat();
    let params = match sa
    {
        signatory_authorites::ГЛАВА_РЕСПУБЛИКИ_БАШКОРТОСТАН => 
        &[
            ("filter_type", "5"),
            ("filter_organization", "92"),
            ("filter_reg_date_from", &start_date),
            ("filter_reg_date_to", &end_date),
            ("nav-documents", &["page-", &page.to_string()].concat()),
        ],
        signatory_authorites::РЕСПУБЛИКА_БАШКОРТОСТАН => 
        &[
            ("filter_type", "4"),
            ("filter_organization", ""),
            ("filter_reg_date_from", &start_date),
            ("filter_reg_date_to", &end_date),
            ("nav-documents", &["page-", &page.to_string()].concat()),
        ],
        _ => 
        &[
            ("filter_type", "5"),
            ("filter_organization", "92"),
            ("filter_reg_date_from", &start_date),
            ("filter_reg_date_to", &end_date),
            ("nav-documents", &["page-", &page.to_string()].concat()),
        ]
    };
    let req = client.get_with_params(params).await?;
    let html = String::from_utf8(req.1.to_vec())?;
    Ok(html)
}

#[cfg(test)]
mod tests
{
    use scraper::Selector;
    #[tokio::test]
    async fn test_raw_numbers()
    {
        let _ = logger::StructLogger::new_default();
        let _q = super::query(2024, 1, "", "https://npa.bashkortostan.ru").await;
    }

    #[test]
    fn test_scrapper()
    {
        let _ = logger::StructLogger::new_custom(logger::LevelFilter::Debug, Some(&[("html5ever", logger::LevelFilter::Info), ("selectors::matching", logger::LevelFilter::Info)]));
        let bytes = include_bytes!("./bash_off_site_html_response.html");
        let html = String::from_utf8_lossy(bytes);
        let parsed = scraper::Html::parse_document(html.as_ref());
        let doc_item = Selector::parse("a[class=\"documents__item-name\"]").unwrap();
        let frag = parsed.select(&doc_item);
        for href in frag
        {
            logger::debug!("документ: {:?}",  href.inner_html());
        }
    }
}