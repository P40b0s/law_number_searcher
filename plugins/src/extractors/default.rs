use std::u32;
use logger::info;
use crate::ExtractorError;
use super::{number_extractors, signatory_authorites};
use super::ExtractorPlugin;

///Если данного вида акта не будет в перечне плагинов, то будет кидать на этот плагин
#[derive(Debug)]
pub struct DefaultPlugin where Self: Send + Sync {}

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