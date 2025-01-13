use std::cell::OnceCell;
use std::u32;
use logger::info;
use regex::Regex;
use crate::ExtractorError;
use super::plugin_trait::Number;
use super::{number_extractors, signatory_authorites};
use super::ExtractorPlugin;

///Если данного вида акта не будет в перечне плагинов, то будет кидать на этот плагин
#[derive(Debug)]
pub struct DefaultPlugin where Self: Send + Sync {}
static CLEAR_NUMBER_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(r"^\d{1,4}").unwrap());
impl<'a> ExtractorPlugin<'a> for DefaultPlugin
{
   // &DocumentTypes=&DocumentTypes=&PublishDateSearchType=0&NumberSearchType=0&DocumentDateSearchType=0&JdRegSearchType=0&SortedBy=6&SortDestination=1
    fn signatory_authority(&self) -> &'static str
    {
        "default"
    }

    fn official_publication_url(&self) -> Option<&'static str> 
    {
        None
    }

    fn get_raw_number<'b>(&'a self, act_type: &str,  number: &'b str) -> Result<Number, crate::error::ExtractorError>
    {
        //указы распоряжения итд со всякими постфиксами точно пападут под этот регекс, поэтому обработать нужно будет только крайние случаи
        if let Some(mch) = CLEAR_NUMBER_RE.find(number)
        {
            let index = mch.end();
            let n = number.split_at(index);
            return Ok(Number {
                number: n.0.parse().unwrap(),
                postfix: Some(n.1.to_owned()),
                prefix: None
            });
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

    // fn get_skip_numbers<'b: 'a>(&'a self,  act_type: &str, numbers: Vec<String>) -> Result<Vec<String>, crate::error::ExtractorError>
    // {
    //     if numbers.len() == 0
    //     {
    //         return Ok(Vec::<String>::new());
    //     }
    //     let mut raw_numbers = Vec::with_capacity(numbers.len());
    //     let mut max = 0;
    //     let mut min = u32::MAX;
    //     let postfix: OnceCell<String> = OnceCell::new();
    //     for n in numbers
    //     {
    //         let raw_number = self.get_raw_number(act_type, &n)?;
    //         let _ = postfix.set(raw_number.postfix.unwrap_or_default().to_owned());
    //         max = max.max(raw_number.number);
    //         min = min.min(raw_number.number);
    //         raw_numbers.push(raw_number.number);
    //     }
    //     raw_numbers.sort();
    //     let mut skipped = Vec::new();
    //     for n in min..max
    //     {
    //         //если номер не найден добавляем его в список пропущеных
    //         if let Err(_) = raw_numbers.binary_search(&n)
    //         {
    //             let formatted = [n.to_string(), postfix.get().unwrap().to_owned()].concat();
    //             skipped.push(formatted);
    //         }
    //     }
    //     Ok(skipped)
    // }
    

    // fn number_format(&'a self, act_type: &str, number: &[u32]) -> String 
    // {
    //     match act_type
    //     {
    //         ActType::ФедеральныйЗакон => [number.to_string(), "-ФЗ".to_owned()].concat(),
    //         ActType::Указ => number.to_string(),
    //         ActType::Распоряжение => [number.to_string(), "-рп".to_owned()].concat(),
    //     }
    // }


}