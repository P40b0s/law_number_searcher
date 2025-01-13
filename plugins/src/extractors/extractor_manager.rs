use hashbrown::HashMap;
use super::{default::DefaultPlugin, plugin_trait::ExtractorPlugin, prezident::PrezidentPlugin};
use crate::ExtractorError;


pub struct ExtractorManager<'a> where Self: Send + Sync 
{
    extractors: hashbrown::HashMap<String, Box<dyn ExtractorPlugin<'a>>>
}
impl<'a> ExtractorManager<'a>
{
    pub fn new() -> Self
    {
        let mut hm = HashMap::new();
        let default_plugin = Box::new(DefaultPlugin{});
        let prez_plugin: Box<dyn ExtractorPlugin> = Box::new(PrezidentPlugin{});
        let bash_plugin = super::bash::CustomPlugin::get_plugin();
        hm.insert(prez_plugin.signatory_authority().to_owned(), prez_plugin);
        hm.insert(bash_plugin.signatory_authority().to_owned(), bash_plugin);
        hm.insert(default_plugin.signatory_authority().to_owned(), default_plugin);
        Self
        {
            extractors: hm
        }
    }
    /// получить плагин для разбора документа, если плагин не найден по signatory authority id то отдаем дефолтный
    pub fn get_plugin(&self, signatory_authority: &str) -> Result<&Box<dyn ExtractorPlugin<'a>>, ExtractorError>
    {
        if let Some(plugin) = self.extractors.get(signatory_authority)
        {
            Ok(plugin)
        }
        else 
        {
           Ok(self.extractors.get("default").unwrap())
        }
    }
}
