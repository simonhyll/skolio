use std::sync::Mutex;

use log::trace;
use tauri::{
    AppHandle, CustomMenuItem, Manager, Monitor, PhysicalPosition, SystemTray, SystemTrayEvent,
    SystemTrayMenu, Window, WindowBuilder,
};

use crate::quizzy::QuizzyState;

const TRAY_SIZE: (u32, u32) = (400, 600);

pub(crate) fn init() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

fn add_window_events(app: &AppHandle, window: &Window) {
    let handle = app.app_handle();
    trace!("Creating window");
    window.once("ready", move |_| {
        trace!("Received ready");
        match handle.get_window("tray").unwrap().show() {
            Ok(_) => trace!("Showing tray menu"),
            Err(_) => trace!("failed to show tray menu"),
        }
    });
    let handle = app.app_handle();
    window.listen("settings", move |event| {
        trace!("Event received");
        let payload: QuizzyState = serde_json::from_str(event.payload().unwrap()).unwrap();
        let state = handle.state::<Mutex<QuizzyState>>();
        let mut lock = state.lock().unwrap();
        *lock = payload;
    });
}

fn get_window(app: &AppHandle) -> Window {
    let window = if app.get_window("tray").is_some() {
        app.get_window("tray").unwrap()
    } else {
        let w = WindowBuilder::new(app, "tray", tauri::WindowUrl::App("/".into()))
            .visible(false)
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
            .unwrap();
        add_window_events(app, &w);
        w
    };
    window
}

pub(crate) fn events(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let tray_window = get_window(app);
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
                let _ = tray_window.close();
            } else {
                let _ = tray_window.show();
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
    }
}
