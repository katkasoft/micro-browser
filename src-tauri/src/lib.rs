use tauri::menu::{MenuBuilder, PredefinedMenuItem};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let menu = MenuBuilder::new(app)
                .text("home", "Home")
                .separator()
                .item(&PredefinedMenuItem::separator(app)?)
                .text("back", "Back")
                .text("forward", "Forward")
                .text("reload", "Reload")
                .build()?;

            let window = app.get_webview_window("main")
                .expect("main window not found");
            window.set_menu(menu)?;

            let window_clone = window.clone();

            app.on_menu_event(move |_app, event| {
                match event.id().as_ref() {
                    "home" => {
                        window_clone.eval("window.location.href = 'http://localhost:1430/'").unwrap();
                    }
                    "back" => {
                        window_clone.eval("window.history.back();").unwrap();
                    }
                    "forward" => {
                        window_clone.eval("window.history.forward();").unwrap();
                    }
                    "reload" => {
                        window_clone.eval("window.location.reload();").unwrap();
                    }
                    _ => {}
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}