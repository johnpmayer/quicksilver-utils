use crate::request::RequestError;
use futures_util::future::poll_fn;
use log::debug;
use std::cell::RefCell;
use std::sync::Arc;
use std::task::Poll;
use std_web::{
    traits::*,
    unstable::TryInto,
    web::{
        event::{ProgressAbortEvent, ProgressLoadEvent},
        ArrayBuffer, TypedArray, XhrReadyState, XhrResponseType, XmlHttpRequest,
    },
    Reference,
};

struct XhrClosureInner {
    xhr: XmlHttpRequest,
    have_set_handlers: bool,
}

struct XhrClosure {
    inner: Arc<RefCell<XhrClosureInner>>,
}

pub async fn get_resource(url: &str) -> Result<String, RequestError> {
    debug!("stdweb get request {}", url);
    let xhr = XmlHttpRequest::new();
    xhr.open("GET", url)
        .map_err(|e| RequestError::NativeError(format!("Open: {}", e)))?;
    xhr.set_response_type(XhrResponseType::ArrayBuffer)
        .map_err(|e| RequestError::NativeError(format!("Set Response Type: {}", e)))?;
    xhr.send()
        .map_err(|e| RequestError::NativeError(format!("Send: {}", e)))?;

    let xhr_closure = XhrClosure {
        inner: Arc::new(RefCell::new(XhrClosureInner {
            xhr,
            have_set_handlers: false,
        })),
    };

    let result = poll_fn(move |ctx| {
        debug!("stdweb get request Polling");
        let inner: &mut XhrClosureInner = &mut *xhr_closure.inner.borrow_mut();

        if !inner.have_set_handlers {
            inner.have_set_handlers = true;
            let waker = ctx.waker().clone();
            inner
                .xhr
                .add_event_listener(move |_: ProgressLoadEvent| waker.wake_by_ref());
            let waker = ctx.waker().clone();
            inner
                .xhr
                .add_event_listener(move |_: ProgressAbortEvent| waker.wake_by_ref());
        }

        let status = inner.xhr.status();
        let ready_state = inner.xhr.ready_state();
        match (status / 100, ready_state) {
            (2, XhrReadyState::Done) => {
                let reference: Reference = inner
                    .xhr
                    .raw_response()
                    .try_into()
                    .expect("The response will always be a JS object");
                Poll::Ready(
                    reference
                        .downcast::<ArrayBuffer>()
                        .map(|arr| TypedArray::<u8>::from(arr).to_vec())
                        .ok_or_else(|| {
                            RequestError::NativeError("Failed to cast file into bytes".to_string())
                        }),
                )
            }
            (2, _) => Poll::Pending,
            (0, _) => Poll::Pending,
            _ => Poll::Ready(Err(RequestError::NativeError(
                "Non-200 status code returned".to_string(),
            ))),
        }
    })
    .await?;

    String::from_utf8(result).map_err(|e| RequestError::NativeError(format!("Invalid utf8 {}", e)))
}
