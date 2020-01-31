extern crate log;
extern crate quicksilver;
extern crate url;

use log::{info, debug};
use quicksilver_utils_async::{task_context::TaskContext, time::sleep_ms, websocket::{WebSocket, WebSocketMessage}};

use quicksilver::{
    graphics::Graphics,
    lifecycle::{ElementState, Event as BlindsEvent, EventStream, Key, Window},
    Result,
};

use url::Url;

#[derive(Debug)]
enum CustomEvent {
    OnePingOnly,
    Ticked,
    EchoResponse(WebSocketMessage),
}

async fn tick_loop<'a>(task_context: TaskContext<'a, CustomEvent>) {
    loop {
        task_context.dispatch(CustomEvent::Ticked);
        sleep_ms(500).await
    }
}

async fn read_websocket_loop<'a>(task_context: TaskContext<'a, CustomEvent>, ws: WebSocket) {
    loop {
        let message: WebSocketMessage = ws.receive().await.unwrap();
        task_context.dispatch(CustomEvent::EchoResponse(message))
    }
}

pub async fn app(_window: Window, _gfx: Graphics, mut event_stream: EventStream) -> Result<()> {
    let mut task_context: TaskContext<CustomEvent> = TaskContext::new();

    task_context.spawn(tick_loop(task_context.clone()));

    let cloned_task_context = task_context.clone();
    task_context.spawn(async move {
        cloned_task_context.dispatch(CustomEvent::OnePingOnly);
    });


    let url_string = "ws://echo.websocket.org";
    // let url_string = "wss://echo.websocket.org"; // fails TLS?
    let ws = WebSocket::connect(&Url::parse(url_string).unwrap()).await.unwrap();
    task_context.spawn(read_websocket_loop(task_context.clone(), ws.clone()));

    'main: loop {
        task_context.run_until_stalled().await;

        for custom_event in task_context.drain().into_iter() {
            info!("CustomEvent: {:?}", custom_event)
        }

        while let Some(ev) = event_stream.next_event().await {
            if let BlindsEvent::KeyboardInput {
                key: Key::Escape, ..
            } = ev
            {
                break 'main;
            }

            if let BlindsEvent::KeyboardInput {
                key: Key::P,
                state: ElementState::Pressed,
            } = ev
            {
                let cloned_task_context = task_context.clone();
                task_context
                    .spawn(async move { cloned_task_context.dispatch(CustomEvent::OnePingOnly) });
            }

            if let BlindsEvent::KeyboardInput {
                key: Key::W,
                state: ElementState::Pressed,
            } = ev
            {
                ws.send("Hello free infrastructure").await.unwrap();
            }

            debug!("BlindsEvent: {:?}", ev);
        }
    }

    Ok(())
}
