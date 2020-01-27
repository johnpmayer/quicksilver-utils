extern crate log;
extern crate quicksilver;

use log::{info, trace};
use quicksilver_utils_async::{task_context::TaskContext, time::sleep_ms};

use quicksilver::{
    graphics::Graphics,
    lifecycle::{ElementState, Event as BlindsEvent, EventStream, Key, Window},
    Result,
};

#[derive(Debug)]
enum CustomEvent {
    OnePingOnly,
    Ticked,
}

async fn tick_loop<'a>(task_context: TaskContext<'a, CustomEvent>) {
    loop {
        task_context.dispatch(CustomEvent::Ticked);
        sleep_ms(500).await
    }
}

pub async fn app(_window: Window, _gfx: Graphics, mut event_stream: EventStream) -> Result<()> {
    let mut task_context: TaskContext<CustomEvent> = TaskContext::new();

    task_context.spawn(tick_loop(task_context.clone()));

    let cloned_task_context = task_context.clone();
    task_context.spawn(async move {
        cloned_task_context.dispatch(CustomEvent::OnePingOnly);
    });

    'main: loop {
        trace!("Main loop wrapped around");
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

            info!("BlindsEvent: {:?}", ev);
        }
    }

    Ok(())
}
