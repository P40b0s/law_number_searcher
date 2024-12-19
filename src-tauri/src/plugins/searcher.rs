use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime, State};

use crate::db::{AppRepository, IRepository, NumberDBO, Repository};
use crate::state::AppState;
use crate::Error;
use searcher::{SearcherError, Dictionary};
#[derive(Serialize, Deserialize)]
pub struct ExistsNumbersRequest<'a>
{
    signatory_authority: &'a str,
    act_type: &'a str,
    year: u32
}
#[derive(Serialize, Deserialize)]
pub struct Number
{
    signatory_authority: String,
    type_id: String,
    year: u32,
    number: String,
    note: Option<String>,
    status: u32
}

impl Into<Number> for NumberDBO
{
    fn into(self) -> Number 
    {
        Number
        {
            signatory_authority: self.signatory_authority.to_string(),
            type_id: self.type_id.to_string(),
            year: self.year,
            number: self.number,
            note: self.note,
            status: self.status
        }
    }
}

#[tauri::command]
pub async fn get_signatory_authorites() -> Result<Vec<Dictionary>, Error>
{
    let mut organs = searcher::Searcher::get_signatory_authorites().await?;
    organs.sort_by(|a, b| a.name.cmp(&b.name));
	logger::debug!("Получено органов: {}", organs.len());
    Ok(organs)
}

#[tauri::command]
pub async fn get_types(payload: &str) -> Result<Vec<Dictionary>, Error>
{
    let mut doc_types = searcher::Searcher::get_types(payload).await?;
    doc_types.sort_by(|a, b| a.name.cmp(&b.name));
	logger::debug!("Найдено типов документов: {}", doc_types.len());
    Ok(doc_types)
}

#[tauri::command]
pub async fn get_exists_numbers<'a>(ExistsNumbersRequest {signatory_authority, act_type, year}: ExistsNumbersRequest<'a>) -> Result<Vec<String>, Error>
{
    let mut numbers = searcher::Searcher::get_exists_numbers(signatory_authority, act_type, year).await?;
    //doc_types.sort_by(|a, b| a.name.cmp(&b.name));
	logger::debug!("Найдено номеров документов: {}", numbers.len());
    Ok(numbers)
}

#[tauri::command]
pub async fn get_lost_numbers<'a>(ExistsNumbersRequest {signatory_authority, act_type, year}: ExistsNumbersRequest<'a>, db: State<'_, Arc<AppRepository<Repository>>>) -> Result<Vec<String>, Error>
{
    let numbers = searcher::Searcher::get_lost_numbers(signatory_authority, act_type, year).await?;
    let numbers = numbers.into_iter().map(|n|
    {
        let db_obj = db.repository.get_number()
    });
	logger::debug!("Найдено пропущеных номеров: {}", numbers.len());
    Ok(numbers)
}



// #[tauri::command]
// pub fn get_exists_parsers<'a>() -> Result<Vec<&'a str>, Error>
// {
//     let parsers = searcher::Searcher::get_exists_parsers()?;
//     Ok(parsers)
// }

pub fn searcher_plugin<R: Runtime>(app_state: Arc<AppState>, repository: Arc<AppRepository<Repository>>) -> TauriPlugin<R> 
{
    Builder::new("searcher")
      .invoke_handler(tauri::generate_handler![
        get_signatory_authorites,
        get_types,
        get_exists_numbers,
        get_lost_numbers
        ])
        .setup(|app_handle, _| 
        {
            app_handle.manage(repository);
            app_handle.manage(app_state);
            Ok(())
        })
      .build()
}