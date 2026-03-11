use tauri::{menu::{MenuBuilder, PredefinedMenuItem}, Manager, WebviewUrl, WebviewWindowBuilder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let menu = MenuBuilder::new(app)
                .text("home", "Home")
                .item(&PredefinedMenuItem::separator(app)?)
                .text("back", "Back")
                .text("forward", "Forward")
                .text("reload", "Reload")
                .item(&PredefinedMenuItem::separator(app)?)
                .text("new-window", "New window")
                .build()?;
            let main_window = app.get_webview_window("main").expect("main window not found");
            main_window.set_menu(menu)?;
            main_window.on_document_title_changed(move |title| {
                let _ = main_window.set_title(&format!("{} - Browser", title));
            });
            let handle = app.handle().clone();
            app.on_menu_event(move |app_handle, event| {
                let active_window = app_handle.get_webview_window("main").unwrap();
                match event.id().as_ref() {
                    "home" => { let _ = active_window.eval("window.location.href = 'http://localhost:1430/'"); }
                    "back" => { let _ = active_window.eval("window.history.back();"); }
                    "forward" => { let _ = active_window.eval("window.history.forward();"); }
                    "reload" => { let _ = active_window.eval("window.location.reload();"); }
                    "new-window" => {
                        let current_pos = active_window.outer_position().unwrap_or_default();
                        let current_size = active_window.inner_size().unwrap_or_default();
                        let label = format!("clone_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis());
                        let new_win = WebviewWindowBuilder::new(
                            app_handle,
                            label,
                            WebviewUrl::App("index.html".into())
                        )
                        .position((current_pos.x + 20) as f64, (current_pos.y + 20) as f64)
                        .inner_size(current_size.width as f64, current_size.height as f64)
                        .build()
                        .unwrap();
                        new_win.on_document_title_changed(move |title| {
                            let _ = new_win.set_title(&format!("{} - Browser", title));
                        });
                    }
                    _ => {}
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
