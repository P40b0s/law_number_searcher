use std::sync::{Arc, LazyLock};
pub use error::SearcherError;
use plugins::{ExtractorManager, ExtractorPlugin};
use publication_api::PublicationDocumentCard;
pub use publication_api::{SignatoryAuthority, PublicationApiError, DocumentType};
use regex::Regex;
use serde::{Deserialize, Serialize};
use utilites::Date;
mod error;
static PLUGINS: LazyLock<Arc<ExtractorManager>> = LazyLock::new(|| Arc::new(ExtractorManager::new()));
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Dictionary
{
    pub id: String,
    pub name: String,
    /// тип парсера  
    /// 0 - default  
    /// 1 - кастомная имлементация парсера под конкретный signatory_authority id  
    /// -1 - данный документ не поддерживается
    pub parser_type: i8
}
impl Into<Dictionary> for SignatoryAuthority
{
    fn into(self) -> Dictionary 
    {
        Dictionary 
        {
            id: self.id,
            name: self.name,
            parser_type: 0
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
            parser_type: 0
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
                d.parser_type = Self::organ_parser_type(&d.id);
                d
            }
        ).collect();
        Ok(organs)
    }

    pub async fn get_types(sa: &str) -> Result<Vec<Dictionary>, SearcherError>
    {
        let types = publication_api::PublicationApi::get_documents_types_by_signatory_authority(sa).await?;
        //let types_with_parsers = Self::get_types_in_parser(sa).await?;
        let plugin = PLUGINS.get_plugin(sa)?;
        let mut result: Vec<Dictionary> = Vec::with_capacity(types.len());
        for dt in types
        {
            let mut d: Dictionary = dt.into();
            let first_number = Self::get_first_number(sa, &d.id).await.unwrap_or("".to_owned());
            let support = plugin.number_is_support(&first_number);
            let organ_parser = Self::organ_parser_type(sa);
            if support
            {
                d.parser_type = organ_parser
            }
            else 
            {
                d.parser_type = -1;
            }
            result.push(d);
        }
        // let types: Vec<Dictionary> = types.into_iter().map(|t|
        // {
        //     let mut d: Dictionary = t.into();
        //     let first_number = publication_api::PublicationApi::get_first_document(sa, &d.id).await;
        //     let support = plugin.number_is_support(&d.id, number);
        //     let organ_parser = Self::organ_parser_type(sa);
        //     if support
        //     {
        //         d.parser_type = 
        //     }
        //     d.parser_type = ;
        //     d
        // }).collect();
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
        let plugin = PLUGINS.get_plugin(sa).unwrap();
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
    pub async fn get_exists_numbers(signatory_authority: &str, doc_type: &str, year: u32) -> Result<Vec<String>, SearcherError>
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
            None).await?;
        //let mut numbers = Vec::with_capacity(docs.len());
        let result: Vec<String> = docs.into_iter().map(|d| d.number).collect();
        //static RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(r"№\s+(.+)\s").unwrap());
        // for d in docs
        // {
        //     if let Some(number) = RE.captures(&d.complex_name)
        //     {
        //         numbers.push(number[1].to_owned());
        //     }
        //     else 
        //     {
        //         logger::error!("Ошибка извлечения номера из названия `{}` -> `{}`", d.complex_name, d.eo_number);
        //         return Err(SearcherError::ParseNumberError(d.complex_name, d.eo_number))
        //     }
        // }
        Ok(result)
    }

    /// получение всех пропущеных номеров
    pub async fn get_lost_numbers(signatory_authority: &str, doc_type: &str, year: u32) -> Result<Vec<String>, SearcherError>
    {
        let plugin = PLUGINS.get_plugin(signatory_authority)?;
        let numbers = Self::get_exists_numbers(signatory_authority, doc_type, year).await?;
        //logger::debug!("numbers {:?}", &numbers);
        let skipped = plugin.get_skip_numbers(doc_type, numbers)?;
        Ok(skipped)
    }
    /// получение номера первого документа из списка
    pub async fn get_first_number(signatory_authority: &str, doc_type: &str) -> Result<String, SearcherError>
    {
        let first = publication_api::PublicationApi::get_first_document(signatory_authority, doc_type).await?;
        Ok(first.number)
    }
}



#[cfg(test)]
mod tests
{
    use regex::Regex;
    use serde_json::error;

    #[tokio::test]
    async fn test_get_organs()
    {
        logger::StructLogger::new_default();
        let organs = super::Searcher::get_signatory_authorites().await.unwrap();
        logger::debug!("{:?}", organs);
    } 
    #[tokio::test]
    async fn test_get_all_numbers()
    {
        logger::StructLogger::new_default();
        let organs = super::Searcher::get_exists_numbers(plugins::signatory_authorites::ПРЕЗИДЕНТ_РОССИЙСКОЙ_ФЕДЕРАЦИИ, plugins::types::УКАЗ, 2024).await.unwrap();
        logger::debug!("{:?}", organs);
    } 
    #[tokio::test]
    async fn test_get_skipped_numbers()
    {
        logger::StructLogger::new_default();
        let organs = super::Searcher::get_lost_numbers(plugins::signatory_authorites::ПРЕЗИДЕНТ_РОССИЙСКОЙ_ФЕДЕРАЦИИ, plugins::types::УКАЗ, 2024).await.unwrap();
        logger::debug!("{:?}", organs);
    } 
    #[tokio::test]
    async fn test_get_skipped_numbers_fz()
    {
        logger::StructLogger::new_default();
        let organs = super::Searcher::get_lost_numbers(plugins::signatory_authorites::ПРЕЗИДЕНТ_РОССИЙСКОЙ_ФЕДЕРАЦИИ, plugins::types::ФЕДЕРАЛЬНЫЙ_ЗАКОН, 2024).await.unwrap();
        logger::debug!("{:?}", organs);
    } 
    #[tokio::test]
    /// получать карточку каждого документа это очень долго, есть вариант взять номер из полного наименования
    async fn test_split()
    {
        logger::StructLogger::new_default();
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

    // 'Федеральный закон от 22.06.2024 № 160-ФЗ\n "О внесении изменений в статью 19 Федерального закона "О крестьянском (фермерском) хозяйстве" и Федеральный закон "О развитии сельского хозяйства"'
}