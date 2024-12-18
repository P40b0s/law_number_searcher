use std::sync::{Arc, LazyLock};
pub use error::SearcherError;
use plugins::ExtractorManager;
pub use publication_api::{SignatoryAuthority, PublicationApiError, DocumentType};
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