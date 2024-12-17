use std::{fmt::Display, str::FromStr};
use prezident::PrezidentPlugin;
use serde::{Deserialize, Serialize};
use crate::ExtractorError;
mod prezident;
pub mod number_extractors;



pub struct ExtractorManager<'a>
{
    extractors: Vec<Box<dyn ExtractorPlugin<'a>>>
}
impl<'a> ExtractorManager<'a>
{
    pub fn new() -> Self
    {
        Self
        {
            extractors: vec![
                Box::new(PrezidentPlugin{})
            ]
        }
    }
    pub fn get_plugin(&self, signatory_authority: &str) -> Result<&Box<dyn ExtractorPlugin<'a>>, ExtractorError>
    {
        if let Some(plugin) = self.extractors.iter().find(|f| f.signatory_authority() == signatory_authority)
        {
            Ok(plugin)
        }
        else 
        {
            Err(ExtractorError::PluginNotFound(signatory_authority.to_owned()))
        }
    }
}





pub trait ExtractorPlugin<'a>
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

    fn get_skip_numbers(&'a self, act_type: &str, numbers: &[&str]) -> Result<Vec<String>, crate::error::ExtractorError>
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
        logger::info!("min {} max {} all {:?}", min, max, &numbers);
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
                "7ff5b3b5-3757-44f1-bb76-3766cabe3593" => Ok(ActType::ФедеральныйЗакон),
                "82a8bf1c-3bc7-47ed-827f-7affd43a7f27" => Ok(ActType::Указ),
                "0790e34b-784b-4372-884e-3282622a24bd" => Ok(ActType::Распоряжение),
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
            ActType::ФедеральныйЗакон => f.write_str("7ff5b3b5-3757-44f1-bb76-3766cabe3593"),
            ActType::Указ => f.write_str("82a8bf1c-3bc7-47ed-827f-7affd43a7f27"),
            ActType::Распоряжение => f.write_str("0790e34b-784b-4372-884e-3282622a24bd")
        }
    }
}




