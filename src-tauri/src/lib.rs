use std::sync::Arc;
use db::{IRepository, Repository};
use plugins::searcher_plugin;
use state::AppState;
use tauri::Manager;
use tokio::runtime::Handle;
mod plugins;
mod db;
mod error;
mod state;
mod emits;
pub use error::Error;




// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() 
{
    let _ = logger::StructLogger::new_custom(logger::LevelFilter::Debug, Some(&[("html5ever", logger::LevelFilter::Info), ("selectors::matching", logger::LevelFilter::Info)]));
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    let repo = Arc::new(db::AppRepository {repository: Repository::new().await.expect("Возникли проблемы с инициализацией базы данных!")} );
    let app_state = Arc::new(AppState{});
    let clos_repo = Arc::clone(&repo);
    let clos_state = Arc::clone(&app_state);
    tauri::Builder::default()
    .setup(move |app| 
    {
        app.manage(clos_repo);
        app.manage(clos_state);
        // tokio::task::block_in_place(||
        // {
        //     Handle::current().block_on(async move 
        //     {
        //         let repo = Repository::new().await;
        //         if let Ok(r) = repo
        //         {
        //             let repo = Arc::new(db::AppRepository {repository: r});
        //             app.manage(repo);
        //             app.manage();
        //         }
        //     })
        // });
        //app.manage(repo);
        Ok(())

    })
    .plugin(tauri_plugin_shell::init())
    .plugin(searcher_plugin(app_state, repo))
    .plugin(tauri_plugin_process::init())
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
