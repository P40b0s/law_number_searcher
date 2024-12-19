use std::sync::{Arc, LazyLock};
pub use error::SearcherError;
use plugins::ExtractorManager;
use publication_api::PublicationDocumentCard;
pub use publication_api::{SignatoryAuthority, PublicationApiError, DocumentType};
use regex::Regex;
use utilites::Date;
mod error;
static PLUGINS: LazyLock<Arc<ExtractorManager>> = LazyLock::new(|| Arc::new(ExtractorManager::new()));

pub struct Searcher{}
impl Searcher
{
    pub async fn get_signatory_authorites() -> Result<Vec<SignatoryAuthority>, SearcherError>
    {
        let organs = publication_api::PublicationApi::get_signatory_authorites().await?;
        Ok(organs)
    }
    pub async fn get_types(sa: &str) -> Result<Vec<DocumentType>, SearcherError>
    {
        let types = publication_api::PublicationApi::get_documents_types_by_signatory_authority(sa).await?;
        Ok(types)
    }
    pub fn get_exists_parsers<'a>() -> Result<Vec<&'a str>, SearcherError>
    {
        let parsers = PLUGINS.get_exists_parsers()?;
        Ok(parsers)
    }
    pub async fn get_exists_numbers(signatory_authority: &str, doc_type: &str, year: u32) -> Result<Vec<String>, SearcherError>
    {
        let date_from_format = ["01.01.".to_owned(), year.to_string()].concat();
        let date_to_format = ["31.12.".to_owned(), year.to_string()].concat();
        let date_from  = Date::parse(&date_from_format);
        if date_from.is_none()
        {
            return Err(SearcherError::DateFormatError(date_from_format));
        }
        let date_to  = Date::parse(&date_to_format);
        if date_to.is_none()
        {
            return Err(SearcherError::DateFormatError(date_to_format));
        }
        let docs = publication_api::PublicationApi::get_documents_for_period(
            date_from.as_ref().unwrap(),
            date_to.as_ref().unwrap(),
            &[doc_type.to_owned()],
            Some(&signatory_authority.to_owned()),
            None).await?;
        let mut numbers = Vec::with_capacity(docs.len());
        static RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(r"№\s+(.+)\s").unwrap());
        for d in docs
        {
            if let Some(number) = RE.captures(&d.complex_name)
            {
                numbers.push(number[1].to_owned());
            }
            else 
            {
                logger::error!("Ошибка извлечения номера из названия `{}` -> `{}`", d.complex_name, d.eo_number);
                return Err(SearcherError::ParseNumberError(d.complex_name, d.eo_number))
            }
        }
        Ok(numbers)
    }

    
}



#[cfg(test)]
mod tests
{
    use regex::Regex;
    use serde_json::error;

    #[tokio::test]
    async fn test_get_organs()
    {
        logger::StructLogger::new_default();
        let organs = super::Searcher::get_signatory_authorites().await.unwrap();
        logger::debug!("{:?}", organs);
    } 
    #[tokio::test]
    async fn test_get_all_numbers()
    {
        logger::StructLogger::new_default();
        let organs = super::Searcher::get_exists_numbers("225698f1-cfbc-4e42-9caa-32f9f7403211", "82a8bf1c-3bc7-47ed-827f-7affd43a7f27", 2024).await.unwrap();
        logger::debug!("{:?}", organs);
    } 
    #[tokio::test]
    /// получать карточку каждого документа это очень долго, есть вариант взять номер из полного наименования
    async fn test_split()
    {
        logger::StructLogger::new_default();
        let text = r#"Федеральный закон от 13.12.2024 № 475-ФЗ\n \"О внесении изменений в отдельные законодательные акты Российской Федерации\""#;
        static RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(r"№\s+(.+)\s").unwrap());
        if let Some(caps) = RE.captures(text) 
        {
            logger::debug!("{:?}", &caps[1]);
        }
        else 
        { 
            logger::error!("ERROR!");
        };

        //let Some(caps) = RE.captures(text) else { return };
       
    } 

    // 'Федеральный закон от 22.06.2024 № 160-ФЗ\n "О внесении изменений в статью 19 Федерального закона "О крестьянском (фермерском) хозяйстве" и Федеральный закон "О развитии сельского хозяйства"'
}