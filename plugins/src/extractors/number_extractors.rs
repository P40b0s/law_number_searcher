use crate::{create_error, ExtractorError};

/// извлечение номера с разделителем-тире например 123-ФЗ 512-рп итд
pub fn get_number_with_dash_delim(number: &str) -> Result<u32, crate::error::ExtractorError>
{
    if let Some((n, _)) = number.split_once("-")
    {
        let number: u32 = n.parse()
        .map_err(|e| ExtractorError::ParseNumberError { number: number.to_owned(), source: e })?;
        return Ok(number);
    }
    else 
    {
        return create_error!(ExtractorError::NumberFormatError(number.to_owned()));   
    }
}
///получение чистого номера - 123, 512
pub fn get_clean_number(number: &str) -> Result<u32, crate::error::ExtractorError>
{
    let number: Result<u32, ExtractorError> = number.parse()
        .map_err(|e| ExtractorError::ParseNumberError { number: number.to_owned(), source: e });
    if number.is_err()
    {
        let err = number.err().unwrap();
        return create_error!(err);   
    }
    return Ok(number.unwrap());
}

