pub mod logger;
pub mod math;
mod graphics;
mod resources;

pub use logger::{debug, error, info, trace, warn};

pub trait BaseApp {}

#[cfg(target_arch = "wasm32")]
pub fn date_now() -> u64 {
    js_sys::Date::now() as u64
}

#[cfg(not(target_arch = "wasm32"))]
pub fn date_now() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
