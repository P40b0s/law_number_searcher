mod prezident;
pub mod plugin_trait;
mod bash;
pub use plugin_trait::ExtractorPlugin;
pub mod number_extractors;
pub mod types;
pub mod signatory_authorites;
mod default;
mod extractor_manager;
pub use extractor_manager::ExtractorManager;


#[macro_export]
macro_rules! create_error {
    ($e:expr) => {{
        logger::error!("{}", $e);
        Err($e)
    }};
}

/// `struct_name` наименование структуры
/// `regex_pattern` паттерн регекса
/// `sa` id signatory authority
#[macro_export]
macro_rules! create_plugin 
{
    ($struct_name:ident, $regex_pattern:literal, $sa:expr, $off_site:expr) => 
    {
        use regex::Regex;
        #[derive(Debug)]
        pub struct $struct_name where Self: Send + Sync {}
        static NUMBER_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new($regex_pattern).unwrap());
        impl<'a> ExtractorPlugin<'a> for $struct_name
        {
            //&DocumentTypes=&DocumentTypes=&PublishDateSearchType=0&NumberSearchType=0&DocumentDateSearchType=0&JdRegSearchType=0&SortedBy=6&SortDestination=1
            fn signatory_authority(&self) -> &'static str
            {
                $sa
            }
            fn official_publication_url(&self) -> Option<&'static str> 
            {
                $off_site
            }
            fn number_is_support(&'a self, number: &str) -> bool
            {
                NUMBER_RE.find(number).is_some()
            }
            fn get_raw_number<'b>(&'a self, act_type: &str, number: &'b str) -> Result<Number, crate::error::ExtractorError>
            {
                //указы распоряжения итд со всякими постфиксами точно пападут под этот регекс, поэтому обработать нужно будет только крайние случаи
                if let Some(caps) = NUMBER_RE.captures(number)
                {
                    if let Some(n) = caps.name("number").and_then(|pn| pn.as_str().parse().ok())
                    {
                        return Ok(Number 
                        {
                            number: n,
                            postfix: caps.name("postfix").and_then(|p| Some(p.as_str().to_owned())),
                            prefix: caps.name("prefix").and_then(|p| Some(p.as_str().to_owned())),
                        });
                    }
                    else
                    {
                        return Err(crate::error::ExtractorError::NumberFormatError(number.to_owned()));
                    } 
                }
                else
                {
                    return Err(crate::error::ExtractorError::NumberFormatError(number.to_owned()));
                }
            }
        }
        impl $struct_name
        {
            pub fn get_plugin<'a>() -> Box<dyn ExtractorPlugin<'a>>
            {
                let plugin: Box<dyn ExtractorPlugin> = Box::new($struct_name{});
                plugin
            }
        }
    };
}


#[cfg(test)]
mod tests
{
    use plugin_trait::Number;
    use regex::Match;

    use super::*;
    #[test]
    fn test_act_type_from_str()
    {
        logger::StructLogger::new_default();

        let e = crate::error::ExtractorError::ParseActTypeError("123321".to_owned());
        //let e1 = create_error!(e);
    }
    #[test]
    fn test_macro()
    {
        logger::StructLogger::new_default();
        create_plugin!(TestStuct, "e", "asqwqweqweqwe", None);
        let ts = TestStuct::get_plugin();

        logger::debug!("{:?}", ts.signatory_authority());
    }
}



