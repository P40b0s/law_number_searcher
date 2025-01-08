use hashbrown::HashMap;
use super::{default::DefaultPlugin, plugin_trait::NumberExtractorPlugin, OffSiteParser};
use crate::{extractors::bash::BashOffSiteParser, signatory_authorites};


// macro_rules! add_plugins
// {
//     ($([$extractors:expr, $parsers:expr, $signatory_authority:expr, $extractor:expr, $parser:expr]),+) => 
//     {{
//         $(
//             let plugin: Box<dyn NumberExtractorPlugin> = Box::new($extractor);
//             $extractors.insert($signatory_authority.to_owned(), plugin);
//             if $parser.is_some()
//             {
//                 let parser: Box<dyn OffSiteParser> = Box::new($parser.unwrap());
//                 $parsers.insert($signatory_authority.to_owned(), parser);
//             }
//         )+
//     }};
//     ($([$signatory_authority:expr, $extractor:expr]),+) => 
//     {{
//         let mut extractors = HashMap::new();
//         let mut parsers = HashMap::new();
//         $(
//             let plugin: Box<dyn NumberExtractorPlugin> = Box::new($extractor);
//             extractors.insert($signatory_authority.to_owned(), plugin);
//         )+
//         return ExtractorManager 
//         {
//             extractors,
//             parsers
//         };
//     }};
// }


struct PluginRegistrator<'a>
{
    extractors: hashbrown::HashMap<String, Box<dyn NumberExtractorPlugin<'a>>>,
    parsers: hashbrown::HashMap<String, Box<dyn OffSiteParser>>
}
impl<'a> Default for PluginRegistrator<'a>
{
    fn default() -> Self {
        Self
        {
            parsers: HashMap::new(),
            extractors: HashMap::new()
        }
    }
}
impl<'a> Into<ExtractorManager<'a>> for PluginRegistrator<'a>
{
    fn into(self) -> ExtractorManager<'a> 
    {
        ExtractorManager
        {
            extractors: self.extractors,
            parsers: self.parsers
        }
    }
}
impl<'a> PluginRegistrator<'a>
{
    fn new() -> Self
    {
        Self
        {
            parsers: HashMap::new(),
            extractors: HashMap::new()
        }
    }
    fn register_plugin<K: ToString, E: NumberExtractorPlugin<'a>>(&mut self, key: K, plugin: E)
    {
        let plugin: Box<dyn NumberExtractorPlugin<'a>> = Box::new(plugin);
        self.extractors.insert(key.to_string(), plugin);
    }
    fn register_parser<K: ToString, P>(&mut self, key: K, parser: P) 
    where P: OffSiteParser + 'static
    {
        let parser: Box<dyn OffSiteParser> = Box::new(parser);
        self.parsers.insert(key.to_string(), parser);
    }
    fn register_all<K: ToString, E, P>(&mut self, key: K, plugin: E, parser: P) 
    where P: OffSiteParser + 'static, E: NumberExtractorPlugin<'a>
    {
        let plugin: Box<dyn NumberExtractorPlugin<'a>> = Box::new(plugin);
        let parser: Box<dyn OffSiteParser> = Box::new(parser);
        self.extractors.insert(key.to_string(), plugin);
        self.parsers.insert(key.to_string(), parser);
    }
   
}

pub struct ExtractorManager<'a> where Self: Send + Sync 
{
    extractors: hashbrown::HashMap<String, Box<dyn NumberExtractorPlugin<'a>>>,
    parsers: hashbrown::HashMap<String, Box<dyn OffSiteParser>>
}
impl<'a> ExtractorManager<'a>
{
    pub fn new() -> ExtractorManager<'a>
    {
        let mut registrator = PluginRegistrator::new();
        
        registrator.register_plugin("default", DefaultPlugin{});
        //registrator.register_plugin(signatory_authorites::ПРЕЗИДЕНТ_РОССИЙСКОЙ_ФЕДЕРАЦИИ, PrezidentPlugin{});
        registrator.register_all(signatory_authorites::ГЛАВА_РЕСПУБЛИКИ_БАШКОРТОСТАН, super::bash::HeadPlugin::new(), BashOffSiteParser::new());
        registrator.register_parser(signatory_authorites::РЕСПУБЛИКА_БАШКОРТОСТАН, BashOffSiteParser::new());
        registrator.register_parser(signatory_authorites::РЕСПУБЛИКА_БУРЯТИЯ, super::burat::BuryatOffSiteParser::new());
        let extractor = registrator.into();
        extractor
        
    }
    /// получить плагин для разбора документа, если плагин не найден по signatory authority id то отдаем дефолтный
    pub fn get_number_extractor_plugin(&self, signatory_authority: &str) -> &Box<dyn NumberExtractorPlugin<'a>>
    {
        if let Some(plugin) = self.extractors.get(signatory_authority)
        {
            plugin
        }
        else 
        {
           self.extractors.get("default").unwrap()
        }
    }
    pub fn get_off_site_parser(&self, signatory_authority: &str) -> Option<&Box<dyn OffSiteParser>>
    {
        self.parsers.get(signatory_authority)
    }
}
