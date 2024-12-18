use std::sync::Arc;

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime, State};

use crate::db::{AppRepository, Repository};
use crate::state::AppState;
use crate::Error;
use searcher::{DocumentType, SearcherError, SignatoryAuthority};


#[tauri::command]
pub async fn get_signatory_authorites() -> Result<Vec<SignatoryAuthority>, Error>
{
    let mut organs = searcher::Searcher::get_signatory_authorites().await?;
    organs.sort_by(|a, b| a.name.cmp(&b.name));
	logger::debug!("Получено органов: {}", organs.len());
    Ok(organs)
}

#[tauri::command]
pub async fn get_types(payload: &str) -> Result<Vec<DocumentType>, Error>
{
    let mut doc_types = searcher::Searcher::get_types(payload).await?;
    doc_types.sort_by(|a, b| a.name.cmp(&b.name));
	logger::debug!("Найдено типов документов: {}", doc_types.len());
    Ok(doc_types)
}

#[tauri::command]
pub fn get_exists_parsers<'a>() -> Result<Vec<&'a str>, Error>
{
    let parsers = searcher::Searcher::get_exists_parsers()?;
    Ok(parsers)
}

pub fn searcher_plugin<R: Runtime>(app_state: Arc<AppState>, repository: Arc<AppRepository<Repository>>) -> TauriPlugin<R> 
{
    Builder::new("searcher")
      .invoke_handler(tauri::generate_handler![
        get_signatory_authorites,
        get_exists_parsers,
        get_types
        ])
        .setup(|app_handle, _| 
        {
            app_handle.manage(repository);
            app_handle.manage(app_state);
            Ok(())
        })
      .build()
}