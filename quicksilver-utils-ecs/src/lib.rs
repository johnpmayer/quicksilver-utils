extern crate quicksilver;
extern crate send_wrapper;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use quicksilver::{graphics::Graphics, lifecycle::Event};
use send_wrapper::SendWrapper;
use specs::{prelude::*, Component, System, Write};

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Sprite {
    dummy: bool,
}

#[derive(Component)]
struct PlayerInputFlag;

struct SharedRenderingContext {
    gfx: SendWrapper<Graphics>,
}

impl Default for SharedRenderingContext {
    fn default() -> Self {
        panic!("must be injected...")
    }
}

struct RenderSprites;

impl<'a> System<'a> for RenderSprites {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Sprite>,
        Write<'a, SharedRenderingContext>,
    );
    fn run(&mut self, (position_storage, sprite_storage, mut ctx_resource): Self::SystemData) {
        let gfx: &mut Graphics = &mut ctx_resource.gfx;

        for (position, sprite) in (&position_storage, &sprite_storage).join() {
            //
        }
        // now loop over some sort of render component...
    }
}

// Could replace this with specific events
#[derive(Default)]
struct EventBuffer {
    events: Vec<Event>,
}

struct WasdMovement;

impl<'a> System<'a> for WasdMovement {
    type SystemData = (
        Read<'a, EventBuffer>,
        ReadStorage<'a, PlayerInputFlag>,
        WriteStorage<'a, Position>,
    );

    fn run(
        &mut self,
        (events_resource, player_input_flag_storage, mut position_storage): Self::SystemData,
    ) {
        let events: &EventBuffer = &events_resource;

        for (_flag, position) in (&player_input_flag_storage, &mut position_storage).join() {
            // Should only be one...
        }
    }
}
