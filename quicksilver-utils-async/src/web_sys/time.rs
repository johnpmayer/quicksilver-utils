use std::cell::RefCell;
use std::sync::Arc;
use std::task::{Poll, Waker};

use wasm_bindgen::prelude::{Closure, JsValue};
use wasm_bindgen::JsCast;

use futures_util::future::poll_fn;
use js_sys::Array;
use web_sys::console;
use web_sys::window;

struct ReadyWaker {
    ready: bool,
    waker: Option<Waker>,
}

pub async fn sleep_ms(ms: u32) {
    let window = window().expect("Get the window");

    let ready_waker = Arc::new(RefCell::new(ReadyWaker {
        ready: false,
        waker: None,
    }));

    let callback = {
        let ready_waker = ready_waker.clone();
        Closure::wrap(Box::new(move |_| {
            console::log_1(&JsValue::from_str("set_timeout callback!")); // TODO: debug logging
            let inner: &mut ReadyWaker = &mut *ready_waker.borrow_mut();
            inner.ready = true;
            if let Some(waker) = inner.waker.take() {
                waker.wake()
            }
        }) as Box<dyn FnMut(JsValue)>)
    };

    window
        .set_timeout_with_callback_and_timeout_and_arguments(
            callback.as_ref().unchecked_ref(),
            ms as i32,
            &Array::new(),
        )
        .expect("Invoke set_timeout");
    callback.forget();

    poll_fn({
        let ready_waker = ready_waker.clone();
        move |cx| {
            console::log_1(&JsValue::from_str("Polling"));
            let inner: &mut ReadyWaker = &mut *ready_waker.borrow_mut();
            if inner.ready {
                Poll::Ready(())
            } else {
                inner.waker.replace(cx.waker().clone());
                Poll::Pending
            }
        }
    })
    .await
}
