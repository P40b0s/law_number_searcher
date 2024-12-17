use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtractorError 
{
    #[error("Ошибка ввода-вывода `{}`", source)]
    Io 
    {
        #[from]
        source: std::io::Error,
    },
    #[error("Ошибка преобразования номера документа в число: `{}`", source)]
    ParseNumberError {number: String, #[source] source: ParseIntError},
    #[error("Ошибка формат номера `{}` не поддерживается", .0)]
    NumberFormatError(String),
    #[error("Ошибка идентификации типа документа: `{}`", .0)]
    ParseActTypeError(String),
    #[error("В текущем парсере остуствует тип документа: `{}`", .0)]
    ActTypeNotSupported(String),
    #[error("Для id подписанта: `{}` плагины не найдены", .0)]
    PluginNotFound(String)
}