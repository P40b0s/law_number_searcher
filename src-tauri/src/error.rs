use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error 
{
    #[error(transparent)]
    DeserializeError(#[from] serde_json::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error("Ошибка, введен неверный ключ регистрации `{0}`")]
    WrongRegisterKeyError(String),
    #[error(transparent)]
    SearcherError(#[from] searcher::SearcherError),
    #[error("Ошибка, при поиске номеров на альтернативном сайте опубликования `{0}`")]
    AlternativePublSiteError(String),
    #[error("На альтернативном сайте опубликования не найдено ни одного документа")]
    AlternativePublSiteNoData,
    #[error("Ошибка экспорта списка номеров в формат excel: `{0}`")]

    ExportError(String)

}

impl serde::Serialize for Error 
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
  S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}