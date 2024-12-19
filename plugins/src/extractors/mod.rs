use std::{fmt::Display, str::FromStr};
use hashbrown::HashMap;
use prezident::PrezidentPlugin;
use serde::{Deserialize, Serialize};
use crate::ExtractorError;
mod prezident;
pub mod number_extractors;
pub mod types;
pub mod signatory_authorites;



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





pub trait ExtractorPlugin<'a> where Self: Send + Sync
{
    fn semantic_version(&self) -> &'static str;
    ///наименование органа
    fn name(&self) -> &'static str;
    ///guid signatory_authority
    fn signatory_authority(&self) -> &'static str;
    /// guid с типом документа
    fn type_ids(&self) -> &'static[ActType];
    ///url сайта где официально опубликовываются данные докуенты (кроме publication.pravo.gov.ru)
    fn official_publication_url(&self) -> Option<&'static str>;


    fn get_raw_number(&self, act_type: &str,  number: &str) -> Result<u32, crate::error::ExtractorError>;

    fn get_skip_numbers(&'a self, act_type: &str, numbers: &[String]) -> Result<Vec<String>, crate::error::ExtractorError>
    {
        let tp: ActType = act_type.parse()?;
        let mut raw_numbers = Vec::with_capacity(numbers.len());
        let mut max = 0;
        let mut min = u32::MAX;
        for n in numbers
        {
            let raw_number = self.get_raw_number(act_type, n)?;
            max = max.max(raw_number);
            min = min.min(raw_number);
            raw_numbers.push(raw_number);
        }
        //logger::info!("min {} max {} all {:?}", min, max, &numbers);
        raw_numbers.sort();
        let mut skipped = Vec::new();
        for n in min..max
        {
            if let Err(_) = raw_numbers.binary_search(&n)
            {
                skipped.push(self.number_format(&tp, n));
            }
        }
        Ok(skipped)
    }
    fn number_format(&'a self, act_type: &ActType, number: u32) -> String;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActType
{
    Указ,
    Распоряжение,
    ФедеральныйЗакон
}

impl FromStr for ActType
{
    type Err = super::error::ExtractorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> 
    {
        if let Ok(guid) = uuid::Uuid::parse_str(s)
        {
            let s = guid.to_string();
            let s = s.as_str();
            match s
            {
                types::ФЕДЕРАЛЬНЫЙ_ЗАКОН => Ok(ActType::ФедеральныйЗакон),
                types::УКАЗ => Ok(ActType::Указ),
                types::РАСПОРЯЖЕНИЕ => Ok(ActType::Распоряжение),
                _ => Err(crate::ExtractorError::ParseActTypeError(s.to_owned()))
            }
        }
        else 
        {
            return Err(crate::ExtractorError::ParseActTypeError(s.to_owned()));   
        }
    }
}
impl TryInto<ActType> for &str
{
    type Error = super::error::ExtractorError;
    fn try_into(self) -> Result<ActType, Self::Error> 
    {
        self.parse()
    }
}

impl Display for ActType
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self
        {
            ActType::ФедеральныйЗакон => f.write_str(types::ФЕДЕРАЛЬНЫЙ_ЗАКОН),
            ActType::Указ => f.write_str(types::УКАЗ),
            ActType::Распоряжение => f.write_str(types::РАСПОРЯЖЕНИЕ)
        }
    }
}
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



