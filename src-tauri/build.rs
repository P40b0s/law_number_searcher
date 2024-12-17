fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new()
        .plugin("searcher",
      tauri_build::InlinedPlugin::new().commands(&[
        "get_signatory_authorites",
        ]))
      ).unwrap();
    }
