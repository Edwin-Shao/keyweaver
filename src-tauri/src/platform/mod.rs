#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod fallback;

#[cfg(target_os = "macos")]
pub use macos::start_keyboard_hook;
#[cfg(target_os = "windows")]
pub use windows::start_keyboard_hook;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub use fallback::start_keyboard_hook;
