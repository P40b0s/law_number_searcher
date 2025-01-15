use hashbrown::HashMap;
use super::{default::DefaultPlugin, plugin_trait::NumberExtractorPlugin, prezident::PrezidentPlugin, OffSiteParser};
use crate::{extractors::{bash::BashOffSiteParser, default::DefaultParser}, signatory_authorites, ExtractorError};



macro_rules! add_plugins
{
    ($([$signatory_authority:expr, $extractor:expr, $parser:expr]),+) => 
    {{
        let mut extractors = HashMap::new();
        let mut parsers = HashMap::new();
        $(
            let plugin: Box<dyn NumberExtractorPlugin> = Box::new($extractor);
            extractors.insert($signatory_authority.to_owned(), plugin);
            if $parser.is_some()
            {
                let parser: Box<dyn OffSiteParser> = Box::new($parser.unwrap());
                parsers.insert($signatory_authority.to_owned(), parser);
            }
        )+
        return ExtractorManager 
        {
            extractors,
            parsers
        };
    }};
}


pub struct ExtractorManager<'a> where Self: Send + Sync 
{
    extractors: hashbrown::HashMap<String, Box<dyn NumberExtractorPlugin<'a>>>,
    parsers: hashbrown::HashMap<String, Box<dyn OffSiteParser>>
}
impl<'a> ExtractorManager<'a>
{
    pub fn new() -> Self
    {

        // let mut hm = HashMap::new();
        // let default_plugin = Box::new(DefaultPlugin{});
        // let prez_plugin: Box<dyn NumberExtractorPlugin> = Box::new(PrezidentPlugin{});
        // let bash_plugin = super::bash::CustomPlugin::get_plugin();
        // hm.insert(prez_plugin.signatory_authority().to_owned(), prez_plugin);
        // hm.insert(bash_plugin.signatory_authority().to_owned(), bash_plugin);
        // hm.insert(default_plugin.signatory_authority().to_owned(), default_plugin);
        // Self
        // {
        //     extractors: hm
        // }
        add_plugins!(
            ["default", DefaultPlugin{}, None::<DefaultParser>],
            [signatory_authorites::ПРЕЗИДЕНТ_РОССИЙСКОЙ_ФЕДЕРАЦИИ, PrezidentPlugin{}, None::<DefaultParser>],
            [signatory_authorites::ГЛАВА_РЕСПУБЛИКИ_БАШКОРТОСТАН, super::bash::HeadPlugin::new(), Some(BashOffSiteParser{})],
            [signatory_authorites::РЕСПУБЛИКА_БАШКОРТОСТАН, super::bash::RegionPlugin::new(), Some(BashOffSiteParser{})]
        )
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
