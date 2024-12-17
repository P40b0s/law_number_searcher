use std::sync::{Arc, LazyLock};
pub use error::SearcherError;
use plugins::ExtractorManager;
pub use publication_api::{SignatoryAuthority, PublicationApiError};
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