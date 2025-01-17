use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{AppHandle, Manager, Runtime, State};

use crate::db::{AppRepository, IRepository, NumberDBO, Repository};
use crate::emits::Emits;
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
///status - `0` неопубликован `1` проверен `2` опубликован на другом сайте
pub struct Number
{
    signatory_authority: uuid::Uuid,
    type_id: uuid::Uuid,
    year: u32,
    number: String,
    note: Option<String>,
    status: i8
}

impl Into<Number> for NumberDBO
{
    fn into(self) -> Number 
    {
        Number
        {
            signatory_authority: self.signatory_authority,
            type_id: self.type_id,
            year: self.year,
            number: self.number,
            note: self.note,
            status: self.status
        }
    }
}
impl Into<NumberDBO> for &mut Number
{
    fn into(self) -> NumberDBO
    {
        NumberDBO
        {
            signatory_authority: self.signatory_authority,
            type_id: self.type_id,
            year: self.year,
            number: self.number.clone(),
            note: self.note.clone(),
            status: self.status
        }
    }
}

impl Into<NumberDBO> for Number
{
    fn into(self) -> NumberDBO
    {
        NumberDBO
        {
            signatory_authority: self.signatory_authority,
            type_id: self.type_id,
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
pub async fn get_types<R: Runtime>(app: AppHandle<R>, payload: &str) -> Result<Vec<Dictionary>, Error>
{
    let (sender, mut receiver) =  tokio::sync::mpsc::channel::<u32>(1);
    let join_me = tokio::spawn(
        async move 
        {
            while let Some(p) = receiver.recv().await 
            {
                Emits::load_process_emit(&app, p, "Поиск видов документов");
                logger::info!("текущий процент выполнения: {}%", p);
            }
        });
    let mut doc_types = searcher::Searcher::get_types(payload, Some(sender)).await?;
    doc_types.sort_by(|a, b| a.name.cmp(&b.name));
	logger::debug!("Найдено типов документов: {}", doc_types.len());
    let _ = join_me.await;
    Ok(doc_types)
}

#[tauri::command]
pub async fn get_exists_numbers<'a, R: Runtime>(app: AppHandle<R>, ExistsNumbersRequest {signatory_authority, act_type, year}: ExistsNumbersRequest<'a>) -> Result<Vec<String>, Error>
{
    let (sender, mut receiver) =  tokio::sync::mpsc::channel::<u32>(1);
    let join_me = tokio::spawn(
        async move 
        {
            while let Some(p) = receiver.recv().await 
            {
                Emits::load_process_emit(&app, p, "Поиск номеров");
                logger::info!("текущий процент выполнения: {}%", p);
            }
        });
    let numbers = searcher::Searcher::get_exists_numbers(signatory_authority, act_type, year, Some(sender)).await?;
    //doc_types.sort_by(|a, b| a.name.cmp(&b.name));
	logger::debug!("Найдено номеров документов: {}", numbers.len());
    let _ = join_me.await;
    Ok(numbers)
}

#[tauri::command]
pub async fn get_lost_numbers<'a, R: Runtime>(app: AppHandle<R>, ExistsNumbersRequest {signatory_authority, act_type, year}: ExistsNumbersRequest<'a>, db: State<'_, Arc<AppRepository<Repository>>>) -> Result<Vec<Number>, Error>
{
    let (sender, mut receiver) =  tokio::sync::mpsc::channel::<u32>(1);
    let app_cloned = app.clone();
    let join_me = tokio::spawn(
        async move 
        {
            while let Some(p) = receiver.recv().await 
            {
                Emits::load_process_emit(&app_cloned, p, "Поиск номеров");
                logger::info!("текущий процент выполнения: {}%", p);
            }
        });
    let numbers = searcher::Searcher::get_lost_numbers(signatory_authority, act_type, year, Some(sender)).await?;
    let mut mod_numbers: Vec<Number> = Vec::with_capacity(numbers.len());
    let one_percent_value = numbers.len() as f64 / 100.0;
    let mut current_percent = 0.0;
    for n in &numbers
    {
        let db_number = db.repository.get_number(signatory_authority, act_type, year, n).await?;
        if let Some(db_obj) = db_number
        {
            //значит такой объект уже есть
            mod_numbers.push(db_obj.into());
        }
        else 
        {
            mod_numbers.push(Number 
            { 
                number: n.to_owned(),
                note: None,
                signatory_authority: uuid::Uuid::parse_str(signatory_authority).unwrap(),
                type_id: uuid::Uuid::parse_str(act_type).unwrap(),
                year,
                status: 0
            });
        }
        current_percent += 1.0;
        let percent = (current_percent / one_percent_value).ceil() as u32;
        if percent % 5 == 0
        {
            Emits::load_base_process_emit(&app, percent, "Поиск в базе данных");
        }
    }
	logger::debug!("Найдено пропущеных номеров: {}", mod_numbers.len());
    let _ = join_me.await;
    Ok(mod_numbers)
}

#[tauri::command]
pub fn get_alternative_publ_site<R: Runtime>(app: AppHandle<R>, payload: &str) -> Option<&str>
{
    searcher::Searcher::get_alternative_publ_site(payload)
}
#[tauri::command]
pub async fn save_number<R: Runtime>(payload: Number, app: AppHandle<R>) -> Result<(), Error>
{
    let db = app.state::<Arc<AppRepository<Repository>>>();
    let _ = db.repository.save_number(payload.into()).await;
    Ok(())
}

#[tauri::command]
pub async fn check_alternative_publ_info<R: Runtime>(payload: Vec<Number>, app: AppHandle<R>) -> Result<Vec<Number>, Error>
{
    if let Some(first) = payload.first()
    {
        let db = app.state::<Arc<AppRepository<Repository>>>();
        let new_numbers = searcher::Searcher::check_alternative_publ_site_info(
            &first.signatory_authority.to_string(),
            &first.type_id.to_string(),
            first.year).await?;
        let source: Option<(uuid::Uuid, uuid::Uuid, u32)> = payload
            .first()
            .and_then(|f| Some((f.signatory_authority.clone(), f.type_id.clone(), f.year)));
        let mut old_numbers = payload;
        for n in &new_numbers
        {
            if let Some(on) = old_numbers.iter_mut().find(|f| &f.number == n)
            {
                if on.status == 0
                {
                    on.status = 2;
                    let _ = db.repository.save_number(on.into()).await;
                }
            }
            else 
            {
                if let Some(s) = source
                {
                    old_numbers.push(Number 
                        { 
                            signatory_authority: s.0,
                            type_id: s.1,
                            year: s.2,
                            number: n.to_owned(),
                            note: None,
                            status: 2 
                        });    
                } 
            }
        }
        // for n in old_numbers.iter_mut()
        // {
        //     if new_numbers.contains(&n.number)
        //     {
        //         if n.status == 0
        //         {
        //             n.status = 2;
        //             let _ = db.repository.save_number(n.into()).await;
        //         }
        //     }
        //     //если на сайте есть такой номер а у нас нет
        //     else 
        //     {
               
        //     }
        // }
        return Ok(old_numbers);
    }
    logger::info!("В запрос не передано ни одного номера, возват изначальной коллекции");
    Ok(payload)
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
        get_lost_numbers,
        get_alternative_publ_site,
        save_number,
        check_alternative_publ_info
        ])
        .setup(|app_handle, _| 
        {
            app_handle.manage(repository);
            app_handle.manage(app_state);
            Ok(())
        })
      .build()
}