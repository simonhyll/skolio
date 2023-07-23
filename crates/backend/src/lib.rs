use tauri::{
    CustomMenuItem, Monitor, PhysicalPosition, SystemTray, SystemTrayEvent, SystemTrayMenu,
};
use tauri::{Manager, WindowEvent};

pub(crate) mod quizzer;
pub(crate) mod tasker;

const TRAY_SIZE: (u32, u32) = (400, 600);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: pos,
                size: _,
                ..
            } => {
                let tray_window = if app.get_window("tray").is_some() {
                    app.get_window("tray").unwrap()
                } else {
                    tauri::WindowBuilder::new(app, "tray", tauri::WindowUrl::App("/".into()))
                        .visible(false)
                        .position(pos.x, pos.y)
                        .inner_size(TRAY_SIZE.0 as f64, TRAY_SIZE.1 as f64)
                        .always_on_top(true)
                        .decorations(false)
                        .transparent(true)
                        .focused(true)
                        .resizable(false)
                        .closable(false)
                        .skip_taskbar(true)
                        .shadow(true)
                        .incognito(true)
                        .disable_file_drop_handler()
                        .build()
                        .unwrap()
                };
                let monitor: Monitor = tray_window
                    .current_monitor()
                    .expect("failed to get monitor")
                    .unwrap();
                let monitor_size = monitor.size();
                let _ = tray_window.set_position(PhysicalPosition {
                    x: monitor_size.width - TRAY_SIZE.0,
                    y: monitor_size.height - TRAY_SIZE.1 - 48,
                });
                if tray_window.is_visible().unwrap() {
                    let _ = tray_window.hide();
                } else {
                    let _ = tray_window.show();
                    let _ = tray_window.set_focus();
                }
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {}
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {}
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .on_window_event(|e| {
            if let WindowEvent::Resized(_) = e.event() {
                std::thread::sleep(std::time::Duration::from_nanos(1));
            }
        })
        .setup(|app| {
            let _ = tauri::async_runtime::spawn(crate::quizzer::run(app.handle()));
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
