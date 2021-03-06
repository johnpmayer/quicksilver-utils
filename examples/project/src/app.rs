extern crate log;
extern crate quicksilver;
extern crate url;

use log::{debug, info};
use quicksilver_utils_async::{
    // request::get_resource,
    task_context::TaskContext,
    time::sleep_ms,
    websocket::{WebSocket, WebSocketMessage},
};

use quicksilver::{
    graphics::Graphics,
    input::{Event as BlindsEvent, Input, Key},
    Result, Window,
};

use url::Url;

#[derive(Debug)]
enum CustomEvent {
    OnePingOnly,
    Ticked,
    EchoResponse(WebSocketMessage),
    // Resource(String),
}

async fn tick_loop(task_context: TaskContext<'_, CustomEvent>) {
    loop {
        task_context.dispatch(CustomEvent::Ticked);
        sleep_ms(500).await
    }
}

async fn read_websocket_loop(task_context: TaskContext<'_, CustomEvent>, ws: WebSocket) {
    loop {
        let message: WebSocketMessage = ws.receive().await.unwrap();
        task_context.dispatch(CustomEvent::EchoResponse(message))
    }
}

pub async fn app(_window: Window, _gfx: Graphics, mut input: Input) -> Result<()> {
    let mut task_context: TaskContext<CustomEvent> = TaskContext::new();

    task_context.spawn(tick_loop(task_context.clone()));

    let cloned_task_context = task_context.clone();
    task_context.spawn(async move {
        cloned_task_context.dispatch(CustomEvent::OnePingOnly);
    });

    let url_string = "ws://echo.websocket.org";
    // let url_string = "wss://echo.websocket.org"; // fails TLS on desktop?
    let ws = WebSocket::connect(&Url::parse(url_string).unwrap())
        .await
        .unwrap();
    task_context.spawn(read_websocket_loop(task_context.clone(), ws.clone()));

    'main: loop {
        task_context.run_until_stalled().await;

        for custom_event in task_context.drain().into_iter() {
            info!("CustomEvent: {:?}", custom_event)
        }

        while let Some(ev) = input.next_event().await {
            if let BlindsEvent::KeyboardInput(key_event) = &ev {
                if key_event.key() == Key::Escape && key_event.is_down() {
                    break 'main;
                }

                if key_event.key() == Key::P && key_event.is_down() {
                    let cloned_task_context = task_context.clone();
                    task_context.spawn(async move {
                        cloned_task_context.dispatch(CustomEvent::OnePingOnly)
                    });
                }

                if key_event.key() == Key::W && key_event.is_down() {
                    let msg = WebSocketMessage::String("Hello free infrastructure".to_string());
                    ws.send(&msg).await.unwrap();
                }

                // if key_event.key() == Key::R && key_event.is_down() {
                //     let cloned_task_context = task_context.clone();
                //     task_context.spawn(async move {
                //         let response = get_resource("https://jsonplaceholder.typicode.com/todos/1")
                //             .await
                //             .expect("HTTP GET success");
                //         cloned_task_context.dispatch(CustomEvent::Resource(response))
                //     });
                // }
            }

            debug!("BlindsEvent: {:?}", ev);
        }
    }

    Ok(())
}
