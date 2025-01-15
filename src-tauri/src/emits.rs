use serde::Serialize;
use tauri::{AppHandle, Emitter, Runtime};

#[derive(Serialize, Clone)]
pub struct ProcessType
{
    percent: u32,
    description: String
}
pub struct Emits{}
impl Emits
{
    pub fn load_process_emit<R: Runtime>(handle: &AppHandle<R>, value: u32, description: &str)
    {
        handle.emit("load-process", ProcessType { percent: value, description: description.to_owned() }).unwrap();
    }
    pub fn load_base_process_emit<R: Runtime>(handle: &AppHandle<R>, value: u32, description: &str)
    {
        handle.emit("load-base-process", ProcessType { percent: value, description: description.to_owned() }).unwrap();
    }
    
}