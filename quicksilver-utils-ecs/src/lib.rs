extern crate log;
extern crate quicksilver;
extern crate send_wrapper;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use log::debug;
use quicksilver::{
    geom::Rectangle,
    graphics::{Color, Graphics, Image},
    lifecycle::{Event, Window},
};
use send_wrapper::SendWrapper;
use specs::{prelude::*, Component, System, Write};

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Sprite {
    pub image: SendWrapper<Image>, // quicksilver graphics uses Rc
}

#[derive(Component)]
pub struct PlayerInputFlag;

pub struct SharedRenderingContext {
    pub gfx: SendWrapper<Graphics>,  // quicksilver graphics uses Rc
    pub window: SendWrapper<Window>, // quicksilver graphics uses *mut(0)
}

impl Default for SharedRenderingContext {
    fn default() -> Self {
        panic!("must be injected...")
    }
}

pub struct RenderSprites;

impl<'a> System<'a> for RenderSprites {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Sprite>,
        Write<'a, SharedRenderingContext>,
    );
    fn run(&mut self, (position_storage, sprite_storage, mut ctx_resource): Self::SystemData) {
        let ctx: &mut SharedRenderingContext = &mut ctx_resource;
        ctx.gfx.clear(Color::WHITE);
        debug!("Running RenderSprites");
        for (position, sprite) in (&position_storage, &sprite_storage).join() {
            let sprite_position = Rectangle::new((0, 0), (32, 32));
            let location_size = 64.;
            let location = Rectangle::new(
                (position.x, position.y),
                (position.x + location_size, position.y + location_size),
            );
            debug!(
                "Drawing dude from sprite {:?} at {:?}",
                sprite_position, location
            );
            ctx.gfx
                .draw_subimage(&sprite.image, sprite_position, location);
        }
        ctx.gfx.present(&ctx.window).expect("present"); // FIXME probably a separate system?
    }
}

// Could replace this with specific events
#[derive(Default)]
pub struct EventBuffer {
    events: Vec<Event>,
}

pub struct WasdMovement;

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
        debug!("Running WasdMovement");
        for (_flag, position) in (&player_input_flag_storage, &mut position_storage).join() {
            // Should only be one...
        }
    }
}
