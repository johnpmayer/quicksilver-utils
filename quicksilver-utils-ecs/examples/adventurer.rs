// Sprite Sheet Info

// PNG image - width: 416, height: 512
// Sprites - 16 rows, 13 columns
// Each sprite - 32x32 pixels

use log::{debug, info};
use platter::load_file;
use quicksilver::{
    graphics::{Graphics, Image, PixelFormat},
    lifecycle::{run, EventStream, Settings, Window},
    Result,
};
use quicksilver_utils_ecs::*;
use send_wrapper::SendWrapper;
use specs::prelude::*;
use std::collections::HashMap;

enum Orientation {
    Left,
    Right,
}

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

fn frames() -> HashMap<Animation, Vec<u32>> {
    let mut dat = HashMap::new();
    dat.insert(Animation::Idle, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
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

    world.insert(SharedRenderingContext {
        gfx: SendWrapper::new(gfx),
        window: SendWrapper::new(window),
    });

    world.register::<Position>();
    world.register::<Sprite>();
    world.register::<PlayerInputFlag>();

    // Create the player
    world
        .create_entity()
        .with(Position { x: 0., y: 0. })
        .with(Sprite {
            image: SendWrapper::new(sprite_image),
        })
        .with(PlayerInputFlag)
        .build();

    debug!("Created world, components, and entities");

    // let mut dispatcher = DispatcherBuilder::new()
    //     .with(RenderSprites, "render_sprites", &[])
    //     .build();

    let mut sprite_system = RenderSprites;

    debug!("Entering main loop");

    'main: loop {
        info!("In the loop");

        while let Some(ev) = event_stream.next_event().await {
            debug!("Quicksilver event: {:?}", ev)
        }

        sprite_system.run_now(&world);

        // dispatcher.dispatch_seq(&world) // otherwise SendWrapper will explode!
    }

    Ok(())
}
