
#[macro_use]
extern crate specs_derive;

use log::{debug, trace};
use platter::load_file;
use quicksilver::{
    graphics::{Graphics, Image, VectorFont, FontRenderer},
    lifecycle::{run, Event, EventCache, EventStream, Settings, Window},
    mint::Vector2,
    Result,
};
use quicksilver_utils_ecs::*;
use send_wrapper::SendWrapper;
use specs::prelude::*;
use std::collections::HashMap;

mod background;
mod global;
mod hud;
mod interact;
mod room;

use background::BackgroundRender;
use room::Room;
use global::Global;
use interact::*;
use hud::HudRender;

#[derive(Eq, Hash, PartialEq)]
enum Animation {
    Right,
    Left, // could flip programatically
}

// // Number of milliseconds to display the frame
// fn frames() -> HashMap<Animation, Vec<u32>> {
//     let mut dat = HashMap::new();
//     dat.insert(
//         Animation::Right,
//         vec![
//             1
//         ],
//     );
//     dat.insert(Animation::Left, vec![1]);
//     dat
// }

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

    let font_data = VectorFont::load("Kingthings-Calligraphica/Kingthings_Calligraphica_2.ttf").await?;
    let font: FontRenderer = font_data.to_renderer(&gfx, 36.0)?;

    // Note: this is a camera photo of a drawing
    // I had to downsample it to 800x600 pixels, otherwise it was too large for quicksilver to handle
    let bedroom_data = load_file("bedroom.png").await?;
    let bedroom_image: Image = Image::from_encoded_bytes(&gfx, &bedroom_data)?;

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
    world.insert(Global::new(player_entity, font, bedroom_image));
    
    let bed_sprite = SpriteConfig {
        image: SendWrapper::new(bed_image),
        width: 32,
        height: 32,
        scale: 3.,
        animation: None,
    };

    world
        .create_entity()
        .with(Position { x: 600., y: 400.})
        .with(bed_sprite)
        .with(ObjectInteract { object: Objects::Bed, width: 32. * 3., height: 32.*3.,})
        .build();

    world.create_entity().with(Position{x: 50., y: 140.}).with(ObjectInteract{object: Objects::BedroomExit, width: 100., height: 220.}).build();

    debug!("Created world, components, and entities");

    let mut sprite_system = RenderSprites;
    let mut move_system = WasdMovement {
        event_cache: EventCache::new(),
    };
    let mut interaction_system = InteractionSystem {
        event_cache: EventCache::new(),
    };
    let mut hud_render_system = HudRender; // we could inject the font here instead of the Global resource...
    let mut background_render_system = BackgroundRender;

    debug!("Entering main loop");

    'main: loop {
        let now: f64 = instant::now();
        *world.write_resource::<TimeContext>() = TimeContext { now };

        trace!("In the loop");

        let mut buffer: Vec<Event> = Vec::new();
        while let Some(ev) = event_stream.next_event().await {
            trace!("Quicksilver event: {:?}", ev);
            buffer.push(ev)
        }
        (*world.write_resource::<EventBuffer>()).events = buffer;

        background_render_system.run_now(&world);
        sprite_system.run_now(&world);
        move_system.run_now(&world);
        interaction_system.run_now(&world);
        hud_render_system.run_now(&world);

        let mut ctx = world.get_mut::<RenderContext>().expect("has render context");
        ctx.gfx.present(&ctx.window).expect("present"); // FIXME probably a separate system?
    }
}
