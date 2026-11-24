mod commands;
mod date;
mod db;
mod logging;
mod models;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle().clone();
            logging::init(&handle)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            let db = db::init(&handle)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_items,
            commands::create_item,
            commands::update_item,
            commands::delete_item,
            commands::toggle_item_done,
            commands::list_tags,
            commands::create_tag,
            commands::update_tag,
            commands::export_items
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
