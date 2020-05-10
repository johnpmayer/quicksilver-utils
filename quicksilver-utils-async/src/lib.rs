extern crate futures_util;

#[cfg(all(not(target_arch = "wasm32"), feature = "stdweb"))]
compile_error!("stdweb can only be enabled for wasm32 targets");

#[cfg(all(not(target_arch = "wasm32"), feature = "web-sys"))]
compile_error!("websys can only be enabled for wasm32 targets");

#[cfg(all(feature = "stdweb", feature = "web-sys"))]
compile_error!("stdweb and web_sys may not both be enabled at once, you must pick one");

#[cfg(all(
    target_arch = "wasm32",
    not(feature = "stdweb"),
    not(feature = "web-sys")
))]
compile_error!("either stdweb or web-sys must be enabled for wasm32 targets");

#[cfg(not(target_arch = "wasm32"))]
mod desktop;

#[cfg(all(target_arch = "wasm32", feature = "stdweb"))]
mod std_web;

#[cfg(all(target_arch = "wasm32", feature = "web-sys"))]
mod web_sys;

pub mod request;
pub mod task_context;
pub mod time;
pub mod websocket;
