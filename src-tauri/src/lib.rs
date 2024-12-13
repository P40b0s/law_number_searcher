use std::sync::Arc;

use db::Repository;
use tauri::Manager;
use tokio::runtime::Handle;

mod db;
mod error;





// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() 
{
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
    .setup(async move |app| 
    {
        let repo = Arc::new(db::AppRepository {repository: Repository::new()});
        let _ = repo.create().await;
        // tokio::task::block_in_place(||
        // {
        //     Handle::current().block_on(async move 
        //     {
                
        //     })
        // });
        app.manage(repo);
        Ok(())

    })
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
