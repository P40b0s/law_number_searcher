mod publication_api;
mod deserialization;
mod models;
mod requests;
mod error;


use utilites::Url;
pub use publication_api::PublicationApi;
pub use models::{PublicationDocumentCard, PublicationDocumentCardList, PublicationDocumentType, ExtendedPublicationDocumentCard};
pub use error::PublicationApiError;


#[cfg(test)]
mod tests
{

    use logger::StructLogger;
    use utilites::{Date, Url};

    use super::{PublicationDocumentCardList, error::PublicationApiError};

    #[tokio::test]
    async fn test_api()
    {
        StructLogger::new_default();
       
    }

    #[tokio::test]
    async fn test_api_1()
    {
        StructLogger::new_default();
       
    }
}