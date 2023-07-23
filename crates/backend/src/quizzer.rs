use std::sync::{Arc, Mutex};

use log::{info, trace};
use tauri::{AppHandle, Manager};

#[derive(Debug)]
struct QuizzyTimerState {
    counter: u64,
    decrement: f64,
    increment: f64,
}

#[derive(Debug)]
struct QuizzyState {
    timer: QuizzyTimerState,
}

pub(crate) async fn run(app: AppHandle) {
    app.manage(Mutex::new(QuizzyState {
        timer: QuizzyTimerState {
            counter: 0,
            decrement: 0.90,
            increment: 60.0,
        },
    }));
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let s = Arc::new(Mutex::new(tx));
    loop {
        let sender = Arc::clone(&s);
        let quizzer_window = if app.get_window("quizzy").is_some() {
            app.get_window("quizzy").unwrap()
        } else {
            info!("Opening a quiz");
            tauri::WindowBuilder::new(&app, "quizzy", tauri::WindowUrl::App("/quizzy".into()))
                .visible(true)
                .maximized(true)
                .fullscreen(true)
                .always_on_top(true)
                .decorations(false)
                .transparent(true)
                .focused(true)
                .resizable(false)
                .closable(false)
                .skip_taskbar(true)
                .disable_file_drop_handler()
                .incognito(true)
                .title("")
                .build()
                .unwrap()
        };
        let handle = app.app_handle();
        let fail_handler = quizzer_window.once("fail", move |_| {
            let state = handle.state::<Mutex<QuizzyState>>();
            let mut lock = state.lock().unwrap();
            let mut decrease = lock.timer.increment * 2.0;
            if lock.timer.counter > 1 {
                while decrease > lock.timer.counter as f64 {
                    decrease = decrease * 0.5;
                }
                trace!("Decreasing by: {}", decrease as u64);
                lock.timer.counter -= decrease as u64;
            }
        });
        let handle = app.app_handle();
        quizzer_window.on_window_event(move |event| match event {
            tauri::WindowEvent::Resized(_) => {}
            tauri::WindowEvent::Moved(_) => {}
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
            }
            tauri::WindowEvent::Destroyed => {
                {
                    let state = handle.state::<Mutex<QuizzyState>>();
                    let mut lock = state.lock().unwrap();
                    trace!("Decreasing to {}%", lock.timer.decrement);
                    lock.timer.counter = (lock.timer.counter as f64 * lock.timer.decrement) as u64;
                    trace!("Increasing by: {}", lock.timer.increment as u64);
                    lock.timer.counter += lock.timer.increment as u64;
                }
                let l = sender.lock().unwrap();
                futures::executor::block_on(async move {
                    let _ = l.send(()).await;
                });
            }
            tauri::WindowEvent::Focused(_) => {}
            tauri::WindowEvent::ScaleFactorChanged { .. } => {}
            tauri::WindowEvent::FileDrop(_) => {}
            tauri::WindowEvent::ThemeChanged(_) => {}
            _ => {}
        });
        trace!("Recving");
        let _ = rx.recv().await.unwrap();
        let state = app.state::<Mutex<QuizzyState>>();
        let sleep_time = state.lock().unwrap().timer.counter.clone();
        trace!("Sleeping for {} seconds", sleep_time);
        quizzer_window.unlisten(fail_handler);
        tokio::time::sleep(tokio::time::Duration::from_secs(sleep_time)).await;
    }
}
