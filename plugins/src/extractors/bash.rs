use std::cell::OnceCell;
use std::u32;
use futures::future::BoxFuture;
use logger::info;
use regex::Match;
use scraper::Selector;
use utilites::http::{HyperClient, Uri};
use crate::{create_plugin, ExtractorError};
use super::plugin_trait::Number;
use super::{number_extractors, signatory_authorites, types, OffSiteParser};
use super::NumberExtractorPlugin;

//Some("https://npa.bashkortostan.ru"),

create_plugin!(HeadPlugin,
    signatory_authorites::ГЛАВА_РЕСПУБЛИКИ_БАШКОРТОСТАН,
    r"(?<prefix>УГ-)(?<number>\d{1,4})");

create_plugin!(RegionPlugin,
    signatory_authorites::РЕСПУБЛИКА_БАШКОРТОСТАН,
    r"(?<number>\d{1,4})(?<postfix>-з)");


pub struct BashOffSiteParser{}
impl OffSiteParser for BashOffSiteParser
{
    fn official_publication_url(&self) -> &'static str
    {
        "https://npa.bashkortostan.ru"
    }
    fn check_numbers_on_alternative_site<'a>(&'a self, sa: &'a str, _act_type: &'a str, year: u32) 
    -> BoxFuture<'a, Result<Vec<String>, crate::error::ExtractorError>>
    {
        Box::pin(async move 
        {
            let regexes: Option<std::sync::LazyLock<Vec<regex::Regex>>> = match sa
            {
                signatory_authorites::ГЛАВА_РЕСПУБЛИКИ_БАШКОРТОСТАН => Some(HeadPlugin::get_regexes()),
                signatory_authorites::РЕСПУБЛИКА_БАШКОРТОСТАН => Some(RegionPlugin::get_regexes()),
                _ => None
            };
            if regexes.is_none()
            {
                return Err(ExtractorError::ActTypeNotSupported(["Не найден регекс к текущему органу: ", sa].concat()));
            }
            let regexes = regexes.unwrap();
            let mut page = 1;
            let mut html = query(year, page, sa, self.official_publication_url()).await?;
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
                page += 1;
                html = query(year, page, sa, self.official_publication_url()).await?;
            }
            Ok(numbers)
        })
    }
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
    let params = &[
        ("filter_type", "5"),
        ("filter_organization", "92"),
        ("filter_reg_date_from", &start_date),
        ("filter_reg_date_to", &end_date),
        ("nav-documents", &["page-", &page.to_string()].concat()),
    ];
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
        logger::StructLogger::new_default();
        let q = super::query(2024, 1, "", "https://npa.bashkortostan.ru").await;
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