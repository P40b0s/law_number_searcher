use std::{cell::OnceCell, fmt::Display, str::FromStr};
use hashbrown::HashMap;
use prezident::PrezidentPlugin;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::ExtractorError;
mod prezident;
pub mod number_extractors;
pub mod types;
pub mod signatory_authorites;
mod universal;



pub struct ExtractorManager<'a> where Self: Send + Sync 
{
    extractors: hashbrown::HashMap<String, Box<dyn ExtractorPlugin<'a>>>
}
impl<'a> ExtractorManager<'a>
{
    pub fn new() -> Self
    {
        let mut hm = HashMap::new();
        let plugin: Box<dyn ExtractorPlugin> = Box::new(PrezidentPlugin{});
        hm.insert(plugin.signatory_authority().to_owned(), plugin);
        Self
        {
            extractors: hm
        }
    }
    pub fn get_plugin(&self, signatory_authority: &str) -> Result<&Box<dyn ExtractorPlugin<'a>>, ExtractorError>
    {
        if let Some(plugin) = self.extractors.get(signatory_authority)
        {
            Ok(plugin)
        }
        else 
        {
            Err(ExtractorError::PluginNotFound(signatory_authority.to_owned()))
        }
    }
}


static CLEAR_NUMBER_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(r"\d{1,4}").unwrap());
pub trait ExtractorPlugin<'a> where Self: Send + Sync
{
    fn signatory_authority(&self) -> Option<&'static str>;
    ///url сайта где официально опубликовываются данные докуенты (кроме publication.pravo.gov.ru)
    fn official_publication_url(&self) -> Option<&'static str>;

    fn get_raw_number(&'a self, act_type: &str,  number: &'a str) -> Result<(u32, &'a str), crate::error::ExtractorError>
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
                // types::УКАЗ => number_extractors::get_clean_number(number),
                // types::ФЕДЕРАЛЬНЫЙ_ЗАКОН => number_extractors::get_number_with_dash_delim(number),
                // types::ФЕДЕРАЛЬНЫЙ_КОНСТИТУЦИОННЫЙ_ЗАКОН => number_extractors::get_number_with_dash_delim(number),
                //здесь рассмотрим пограничные случаи
                _ => Err(crate::error::ExtractorError::NumberFormatError(number.to_owned()))
            }
        }
        
    }

    fn get_first_number(&'a self, signatory_authority: &str, act_type: &str, numbers: &'a [String]) -> Result<String, crate::error::ExtractorError>
    {
        if numbers.len() == 0
        {
            return Ok(Vec::new());
        }
        let mut raw_numbers = Vec::with_capacity(numbers.len());
        let mut max = 0;
        let mut min = u32::MAX;
        let postfix: OnceCell<String> = OnceCell::new();
        for n in numbers
        {
            let raw_number = self.get_raw_number(act_type, n)?;
            postfix.set(raw_number.1.to_owned());
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

    fn get_skip_numbers(&'a self, signatory_authority: &str, act_type: &str, numbers: &'a [String]) -> Result<Vec<String>, crate::error::ExtractorError>
    {
        if numbers.len() == 0
        {
            return Ok(Vec::new());
        }
        let mut raw_numbers = Vec::with_capacity(numbers.len());
        let mut max = 0;
        let mut min = u32::MAX;
        let postfix: OnceCell<String> = OnceCell::new();
        for n in numbers
        {
            let raw_number = self.get_raw_number(act_type, n)?;
            postfix.set(raw_number.1.to_owned());
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
    fn number_format(&'a self, act_type: &str, number: u32) -> String;
}

// #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
// pub enum ActType
// {
//     Указ,
//     Распоряжение,
//     ФедеральныйЗакон
// }

// impl FromStr for ActType
// {
//     type Err = super::error::ExtractorError;
//     fn from_str(s: &str) -> Result<Self, Self::Err> 
//     {
//         if let Ok(guid) = uuid::Uuid::parse_str(s)
//         {
//             let s = guid.to_string();
//             let s = s.as_str();
//             match s
//             {
//                 types::ФЕДЕРАЛЬНЫЙ_ЗАКОН => Ok(ActType::ФедеральныйЗакон),
//                 types::УКАЗ => Ok(ActType::Указ),
//                 types::РАСПОРЯЖЕНИЕ => Ok(ActType::Распоряжение),
//                 _ => Err(crate::ExtractorError::ParseActTypeError(s.to_owned()))
//             }
//         }
//         else 
//         {
//             return Err(crate::ExtractorError::ParseActTypeError(s.to_owned()));   
//         }
//     }
// }
// impl TryInto<ActType> for &str
// {
//     type Error = super::error::ExtractorError;
//     fn try_into(self) -> Result<ActType, Self::Error> 
//     {
//         self.parse()
//     }
// }

// impl Display for ActType
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
//     {
//         match self
//         {
//             ActType::ФедеральныйЗакон => f.write_str(types::ФЕДЕРАЛЬНЫЙ_ЗАКОН),
//             ActType::Указ => f.write_str(types::УКАЗ),
//             ActType::Распоряжение => f.write_str(types::РАСПОРЯЖЕНИЕ)
//         }
//     }
// }
#[macro_export]
macro_rules! create_error {
    ($e:expr) => {{
        logger::error!("{}", $e);
        Err($e)
    }};
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_act_type_from_str()
    {
        logger::StructLogger::new_default();
        let e = crate::error::ExtractorError::ParseActTypeError("123321".to_owned());
        //let e1 = create_error!(e);
    }
}



