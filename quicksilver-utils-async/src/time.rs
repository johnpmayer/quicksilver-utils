//! # Time
//!
//! `time` just contains `sleep_ms` right not, but is the place
//! where I'd put something like a timer or something more
//! sophistocated like periodic scheduler with a "maximum fps"
//! governed periodic invocation

#[cfg(not(target_arch = "wasm32"))]
use crate::desktop::time;

#[cfg(all(target_arch = "wasm32", feature = "stdweb"))]
use crate::std_web::time;

#[cfg(all(target_arch = "wasm32", feature = "web-sys"))]
use crate::web_sys::time;

/// Block the async task until woken by the system after <ms> milliseconds
///
/// # Examples
///
/// ```
/// async fn tick_loop() {
///     loop {
///         sleep_ms(500).await;
///         do_something_periodically()
///     }
/// }
/// ```
pub async fn sleep_ms(ms: u32) {
    time::sleep_ms(ms).await
}
