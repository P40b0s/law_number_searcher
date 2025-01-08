//mod prezident;
pub mod plugin_trait;
mod bash;
mod burat;
pub use plugin_trait::{NumberExtractorPlugin, OffSiteParser};
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
    ($struct_name:ident, $sa:expr, $($regex_pattern:expr),+) => 
    {
        #[derive(Debug)]
        pub struct $struct_name where Self: Send + Sync 
        {
            regexes: std::sync::LazyLock<Vec<regex::Regex>>
        }
        impl $struct_name
        {
            pub fn new() -> Self
            {
                Self
                {
                    regexes: std::sync::LazyLock::new(||
                    {
                        let mut v = Vec::new();
                        $(
                            v.push(regex::Regex::new($regex_pattern).unwrap());
                        )+
                        v
                    })
                }
            }
        }

        impl<'a> NumberExtractorPlugin<'a> for $struct_name
        {
            fn signatory_authority(&self) -> &'static str
            {
                $sa
            }

            fn number_is_support(&'a self, number: &str) -> bool
            {
                self.regexes.iter().any(|s| s.find(number).is_some())
                //NUMBERS_RE.find(number).is_some()
            }

            fn get_raw_number<'b>(&'a self, _act_type: &str, number: &'b str) -> Result<Number, crate::error::ExtractorError>
            {
                for re in self.regexes.iter()
                {
                    if let Some(caps) = re.captures(number)
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
                    }
                }
                return Err(crate::error::ExtractorError::NumberFormatError(number.to_owned()));
            }
        }
    };
}

#[macro_export]
macro_rules! create_parser
{
    ($struct_name:ident, $site_url:expr, $api_url:expr, [$($regex_pattern:expr),*], $f:expr) => 
    {
        #[derive(Debug)]
        pub struct $struct_name where Self: Send + Sync 
        {
            regexes: std::sync::Arc<std::sync::LazyLock<Vec<regex::Regex>>>
        }
        impl $struct_name
        {
            pub fn new() -> Self
            {
                Self
                {
                    regexes: std::sync::Arc::new(std::sync::LazyLock::new(||
                    {
                        #[allow(unused_mut)]
                        let mut v = Vec::new();
                        $(
                            v.push(regex::Regex::new($regex_pattern).unwrap());
                        )*
                        v
                    }))
                }
            }
        }
        
        impl OffSiteParser for $struct_name
        {
            fn official_publication_url(&self) -> &'static str
            {
                $site_url
            }
            fn check_numbers_on_alternative_site<'a>(&'a self, sa: &'a str, act_type: &'a str, year: u32, sender: Option<tokio::sync::mpsc::Sender<String>>) 
            -> BoxFuture<'a, Result<Vec<String>, crate::error::ExtractorError>>
            {
                Box::pin(async move 
                {
                   $f(std::sync::Arc::clone(&self.regexes), $api_url, sa, act_type, year, sender).await
                })
            }
        }
    };
}

#[cfg(test)]
mod tests
{
    #[test]
    fn test_act_type_from_str()
    {
        let _ = logger::StructLogger::new_default();

        let _ = crate::error::ExtractorError::ParseActTypeError("123321".to_owned());
        //let e1 = create_error!(e);
    }
    #[test]
    fn test_macro()
    {
        let _ = logger::StructLogger::new_default();
        //create_plugin2!("1", "2", "3", "4");
        
        //create_plugin!(TestStuct, "asqwqweqweqwe", None, "1", "2", "3", |_| Box::pin(async move {Ok(None)}));
        //let ts = TestStuct::get_plugin();

        //logger::debug!("{:?}", ts.signatory_authority());
    }
}



