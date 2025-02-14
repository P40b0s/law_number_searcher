use std::sync::{Arc, LazyLock};
pub use error::SearcherError;
use plugins::ExtractorManager;
pub use publication_api::{SignatoryAuthority, PublicationApiError, DocumentType};
use serde::{Deserialize, Serialize, Serializer};
use utilites::Date;
mod error;
static PLUGINS: LazyLock<Arc<ExtractorManager>> = LazyLock::new(|| Arc::new(ExtractorManager::new()));

#[derive(Deserialize, Debug)]
pub enum DictionaryType
{
    Organ,
    Type
}

impl serde::Serialize for DictionaryType
{
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self
        {
            DictionaryType::Organ => s.serialize_str("organ"),
            DictionaryType::Type => s.serialize_str("type")
        }
    }
}
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Dictionary
{
    pub id: String,
    pub name: String,
    /// тип парсера  
    /// 0 - default  
    /// 1 - кастомная имлементация парсера под конкретный signatory_authority id 
    /// 2 - для проверки номера документ небыл найден, значит за год нет ни одного документа
    /// -1 - данный документ не поддерживается
    pub parser_type: i8,
    pub number_example: Option<String>,
    pub data_type: DictionaryType,
    pub alternative_site: Option<String>
}

impl Into<Dictionary> for SignatoryAuthority
{
    fn into(self) -> Dictionary 
    {
        Dictionary 
        {
            id: self.id,
            name: self.name,
            parser_type: 0,
            number_example: None,
            data_type: DictionaryType::Organ,
            alternative_site: None
        }
    }
}
impl Into<Dictionary> for DocumentType
{
    fn into(self) -> Dictionary 
    {
        Dictionary 
        {
            id: self.id,
            name: self.name,
            parser_type: 0,
            number_example: None,
            data_type: DictionaryType::Type,
            alternative_site: None
        }
    }
}

pub struct Searcher{}
///так как DocumentType и signatory authority одинаковые и идут с весами которые нам ненужны, но отсуствует поле что для них есть парсер которое нам как раз нужно, сделаем новую структуру и будет все конвертить в нее
impl Searcher
{
    pub async fn get_signatory_authorites() -> Result<Vec<Dictionary>, SearcherError>
    {
        let organs = publication_api::PublicationApi::get_signatory_authorites().await?;
        let organs: Vec<Dictionary> = organs.into_iter().map(|o| 
            {
                let mut d: Dictionary = o.into();
                let parser = Self::get_alternative_publ_site(&d.id);
                d.parser_type = Self::organ_parser_type(&d.id);
                d.data_type = DictionaryType::Organ;
                d.alternative_site = parser.as_ref().and_then(|p| Some(p.to_string()));
                d
            }
        ).collect();
        Ok(organs)
    }

    pub async fn get_types(sa: &str, sender: Option<tokio::sync::mpsc::Sender<u32>>) -> Result<Vec<Dictionary>, SearcherError>
    {
        let types = publication_api::PublicationApi::get_documents_types_by_signatory_authority(sa).await?;
        //let types_with_parsers = Self::get_types_in_parser(sa).await?;
        let plugin = PLUGINS.get_number_extractor_plugin(sa);
        let mut result: Vec<Dictionary> = Vec::with_capacity(types.len());
        let percentage_mul = 100 / types.len() as u32;
        for (i, dt) in types.into_iter().enumerate()
        {
            let mut d: Dictionary = dt.into();
            d.data_type = DictionaryType::Type;
            let first_number = Self::get_first_number(sa, &d.id).await?;
            let organ_parser = Self::organ_parser_type(sa);
            if let Some(f_n) = first_number
            {
                let support = plugin.number_is_support(&f_n);
                if support
                {
                    d.parser_type = organ_parser
                }
                else 
                {
                    d.parser_type = -1;
                }
                d.number_example = Some(f_n);
            }
            else 
            {
                d.parser_type = 2;
            }
            if let Some(c) = sender.as_ref()
            {
                let _ = c.send(percentage_mul * (i+1) as u32).await;
            }
            result.push(d);
        }
        Ok(result)
    }
        //TODO необходимо сделать минимальную выборку и взять первый номер документа, проверить его, можем ли мы его парсить, и уже это показать в виде докуента можем ли мы его парсить или нет
    //http://publication.pravo.gov.ru/api/Documents?SignatoryAuthorityId=8d31525e-fafc-4590-8580-422f588d20c9&DocumentTypes=2dddb344-d3e2-4785-a899-7aa12bd47b6f&pageSize=10&index=1
    
