use std::u32;
use logger::info;
use crate::ExtractorError;
use super::number_extractors;
use super::{ActType, ExtractorPlugin};

#[derive(Debug)]
pub struct PrezidentPlugin where Self: Send + Sync {}

impl<'a> ExtractorPlugin<'a> for PrezidentPlugin
{
    fn semantic_version(&self) -> &'static str
    {
       "0.1.0"
    }
    fn name(&self) -> &'static str
    {
        "Президент Российской Федерации"
    }
   // &DocumentTypes=&DocumentTypes=&PublishDateSearchType=0&NumberSearchType=0&DocumentDateSearchType=0&JdRegSearchType=0&SortedBy=6&SortDestination=1
    fn signatory_authority(&self) -> &'static str
    {
        "225698f1-cfbc-4e42-9caa-32f9f7403211"
    }

    fn type_ids(&self) -> &'static[ActType] 
    {
        &[
            //указ
            ActType::Указ,
            //распоряжение
            ActType::ФедеральныйЗакон,
            //федеральный закон
            ActType::Распоряжение
        ]
    }

    fn official_publication_url(&self) -> Option<&'static str> 
    {
        Some("http://publication.pravo.gov.ru")
    }

    fn get_raw_number(&self, act_type: &str,  number: &str) -> Result<u32, ExtractorError>
    {
        let tp: ActType = act_type.parse()?;
        if self.type_ids().contains(&tp)
        {
            match tp
            {
                ActType::Указ => number_extractors::get_clean_number(number),
                ActType::ФедеральныйЗакон => number_extractors::get_number_with_dash_delim(number),
                ActType::Распоряжение => number_extractors::get_number_with_dash_delim(number)
            }
        }
        else 
        {
            return Err(crate::ExtractorError::ActTypeNotSupported(act_type.to_owned()));    
        }
    }
    fn number_format(&'a self, act_type: &ActType, number: u32) -> String 
    {
        match act_type
        {
            ActType::ФедеральныйЗакон => [number.to_string(), "-ФЗ".to_owned()].concat(),
            ActType::Указ => number.to_string(),
            ActType::Распоряжение => [number.to_string(), "-рп".to_owned()].concat(),
        }
    }
}


#[cfg(test)]
mod tests
{
    use crate::extractors::{ActType, ExtractorPlugin};

    #[test]
    fn test()
    {
        let plugin = super::PrezidentPlugin {  };
        logger::StructLogger::new_default();
        logger::debug!("{}", plugin.name());
    }

    #[test]
    fn test_raw_numbers()
    {
        let plugin = super::PrezidentPlugin {  };
        logger::StructLogger::new_default();
        let number1 = plugin.get_raw_number("82a8bf1c-3bc7-47ed-827f-7affd43a7f27", "32123").unwrap();
        assert_eq!(number1, 32123);
        let number2 = plugin.get_raw_number("7ff5b3b5-3757-44f1-bb76-3766cabe3593", "333-ФЗ").unwrap();
        assert_eq!(number2, 333);
        let number3 = plugin.get_raw_number("0790e34b-784b-4372-884e-3282622a24bd", "444-рп").unwrap();
        assert_eq!(number3, 444);
    }

    #[test]
    fn test_skipped_numbers()
    {
        let plugin = super::PrezidentPlugin {  };
        logger::StructLogger::new_default();
        let numbers = vec![
            "166-ФЗ",
            "167-ФЗ",
            "169-ФЗ",
            "170-ФЗ",
            "180-ФЗ"
        ];
        let skipped = plugin.get_skip_numbers("7ff5b3b5-3757-44f1-bb76-3766cabe3593", &numbers).unwrap();
        logger::info!("{:?}", skipped);
        assert_eq!(skipped, vec!["168-ФЗ", "171-ФЗ", "172-ФЗ", "173-ФЗ", "174-ФЗ", "175-ФЗ", "176-ФЗ", "177-ФЗ", "178-ФЗ", "179-ФЗ"]);
    }
}