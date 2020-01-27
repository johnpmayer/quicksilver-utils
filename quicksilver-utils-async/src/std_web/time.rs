use std_web::web::wait;

pub async fn sleep_ms(ms: u32) {
    wait(ms).await
}