    // pub async fn get_types_in_parser(sa: &str) -> Result<Vec<String>, SearcherError>
    // {
    //     let plugin = PLUGINS.get_plugin(sa)?;
    //     let ids: Vec<String> = plugin.type_ids().into_iter().map(|t| t.to_string()).collect();
    //     Ok(ids)
    // }

    fn organ_parser_type(sa: &str) -> i8
    {
        let plugin = PLUGINS.get_number_extractor_plugin(sa);
        if plugin.signatory_authority() == "default"
        {
            0
        }
        else 
        {
            1
        }
    }

    ///получаем все номера документов за текущий год
    pub async fn get_exists_numbers(signatory_authority: &str, doc_type: &str, year: u32, sender: Option<tokio::sync::mpsc::Sender<u32>>) -> Result<Vec<String>, SearcherError>
    {
        let date_from_format = ["01.01.".to_owned(), year.to_string()].concat();
        let date_to_format = ["31.12.".to_owned(), year.to_string()].concat();
        let date_from  = Date::parse(&date_from_format);
        if date_from.is_none()
        {
            return Err(SearcherError::DateFormatError(date_from_format));
        }
        let date_to  = Date::parse(&date_to_format);
        if date_to.is_none()
        {
            return Err(SearcherError::DateFormatError(date_to_format));
        }
        let docs = publication_api::PublicationApi::get_documents_for_period(
            date_from.as_ref().unwrap(),
            date_to.as_ref().unwrap(),
            &[doc_type.to_owned()],
            Some(&signatory_authority.to_owned()),
            None,
            sender).await?;
        //let mut numbers = Vec::with_capacity(docs.len());
        let result: Vec<String> = docs.into_iter().map(|d| d.number).collect();
        Ok(result)
    }

    /// получение всех пропущеных номеров
    pub async fn get_lost_numbers(signatory_authority: &str, doc_type: &str, year: u32, sender: Option<tokio::sync::mpsc::Sender<u32>>) -> Result<Vec<String>, SearcherError>
    {
        let plugin = PLUGINS.get_number_extractor_plugin(signatory_authority);
        let numbers = Self::get_exists_numbers(signatory_authority, doc_type, year, sender).await?;
        //logger::debug!("numbers {:?}", &numbers);
        let skipped = plugin.get_skip_numbers(doc_type, numbers)?;
        Ok(skipped)
    }

    /// получение всех пропущеных номеров
    pub async fn get_alternative_site_numbers(signatory_authority: &str, doc_type: &str, year: u32, sender: Option<tokio::sync::mpsc::Sender<u32>>) -> Result<Vec<String>, SearcherError>
    {
        let plugin = PLUGINS.get_number_extractor_plugin(signatory_authority);
        let numbers = Self::get_exists_numbers(signatory_authority, doc_type, year, sender).await?;
        //logger::debug!("numbers {:?}", &numbers);
        let skipped = plugin.get_skip_numbers(doc_type, numbers)?;
        Ok(skipped)
    }

    /// получение номера первого документа из списка
    pub async fn get_first_number(signatory_authority: &str, doc_type: &str) -> Result<Option<String>, SearcherError>
    {
        let first = publication_api::PublicationApi::get_first_document(signatory_authority, doc_type).await?;
        Ok(first.and_then(|n| Some(n.number)))
    }

    pub fn get_alternative_publ_site(sa: &str) -> Option<&str>
    {
        let plugin = PLUGINS.get_off_site_parser(sa);
        if let Some(p) = plugin
        {
            Some(p.official_publication_url())
        }
        else 
        {
            None
        }
    }

    pub async fn check_alternative_publ_site_info(sa: &str, doc_type: &str, year: u32, sender: Option<tokio::sync::mpsc::Sender<String>>) -> Result<Vec<String>, SearcherError>
    {
        let plugin = PLUGINS.get_off_site_parser(sa);
        if let Some(p) = plugin
        {
            let numbers = p.check_numbers_on_alternative_site(sa, doc_type, year, sender).await?;
            Ok(numbers)
        }
        else 
        {
            Err(SearcherError::AlternativeSiteParserError(sa.to_owned(), doc_type.to_owned()))
        }
    }
   
}



