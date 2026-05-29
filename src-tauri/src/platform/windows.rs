use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use tauri::{AppHandle, Emitter};

static LISTENER_STARTED: AtomicBool = AtomicBool::new(false);

pub fn start_keyboard_hook(app: &AppHandle) {
    if LISTENER_STARTED.swap(true, Ordering::SeqCst) {
        return;
    }

    let app_handle = app.clone();
    thread::spawn(move || {
        // Windows keyboard hook will be implemented here
        // For now, key events come from the frontend via DOM keydown
        eprintln!("Windows keyboard hook not yet implemented");
        let _ = app_handle;
    });
}
