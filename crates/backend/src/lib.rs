use log::trace;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, WindowEvent};

pub(crate) mod quizzy;
pub(crate) mod tray;

#[tauri::command]
async fn get_quizzy_state(app: AppHandle) -> Result<quizzy::QuizzyState, ()> {
    trace!("Getting quizzy state");
    let state = app.state::<Mutex<quizzy::QuizzyState>>();
    let lock = state.lock().unwrap();
    Ok(*lock)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_quizzy_state])
        .plugin(tauri_plugin_window::init())
        .system_tray(tray::init())
        .on_system_tray_event(|app, event| tray::events(app, event))
        .on_window_event(|e| {
            if let WindowEvent::Resized(_) = e.event() {
                std::thread::sleep(std::time::Duration::from_nanos(1));
            }
        })
        .setup(|app| {
            let _ = tauri::async_runtime::spawn(crate::quizzy::run(app.handle()));
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
