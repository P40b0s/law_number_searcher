use std::u32;
use logger::info;
use crate::ExtractorError;
use super::{number_extractors, signatory_authorites};
use super::{ActType, ExtractorPlugin};

#[derive(Debug)]
pub struct PrezidentPlugin where Self: Send + Sync {}

impl<'a> ExtractorPlugin<'a> for PrezidentPlugin
{
    fn semantic_version(&self) -> &'static str
    {
       "0.1.1"
    }
    fn name(&self) -> &'static str
    {
        "Президент Российской Федерации"
    }
   // &DocumentTypes=&DocumentTypes=&PublishDateSearchType=0&NumberSearchType=0&DocumentDateSearchType=0&JdRegSearchType=0&SortedBy=6&SortDestination=1
    fn signatory_authority(&self) -> &'static str
    {
        signatory_authorites::ПРЕЗИДЕНТ_РОССИЙСКОЙ_ФЕДЕРАЦИИ
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
    use crate::extractors::{types, ActType, ExtractorPlugin};

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
        let number1 = plugin.get_raw_number(types::УКАЗ, "32123").unwrap();
        assert_eq!(number1, 32123);
        let number2 = plugin.get_raw_number(types::ФЕДЕРАЛЬНЫЙ_ЗАКОН, "333-ФЗ").unwrap();
        assert_eq!(number2, 333);
        let number3 = plugin.get_raw_number(types::РАСПОРЯЖЕНИЕ, "444-рп").unwrap();
        assert_eq!(number3, 444);
        let number4 = plugin.get_raw_number(types::ФЕДЕРАЛЬНЫЙ_ЗАКОН, "475-ФЗ").unwrap();
        assert_eq!(number4, 475);
        
    }

    #[test]
    fn test_skipped_numbers()
    {
        let plugin = super::PrezidentPlugin {  };
        logger::StructLogger::new_default();
        let numbers = vec![
            "166-ФЗ".to_owned(),
            "167-ФЗ".to_owned(),
            "169-ФЗ".to_owned(),
            "170-ФЗ".to_owned(),
            "180-ФЗ".to_owned()
        ];
        let skipped = plugin.get_skip_numbers(types::ФЕДЕРАЛЬНЫЙ_ЗАКОН, &numbers).unwrap();
        logger::info!("{:?}", skipped);
        assert_eq!(skipped, vec!["168-ФЗ", "171-ФЗ", "172-ФЗ", "173-ФЗ", "174-ФЗ", "175-ФЗ", "176-ФЗ", "177-ФЗ", "178-ФЗ", "179-ФЗ"]);
    }
}