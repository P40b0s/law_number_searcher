use std::cell::OnceCell;
use std::u32;
use logger::info;
use regex::Regex;
use crate::ExtractorError;
use super::plugin_trait::Number;
use super::{number_extractors, signatory_authorites, types};
use super::ExtractorPlugin;

#[derive(Debug)]
pub struct GlavaBashPlugin where Self: Send + Sync {}
static NUMBER_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(r"УГ-\d{1,4}").unwrap());
impl<'a> ExtractorPlugin<'a> for GlavaBashPlugin
{
    //&DocumentTypes=&DocumentTypes=&PublishDateSearchType=0&NumberSearchType=0&DocumentDateSearchType=0&JdRegSearchType=0&SortedBy=6&SortDestination=1
    fn signatory_authority(&self) -> &'static str
    {
        signatory_authorites::ГЛАВА_РЕСПУБЛИКИ_БАШКОРТОСТАН
    }
    fn official_publication_url(&self) -> Option<&'static str> 
    {
        None
    }
    fn number_is_support(&'a self, number: &str) -> bool
    {
        NUMBER_RE.find(number).is_some()
    }
    fn get_raw_number<'b>(&'a self, act_type: &str,  number: &'b str) -> Result<Number<'b>, crate::error::ExtractorError>
    {
        //указы распоряжения итд со всякими постфиксами точно пападут под этот регекс, поэтому обработать нужно будет только крайние случаи
        if let Some(mch) = NUMBER_RE.find(number)
        {
            let index = mch.end();
            let n = number.split_at(index);
            return Ok(Number 
            {
                number: n.0.parse().unwrap(),
                postfix: Some(n.1),
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
}