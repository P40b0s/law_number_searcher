use std::u32;
use logger::info;
use crate::ExtractorError;
use super::plugin_trait::{Number, CLEAR_NUMBER_RE};
use super::{number_extractors, signatory_authorites, types};
use super::NumberExtractorPlugin;
use futures::future::{BoxFuture, FutureExt};
#[derive(Debug)]
pub struct PrezidentPlugin where Self: Send + Sync {}

impl<'a> NumberExtractorPlugin<'a> for PrezidentPlugin
{
   // &DocumentTypes=&DocumentTypes=&PublishDateSearchType=0&NumberSearchType=0&DocumentDateSearchType=0&JdRegSearchType=0&SortedBy=6&SortDestination=1
    fn signatory_authority(&self) -> &'static str
    {
        signatory_authorites::ПРЕЗИДЕНТ_РОССИЙСКОЙ_ФЕДЕРАЦИИ
    }
    fn get_raw_number<'b>(&'a self, act_type: &str,  number: &'b str) -> Result<Number, crate::error::ExtractorError>
    {
        match act_type
        {
            types::УКАЗ 
            | types::РАСПОРЯЖЕНИЕ 
            | types::ФЕДЕРАЛЬНЫЙ_ЗАКОН 
            | types::ФЕДЕРАЛЬНЫЙ_КОНСТИТУЦИОННЫЙ_ЗАКОН => 
            {
                if let Some(mch) = CLEAR_NUMBER_RE.find(number)
                {
                    let index = mch.end();
                    let n = number.split_at(index);
                    return Ok(Number 
                    {
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
                    
            },
            _ => Err(crate::error::ExtractorError::NumberFormatError(number.to_owned()))
        }
        // //указы распоряжения итд со всякими постфиксами точно пападут под этот регекс, поэтому обработать нужно будет только крайние случаи
        // if let Some(mch) = CLEAR_NUMBER_RE.find(number)
        // {
        //     let index = mch.end();
        //     let n = number.split_at(index);
        //     return Ok(Number 
        //     {
        //         number: n.0.parse().unwrap(),
        //         postfix: Some(n.1),
        //         prefix: None
        //     });
        // }
        // else
        // {
        //     match act_type
        //     {
        //         _ => Err(crate::error::ExtractorError::NumberFormatError(number.to_owned()))
        //     }
        // }
    }
    fn number_is_support(&'a self, number: &str) -> bool
    {
        CLEAR_NUMBER_RE.find(number).is_some()
    }
}


#[cfg(test)]
mod tests
{
    use crate::extractors::plugin_trait::NumberExtractorPlugin;
    use crate::extractors::types;

    #[test]
    fn test_raw_numbers()
    {
        let plugin = super::PrezidentPlugin {  };
        logger::StructLogger::new_default();
        let number1 = plugin.get_raw_number(types::УКАЗ, "32123").unwrap();
        assert_eq!(number1.number, 32123);
        let number2 = plugin.get_raw_number(types::ФЕДЕРАЛЬНЫЙ_ЗАКОН, "333-ФЗ").unwrap();
        assert_eq!(number2.number, 333);
        let number3 = plugin.get_raw_number(types::РАСПОРЯЖЕНИЕ, "444-рп").unwrap();
        assert_eq!(number3.number, 444);
        let number4 = plugin.get_raw_number(types::ФЕДЕРАЛЬНЫЙ_ЗАКОН, "475-ФЗ").unwrap();
        assert_eq!(number4.number, 475);
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
        let skipped = plugin.get_skip_numbers(types::ФЕДЕРАЛЬНЫЙ_ЗАКОН, numbers).unwrap();
        logger::info!("{:?}", skipped);
        assert_eq!(skipped, vec!["168-ФЗ", "171-ФЗ", "172-ФЗ", "173-ФЗ", "174-ФЗ", "175-ФЗ", "176-ФЗ", "177-ФЗ", "178-ФЗ", "179-ФЗ"]);
    }
}