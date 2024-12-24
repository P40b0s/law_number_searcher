use tauri::{AppHandle, Emitter, Runtime};

pub struct Emits
{

}
impl Emits
{
    pub fn load_process_emit<R: Runtime>(handle: &AppHandle<R>, value: u32)
    {
        handle.emit("load-process", value).unwrap();
    }
}