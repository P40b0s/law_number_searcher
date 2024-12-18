use std::sync::{Arc, LazyLock};
pub use error::SearcherError;
use plugins::ExtractorManager;
use publication_api::PublicationDocumentCard;
pub use publication_api::{SignatoryAuthority, PublicationApiError, DocumentType};
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
        let date_to  = Date::parse(&date_to_format);
        if date_to.is_none()
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
        for d in docs
        {
            let extended = publication_api::PublicationApi::get_document_extended_card(&d.eo_number).await?;
            numbers.push(extended.number);
        }
        Ok(numbers)
    }
}



#[cfg(test)]
mod tests
{
    #[tokio::test]
    async fn test_get_organs()
    {
        logger::StructLogger::new_default();
        let organs = super::Searcher::get_signatory_authorites().await.unwrap();
        logger::debug!("{:?}", organs);
    } 
}