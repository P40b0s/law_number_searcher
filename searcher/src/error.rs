use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SearcherError 
{
    #[error(transparent)]
    ExtractorError(#[from] plugins::ExtractorError),
    #[error(transparent)]
    PublicationApiError(#[from] publication_api::PublicationApiError),
    #[error("Ошибка, дата `{0}` имеет неверный формат")]
    DateFormatError(String),
    #[error("Ошибка извлечения номера из наименования`{0}` в документе EO `{1}`")]
    /// 1 - наименование из корого производится извлечение
    /// 2 - номер опубликования
    ParseNumberError(String, String)
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