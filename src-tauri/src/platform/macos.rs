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
        let _ = app_handle;
        // Keyboard events are captured via DOM keydown events in the frontend,
        // and forwarded to the Rust backend via process_keystroke.
        // This hook is reserved for future global-hotkey capture.
        eprintln!("Keyboard hook: using DOM fallback on macOS");
    });
}
