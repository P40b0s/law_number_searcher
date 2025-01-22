use std::sync::{Arc, LazyLock};
use std::u32;
use futures::future::BoxFuture;
use regex::Regex;
use scraper::Selector;
use utilites::http::{HyperClient, Uri, ACCEPT, ACCEPT_ENCODING, HOST, USER_AGENT};
use crate::create_parser;
use super::{signatory_authorites, types, OffSiteParser};



create_parser!(BuryatOffSiteParser,
    "https://egov-buryatia.ru",
    "https://egov-buryatia.ru/npa_template",[],
     parse);

async fn parse(_regexes: Arc<LazyLock<Vec<Regex>>>, api_url: &str, sa: &str, act_type: &str, year: u32, sender: Option<tokio::sync::mpsc::Sender<String>>) 
-> Result<Vec<String>, crate::error::ExtractorError>
{
    let mut page = 1;
    let mut html = query(year, page, sa, act_type, api_url).await?;
    let mut numbers = Vec::new();
    //при достижении последней страницы, все последующие страницы будут отображать первыую страницу, так что как только номер совпадет выходим из while
    'wl: while let Some(items) = scrap(&html)
    {
        for name in items
        {
            if !name.is_empty()
            {
                if numbers.contains(&name)
                {
                    break 'wl;
                }
                else 
                {
                    numbers.push(name);
                }
            }
        }
        page += 1;
        if let Some(s) = sender.as_ref()
        {
            let _ = s.send(["Получение данных с ".to_owned(), api_url.to_owned(), " стр. № ".to_owned(), page.to_string()].concat()).await;
        }
        html = query(year, page, sa, act_type, api_url).await?;
    }
    Ok(numbers)
}

fn client(uri: &str) -> HyperClient
{
    let uri: Uri = uri.parse().unwrap();
    let client = utilites::http::HyperClient::new(uri).with_headers(vec![
        (ACCEPT_ENCODING, "gzip, deflate, br, zstd"),
        (ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
        (HOST, "egov-buryatia.ru"),
        (USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:134.0) Gecko/20100101 Firefox/134.0")
    ]);
    client
}

fn scrap(html: &String) -> Option<Vec<String>>
{
    let parsed = scraper::Html::parse_document(html);
    let doc_item_selector = Selector::parse("tr.npa-tr td.npa-td:nth-child(4)").unwrap();
    let frag = parsed.select(&doc_item_selector);
    let items = frag.map(|f| f.inner_html().replace("\t", "").replace("\n", "")).collect::<Vec<String>>();
    if items.len() > 0
    {
        Some(items)
    }
    else 
    {
        None    
    }
}
async fn query(year: u32, page: u32, sa: &str, act_type: &str, uri: &str) -> Result<String, super::super::error::ExtractorError>
{
    //https://npa.bashkortostan.ru/?filter_name=&filter_type=5&filter_organization=92&filter_reg_date_from=01.01.2024&filter_reg_date_to=31.12.2024&filter_pub_date_from=&filter_pub_date_to=&filter_reg_number=%D0%A3%D0%93-12
    let client = client(uri);
    let start_date = [year.to_string(), "-01-01".to_owned()].concat();
    let end_date = [year.to_string(), "-12-31".to_owned()].concat();
    let doc_type = match act_type
    {
        types::УКАЗ => "Указ",
        types::РАСПОРЯЖЕНИЕ => "Распоряжение",
        types::ЗАКОН => "Закон",
        _ => ""

    };
    let signatory_authority = match sa
    {
        signatory_authorites::РЕСПУБЛИКА_БУРЯТИЯ => "Народный Хурал",
        signatory_authorites::ГЛАВА_РЕСПУБЛИКИ_БУРЯТИЯ => "Глава Республики Бурятия",
        _ => ""
    };
    let params =  &[
        ("date_doc_from", start_date.as_str()),
        ("date_doc_to", end_date.as_str()),
        ("TIP_DOC", doc_type),
        ("ORGAN_VLASTI", signatory_authority),
        ("PAGEN_1", &page.to_string()),
    ];
    
    let req = client.get_with_params(params).await?;
    let html = String::from_utf8(req.1.to_vec())?;
    Ok(html)
}

#[cfg(test)]
mod tests
{
    use scraper::Selector;
    use crate::{extractors::OffSiteParser, signatory_authorites, types};

    #[test]
    fn test_scrapper()
    {
        let _ = logger::StructLogger::new_custom(logger::LevelFilter::Debug, Some(&[("html5ever", logger::LevelFilter::Info), ("selectors::matching", logger::LevelFilter::Info)]));
        let bytes = include_bytes!("./buryat_site.html");
        let html = String::from_utf8_lossy(bytes);
        let parsed = scraper::Html::parse_document(html.as_ref());
        let doc_item = Selector::parse("tr.npa-tr td.npa-td:nth-child(4)").unwrap();
        let frag = parsed.select(&doc_item);
        for href in frag
        {
            logger::debug!("документ: {:?}",  href.inner_html().replace("\t", "").replace("\n", ""));
        }
    }

    #[tokio::test]
    async fn test_parser()
    {
        let _ = logger::StructLogger::new_custom(logger::LevelFilter::Debug, Some(&[("html5ever", logger::LevelFilter::Info), ("selectors::matching", logger::LevelFilter::Info)]));
        let br = super::BuryatOffSiteParser::new();
        let res = br.check_numbers_on_alternative_site(signatory_authorites::РЕСПУБЛИКА_БУРЯТИЯ, types::ЗАКОН, 2024, None).await.unwrap();
        for href in res
        {
            logger::debug!("документ: {}",  href);
        }
    }
}

//https://egov-buryatia.ru/npa_template/?date_doc_from=2024-01-01&date_doc_to=2024-12-31&date_public_from=&date_public_to=&TIP_DOC=%D0%A3%D0%BA%D0%B0%D0%B7&ORGAN_VLASTI=%D0%93%D0%BB%D0%B0%D0%B2%D0%B0+%D0%A0%D0%B5%D1%81%D0%BF%D1%83%D0%B1%D0%BB%D0%B8%D0%BA%D0%B8+%D0%91%D1%83%D1%80%D1%8F%D1%82%D0%B8%D1%8F