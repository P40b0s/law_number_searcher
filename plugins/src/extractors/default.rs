use regex::Regex;
use super::plugin_trait::Number;
use super::NumberExtractorPlugin;
///Если данного вида акта не будет в перечне плагинов, то будет кидать на этот плагин
#[derive(Debug)]
pub struct DefaultPlugin where Self: Send + Sync {}

static CLEAR_NUMBER_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(r"^\d{1,4}").unwrap());
impl<'a> NumberExtractorPlugin<'a> for DefaultPlugin
{
   // &DocumentTypes=&DocumentTypes=&PublishDateSearchType=0&NumberSearchType=0&DocumentDateSearchType=0&JdRegSearchType=0&SortedBy=6&SortDestination=1
    fn signatory_authority(&self) -> &'static str
    {
        "default"
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
}
