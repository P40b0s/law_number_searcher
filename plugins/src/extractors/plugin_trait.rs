use std::{cell::OnceCell, fmt::Display, str::FromStr};
use hashbrown::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::ExtractorError;



static CLEAR_NUMBER_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(r"\d{1,4}").unwrap());
pub trait ExtractorPlugin<'a> where Self: Send + Sync
{
    fn signatory_authority(&self) -> &'static str;
    ///url сайта где официально опубликовываются данные докуенты (кроме publication.pravo.gov.ru)
    fn official_publication_url(&self) -> Option<&'static str>;

    fn get_raw_number<'b>(&'a self, act_type: &str,  number: &'b str) -> Result<(u32, &'b str), crate::error::ExtractorError>
    {
        //указы распоряжения итд со всякими постфиксами точно пападут под этот регекс, поэтому обработать нужно будет только крайние случаи
        if let Some(mch) = CLEAR_NUMBER_RE.find(number)
        {
            let index = mch.end();
            let n = number.split_at(index);
            return Ok((n.0.parse().unwrap(), n.1));
        }
        else
        {
            match act_type
            {
                _ => Err(crate::error::ExtractorError::NumberFormatError(number.to_owned()))
            }
        }
        
    }
    fn number_is_support(&'a self, number: &str) -> bool
    {
        CLEAR_NUMBER_RE.find(number).is_some()
    }

    fn get_skip_numbers<'b: 'a>(&'a self,  act_type: &str, numbers: Vec<String>) -> Result<Vec<String>, crate::error::ExtractorError>
    {
        if numbers.len() == 0
        {
            return Ok(Vec::<String>::new());
        }
        let mut raw_numbers = Vec::with_capacity(numbers.len());
        let mut max = 0;
        let mut min = u32::MAX;
        let postfix: OnceCell<String> = OnceCell::new();
        for n in numbers
        {
            let raw_number = self.get_raw_number(act_type, &n)?;
            let _ = postfix.set(raw_number.1.to_owned());
            max = max.max(raw_number.0);
            min = min.min(raw_number.0);
            raw_numbers.push(raw_number.0);
        }
        raw_numbers.sort();
        let mut skipped = Vec::new();
        for n in min..max
        {
            //если номер не найден добавляем его в список пропущеных
            if let Err(_) = raw_numbers.binary_search(&n)
            {
                let formatted = [n.to_string(), postfix.get().unwrap().to_owned()].concat();
                skipped.push(formatted);
            }
        }
        Ok(skipped)
    }
}