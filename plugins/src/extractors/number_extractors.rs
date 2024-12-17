use crate::ExtractorError;

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
        return  Err(ExtractorError::NumberFormatError(number.to_owned()));    
    }
}
///получение чистого номера - 123, 512
pub fn get_clean_number(number: &str) -> Result<u32, crate::error::ExtractorError>
{
    let number: u32 = number.parse()
        .map_err(|e| ExtractorError::ParseNumberError { number: number.to_owned(), source: e })?;
    return Ok(number);
}