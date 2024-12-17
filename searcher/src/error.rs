use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SearcherError 
{
    #[error(transparent)]
    ExtractorError(#[from] plugins::ExtractorError),
    #[error(transparent)]
    PublicationApiError(#[from] publication_api::PublicationApiError)
}

impl serde::Serialize for SearcherError
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
  S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}