use async_std::task::sleep;
use std::time::Duration;

pub(crate) async fn sleep_ms(ms: u32) {
    sleep(Duration::from_millis(ms as u64)).await
}
