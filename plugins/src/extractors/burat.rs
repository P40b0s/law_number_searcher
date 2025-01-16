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
    signatory_authorites::ГЛАВА_РЕСПУБЛИКИ_БУРЯТИЯ,
    r"(?<number>\d{1,4})(?<postfix>-р)",
    r"(?<number>\d{1,4})");

create_plugin!(RegionPlugin,
    signatory_authorites::РЕСПУБЛИКА_БУРЯТИЯ,
    r"(?<number>\d{1,4})(?<postfix>-[VIX]+)");


pub struct BuryatOffSiteParser{}
impl OffSiteParser for BuryatOffSiteParser
{
    fn official_publication_url(&self) -> &'static str
    {
        "https://egov-buryatia.ru/npa_template"
    }
    fn check_numbers_on_alternative_site<'a>(&'a self, sa: &'a str, act_type: &'a str, year: u32) 
    -> BoxFuture<'a, Result<Vec<String>, crate::error::ExtractorError>>
    {
        Box::pin(async move 
        {
            let regexes: Option<std::sync::LazyLock<Vec<regex::Regex>>> = match sa
            {
                signatory_authorites::ГЛАВА_РЕСПУБЛИКИ_БУРЯТИЯ => Some(HeadPlugin::get_regexes()),
                signatory_authorites::РЕСПУБЛИКА_БУРЯТИЯ => Some(RegionPlugin::get_regexes()),
                _ => None
            };
            if regexes.is_none()
            {
                return Err(ExtractorError::ActTypeNotSupported(["Не найден регекс к текущему органу: ", sa].concat()));
            }
            let regexes = regexes.unwrap();
            let mut page = 1;
            let mut html = query(year, page, sa, act_type, self.official_publication_url()).await?;
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
                html = query(year, page, sa, act_type, self.official_publication_url()).await?;
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
    //date_doc_from=2024-01-01&date_doc_to=2024-12-31&date_public_from=&date_public_to=&TIP_DOC=%D0%A3%D0%BA%D0%B0%D0%B7&ORGAN_VLASTI=%D0%93%D0%BB%D0%B0%D0%B2%D0%B0+%D0%A0%D0%B5%D1%81%D0%BF%D1%83%D0%B1%D0%BB%D0%B8%D0%BA%D0%B8+%D0%91%D1%83%D1%80%D1%8F%D1%82%D0%B8%D1%8F
    //nav-documents=page-2
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
        //сцуко с + работает но + перобразуется в симолы url и уже не работает
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
        let br = super::BuryatOffSiteParser{};
        let res = br.check_numbers_on_alternative_site(signatory_authorites::РЕСПУБЛИКА_БУРЯТИЯ, types::ЗАКОН, 2025).await.unwrap();
        for href in res
        {
            logger::debug!("документ: {}",  href);
        }
    }
}

//https://egov-buryatia.ru/npa_template/?date_doc_from=2024-01-01&date_doc_to=2024-12-31&date_public_from=&date_public_to=&TIP_DOC=%D0%A3%D0%BA%D0%B0%D0%B7&ORGAN_VLASTI=%D0%93%D0%BB%D0%B0%D0%B2%D0%B0+%D0%A0%D0%B5%D1%81%D0%BF%D1%83%D0%B1%D0%BB%D0%B8%D0%BA%D0%B8+%D0%91%D1%83%D1%80%D1%8F%D1%82%D0%B8%D1%8F