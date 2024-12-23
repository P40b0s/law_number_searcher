mod prezident;
pub mod plugin_trait;
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