#[cfg(test)]
mod tests
{
    use regex::Regex;

    use crate::Dictionary;

    #[tokio::test]
    async fn test_get_organs()
    {
        let _ = logger::StructLogger::new_default();
        let organs = super::Searcher::get_signatory_authorites().await.unwrap();
        logger::debug!("{:?}", organs);
    } 
    #[tokio::test]
    async fn test_get_all_numbers()
    {
        let _ = logger::StructLogger::new_default();
        let (sender, mut receiver) =  tokio::sync::mpsc::channel::<u32>(1);
        let s = tokio::spawn(
            async move 
            {
                while let Some(p) = receiver.recv().await 
                {
                    logger::info!("текущий процент выполнения: {}%", p);
                }
            });
        let organs = super::Searcher::get_exists_numbers(plugins::signatory_authorites::ПРЕЗИДЕНТ_РОССИЙСКОЙ_ФЕДЕРАЦИИ, plugins::types::УКАЗ, 2024, Some(sender)).await.unwrap();
        logger::debug!("{:?}", organs);
        let _ = s.await;
    } 
    #[tokio::test]
    async fn test_get_skipped_numbers()
    {
        let _ = logger::StructLogger::new_default();
        let (sender, mut receiver) =  tokio::sync::mpsc::channel::<u32>(1);
        let s = tokio::spawn(
            async move 
            {
                while let Some(p) = receiver.recv().await 
                {
                    logger::info!("текущий процент выполнения: {}%", p);
                }
            });
        let organs = super::Searcher::get_lost_numbers(plugins::signatory_authorites::ПРЕЗИДЕНТ_РОССИЙСКОЙ_ФЕДЕРАЦИИ, plugins::types::УКАЗ, 2024, Some(sender)).await.unwrap();
        logger::debug!("{:?}", organs);
        let _ = s.await;
    } 
    #[tokio::test]
    async fn test_get_skipped_numbers_fz()
    {
        let _ = logger::StructLogger::new_default();
        let (sender, mut receiver) =  tokio::sync::mpsc::channel::<u32>(1);
        let s = tokio::spawn(
            async move 
            {
                while let Some(p) = receiver.recv().await 
                {
                    logger::info!("текущий процент выполнения: {}%", p);
                }
            });
        let organs = super::Searcher::get_lost_numbers(plugins::signatory_authorites::ПРЕЗИДЕНТ_РОССИЙСКОЙ_ФЕДЕРАЦИИ, plugins::types::ФЕДЕРАЛЬНЫЙ_ЗАКОН, 2024, Some(sender)).await.unwrap();
        logger::debug!("{:?}", organs);
        let _ = s.await;
    } 
    #[tokio::test]
    /// получать карточку каждого документа это очень долго, есть вариант взять номер из полного наименования
    async fn test_split()
    {
        let _ = logger::StructLogger::new_default();
        let text = r#"Федеральный закон от 13.12.2024 № 475-ФЗ\n \"О внесении изменений в отдельные законодательные акты Российской Федерации\""#;
        static RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(r"№\s+(.+)\s").unwrap());
        if let Some(caps) = RE.captures(text) 
        {
            logger::debug!("{:?}", &caps[1]);
        }
        else 
        { 
            logger::error!("ERROR!");
        };
        //let Some(caps) = RE.captures(text) else { return };
    } 

    #[test]
    /// получать карточку каждого документа это очень долго, есть вариант взять номер из полного наименования
    fn test_serialize_dictionary()
    {
        let _ = logger::StructLogger::new_default();
        let d = Dictionary
        {
            id: "123321".to_owned(),
            name: "Закон".to_owned(),
            parser_type: 2,
            number_example: Some("123-AP".to_owned()),
            data_type: crate::DictionaryType::Organ,
            alternative_site: Some("www.rrr.ru".to_owned())
        };
        let string = serde_json::to_string(&d).unwrap();
        logger::info!("{}", string);
    } 

    // 'Федеральный закон от 22.06.2024 № 160-ФЗ\n "О внесении изменений в статью 19 Федерального закона "О крестьянском (фермерском) хозяйстве" и Федеральный закон "О развитии сельского хозяйства"'
}