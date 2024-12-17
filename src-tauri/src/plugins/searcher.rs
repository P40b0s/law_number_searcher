use std::sync::Arc;

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime, State};

use crate::db::{AppRepository, Repository};
use crate::state::AppState;
use crate::Error;
use searcher::{SignatoryAuthority, SearcherError};


#[tauri::command]
pub async fn get_signatory_authorites() -> Result<Vec<SignatoryAuthority>, Error>
{
    let organs = searcher::Searcher::get_signatory_authorites().await?;
    Ok(organs)
}

pub fn searcher_plugin<R: Runtime>(app_state: Arc<AppState>, repository: Arc<AppRepository<Repository>>) -> TauriPlugin<R> 
{
    Builder::new("searcher")
      .invoke_handler(tauri::generate_handler![
        get_signatory_authorites
        ])
        .setup(|app_handle, _| 
        {
            app_handle.manage(repository);
            app_handle.manage(app_state);
            Ok(())
        })
      .build()
}