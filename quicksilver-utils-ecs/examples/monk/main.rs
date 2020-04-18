
#[macro_use]
extern crate specs_derive;

use log::{debug, info};
use platter::load_file;
use quicksilver::{
    graphics::{Graphics, Image},
    lifecycle::{run, Event, EventCache, EventStream, Settings, Window},
    mint::Vector2,
    Result,
};
use quicksilver_utils_ecs::*;
use send_wrapper::SendWrapper;
use specs::prelude::*;
use std::collections::HashMap;

mod global;
mod interact;
mod room;

use room::Room;
use global::Global;
use interact::*;

#[derive(Eq, Hash, PartialEq)]
enum Animation {
    Right,
    Left, // could flip programatically
}

// Number of milliseconds to display the frame
fn frames() -> HashMap<Animation, Vec<u32>> {
    let mut dat = HashMap::new();
    dat.insert(
        Animation::Right,
        vec![
            1
        ],
    );
    dat.insert(Animation::Left, vec![1]);
    dat
}

fn main() {
    let mut settings = Settings::default();
    settings.size = Vector2::from([800.,600.]);
    run(settings, app)
}

async fn app(window: Window, gfx: Graphics, mut event_stream: EventStream) -> Result<()> {
    let monk_data = load_file("monk.png").await?;
    let monk_image: Image = Image::from_encoded_bytes(&gfx, &monk_data)?;

    let bed_data = load_file("bed.png").await?;
    let bed_image: Image = Image::from_encoded_bytes(&gfx, &bed_data)?;

    debug!("Loaded resources");

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
    world.register::<PlayerInteract>();
    world.register::<ObjectInteract>();

    debug!("Registered types");

    // Create the player
    let player_sprite = SpriteConfig {
        image: SendWrapper::new(monk_image),
        width: 32,
        height: 32,
        scale: 2.,
        animation: None,
    };

    let player_entity = world
        .create_entity()
        .with(Position { x: 0., y: 0. })
        .with(player_sprite)
        .with(PlayerInputFlag)
        .with(PlayerInteract { width: 64., height: 64.})
        .build();

    debug!("attempt to insert a Global");
    world.insert(Global::new(player_entity));
    
    let bed_sprite = SpriteConfig {
        image: SendWrapper::new(bed_image),
        width: 32,
        height: 32,
        scale: 3.,
        animation: None,
    };

    world
        .create_entity()
        .with(Position { x: 100., y: 100.})
        .with(bed_sprite)
        .with(ObjectInteract { object: Objects::Bed, width: 32. * 3., height: 32.*3.,})
        .build();

    debug!("Created world, components, and entities");

    let mut sprite_system = RenderSprites;
    let mut move_system = WasdMovement {
        event_cache: EventCache::new(),
    };
    let mut detect_interaction_range_system = DetectInteractionRange;

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
        detect_interaction_range_system.run_now(&world);
    }
}
