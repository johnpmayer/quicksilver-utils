// Sprite Sheet Info

// PNG image - width: 416, height: 512
// Sprites - 16 rows, 13 columns
// Each sprite - 32x32 pixels

use log::{debug, info};
use platter::load_file;
use quicksilver::{
    graphics::{Graphics, Image},
    lifecycle::{run, Event, EventCache, EventStream, Settings, Window},
    Result,
};
use quicksilver_utils_ecs::*;
use send_wrapper::SendWrapper;
use specs::prelude::*;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
enum Animation {
    Idle,
    Run,
    SlashUp,
    SlashDown,
    SlashForward,
    Jump,
    Hit,
    Faint,
}

// Number of milliseconds to display the frame
fn frames() -> HashMap<Animation, Vec<u32>> {
    let mut dat = HashMap::new();
    dat.insert(
        Animation::Idle,
        vec![
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
    );
    dat.insert(Animation::Run, vec![1, 1, 1, 1, 1, 1, 1, 1]);
    dat.insert(Animation::SlashUp, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    dat.insert(Animation::SlashDown, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    dat.insert(Animation::SlashForward, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    dat.insert(Animation::Jump, vec![1, 1, 1, 1, 1, 1]);
    dat.insert(Animation::Hit, vec![1, 1, 1, 1]);
    dat.insert(Animation::Faint, vec![1, 1, 1, 1, 1, 1, 1]);
    dat
}

fn main() {
    let settings = Settings::default();
    run(settings, app)
}

async fn app(window: Window, gfx: Graphics, mut event_stream: EventStream) -> Result<()> {
    let sprite_sheet_data = load_file("sprite_sheet.png").await?;
    // let sprite_image = Image::from_raw(&gfx, Some(&sprite_sheet), 416, 512, PixelFormat::RGBA)?;
    let sprite_image: Image = Image::from_encoded_bytes(&gfx, &sprite_sheet_data)?;

    debug!("Got the image");

    let mut world = World::new();

    world.insert(RenderContext {
        gfx: SendWrapper::new(gfx),
        window: SendWrapper::new(window),
    });

    let now = instant::now();

    world.insert(TimeContext { now });
    world.insert(EventBuffer { events: Vec::new() });

    world.register::<Position>();
    world.register::<SpriteConfig>();
    world.register::<PlayerInputFlag>();

    // Create the player
    let player_sprite = SpriteConfig {
        image: SendWrapper::new(sprite_image),
        width: 32,
        height: 32,
        scale: 2.,
        animation: Some(AnimationConfig {
            loop_start_time: now,
            frames: frames()
                .get(&Animation::Idle)
                .expect("frames for idle animation")
                .clone(),
        }),
    };

    world
        .create_entity()
        .with(Position { x: 0., y: 0. })
        .with(player_sprite)
        .with(PlayerInputFlag)
        .build();

    debug!("Created world, components, and entities");

    let mut sprite_system = RenderSprites;
    let mut move_system = WasdMovement {
        event_cache: EventCache::new(),
    };

    debug!("Entering main loop");

    'main: loop {
        let now: f64 = instant::now();
        *world.write_resource::<TimeContext>() = TimeContext { now };

        info!("In the loop");

        let mut buffer: Vec<Event> = Vec::new();
        while let Some(ev) = event_stream.next_event().await {
            debug!("Quicksilver event: {:?}", ev);
            buffer.push(ev)
        }
        (*world.write_resource::<EventBuffer>()).events = buffer;

        sprite_system.run_now(&world);
        move_system.run_now(&world);
    }
}
