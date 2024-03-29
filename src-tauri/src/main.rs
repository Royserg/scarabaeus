// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, MenuItemKind},
    tray::{ClickType, TrayIconBuilder},
    Manager,
};

#[tauri::command]
fn today() {
    println!("today");
}

fn main() {
    tauri::Builder::default()
        // .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // -- Shortcut --
            #[cfg(desktop)]
            {
                use tauri::Manager;
                use tauri_plugin_global_shortcut::{Code, Modifiers};

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_shortcut("ctrl+u")?
                        .with_handler(|app, shortcut| {
                            if shortcut.matches(Modifiers::CONTROL, Code::KeyU) {
                                let app = app.app_handle();
                                if let Some(webview_window) = app.get_webview_window("main") {
                                    let _ = webview_window.show();
                                    let _ = webview_window.set_focus();
                                }
                            }
                        })
                        .build(),
                )?;
            }

            // -- Tray --
            let open = MenuItemBuilder::with_id("open", "Open Scarabaeus").build(app)?;
            // let settings = MenuItemBuilder::with_id("settings", "Settings").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;

            let menu = MenuBuilder::new(app)
                .items(&[&open])
                .separator()
                .item(&quit)
                .build()?;

            TrayIconBuilder::new()
                .icon_as_template(false)
                .icon(app.default_window_icon().cloned().unwrap())
                // .icon(Icon include_bytes!("../icons/icon.png").to_vec())
                // .icon(tauri::menu::MenuItemKind(
                //     include_bytes!("../icons/icon.png").to_vec(),
                // ))
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "open" => {
                        let app = app.app_handle();
                        if let Some(webview_window) = app.get_webview_window("main") {
                            let _ = webview_window.show();
                            let _ = webview_window.set_focus();
                        }
                    }
                    "settings" => {
                        println!("settings clicked");
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => (),
                })
                // .on_tray_icon_event(|tray, event| {
                // if event.click_type == ClickType::Left {
                //     let app = tray.app_handle();
                //     if let Some(webview_window) = app.get_webview_window("main") {
                //         let _ = webview_window.show();
                //         let _ = webview_window.set_focus();
                //     }
                // }
                // })
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![today])
        .run(tauri::generate_context!())
        .expect("error while running Scarab application");
}
