use std::sync::{Arc, LazyLock};
pub use error::SearcherError;
use plugins::{ExtractorManager, ExtractorPlugin};
use publication_api::PublicationDocumentCard;
pub use publication_api::{SignatoryAuthority, PublicationApiError, DocumentType};
use regex::Regex;
use serde::{Deserialize, Serialize};
use utilites::Date;
mod error;
static PLUGINS: LazyLock<Arc<ExtractorManager>> = LazyLock::new(|| Arc::new(ExtractorManager::new()));
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Dictionary
{
    pub id: String,
    pub name: String,
    pub having_parser: bool
}
impl Into<Dictionary> for SignatoryAuthority
{
    fn into(self) -> Dictionary 
    {
        Dictionary 
        {
            id: self.id,
            name: self.name,
            having_parser: false
        }
    }
}
impl Into<Dictionary> for DocumentType
{
    fn into(self) -> Dictionary 
    {
        Dictionary 
        {
            id: self.id,
            name: self.name,
            having_parser: false
        }
    }
}
pub struct Searcher{}
///так как DocumentType и signatory authority одинаковые и идут с весами которые нам ненужны, но отсуствует поле что для них есть парсер которое нам как раз нужно, сделаем новую структуру и будет все конвертить в нее
impl Searcher
{
    pub async fn get_signatory_authorites() -> Result<Vec<Dictionary>, SearcherError>
    {
        let organs = publication_api::PublicationApi::get_signatory_authorites().await?;
        let organs: Vec<Dictionary> = organs.into_iter().map(|o| 
            {
                let mut d: Dictionary = o.into();
                d.having_parser = Self::parser_exists(&d.id);
                d
            }
        ).collect();
        Ok(organs)
    }
    pub async fn get_types(sa: &str) -> Result<Vec<Dictionary>, SearcherError>
    {
        let types = publication_api::PublicationApi::get_documents_types_by_signatory_authority(sa).await?;
        let types_with_parsers = Self::get_types_in_parser(sa).await?;
        let types: Vec<Dictionary> = types.into_iter().map(|t|
        {
            let mut d: Dictionary = t.into();
            d.having_parser = types_with_parsers.contains(&d.id);
            d
        }).collect();
        Ok(types)
    }
    pub async fn get_types_in_parser(sa: &str) -> Result<Vec<String>, SearcherError>
    {
        let plugin = PLUGINS.get_plugin(sa)?;
        let ids: Vec<String> = plugin.type_ids().into_iter().map(|t| t.to_string()).collect();
        Ok(ids)
    }
    fn parser_exists(sa: &str) -> bool
    {
        PLUGINS.get_plugin(sa).is_ok()
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


    /// Retrieves a list of "lost" document numbers for the given signatory authority, document type, and year.
    ///
    /// This function first retrieves the list of existing document numbers using the `get_exists_numbers` function.
    /// It then uses a plugin associated with the given signatory authority to determine which of those numbers should be
    /// considered "lost" or skipped. The resulting list of skipped/lost numbers is returned.
    ///
    /// # Arguments
    /// * `signatory_authority` - The signatory authority for which to retrieve lost document numbers.
    /// * `doc_type` - The document type for which to retrieve lost document numbers.
    /// * `year` - The year for which to retrieve lost document numbers.
    ///
    /// # Returns
    /// A `Result` containing a `Vec<String>` of the lost document numbers, or a `SearcherError` if an error occurs.
    pub async fn get_lost_numbers(signatory_authority: &str, doc_type: &str, year: u32) -> Result<Vec<String>, SearcherError>
    {
        let numbers = Self::get_exists_numbers(signatory_authority, doc_type, year).await?;
        //logger::debug!("numbers {:?}", &numbers);
        let plugin = PLUGINS.get_plugin(signatory_authority)?;
        let skipped = plugin.get_skip_numbers(doc_type, &numbers)?;
        Ok(skipped)
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
        let organs = super::Searcher::get_exists_numbers(plugins::signatory_authorites::ПРЕЗИДЕНТ_РОССИЙСКОЙ_ФЕДЕРАЦИИ, plugins::types::УКАЗ, 2024).await.unwrap();
        logger::debug!("{:?}", organs);
    } 
    #[tokio::test]
    async fn test_get_skipped_numbers()
    {
        logger::StructLogger::new_default();
        let organs = super::Searcher::get_lost_numbers(plugins::signatory_authorites::ПРЕЗИДЕНТ_РОССИЙСКОЙ_ФЕДЕРАЦИИ, plugins::types::УКАЗ, 2024).await.unwrap();
        logger::debug!("{:?}", organs);
    } 
    #[tokio::test]
    async fn test_get_skipped_numbers_fz()
    {
        logger::StructLogger::new_default();
        let organs = super::Searcher::get_lost_numbers(plugins::signatory_authorites::ПРЕЗИДЕНТ_РОССИЙСКОЙ_ФЕДЕРАЦИИ, plugins::types::ФЕДЕРАЛЬНЫЙ_ЗАКОН, 2024).await.unwrap();
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