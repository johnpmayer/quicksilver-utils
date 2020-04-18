extern crate log;
extern crate quicksilver;
extern crate send_wrapper;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use log::trace;
use quicksilver::{
    geom::Rectangle,
    graphics::{Color, Graphics, Image},
    lifecycle::{Event, EventCache, Key, Window},
};
use send_wrapper::SendWrapper;
use specs::{prelude::*, Component, System, Write};

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct AnimationConfig {
    pub loop_start_time: f64,
    pub frames: Vec<u32>,
}

#[derive(Component)]
pub struct SpriteConfig {
    pub image: SendWrapper<Image>, // quicksilver graphics uses Rc
    pub width: u32,
    pub height: u32,
    pub scale: f32,
    pub animation: Option<AnimationConfig>,
}

#[derive(Component)]
pub struct PlayerInputFlag;

pub struct RenderContext {
    pub gfx: SendWrapper<Graphics>,  // quicksilver graphics uses Rc
    pub window: SendWrapper<Window>, // quicksilver graphics uses *mut(0)
}

impl Default for RenderContext {
    fn default() -> Self {
        panic!("must be injected...")
    }
}

pub struct TimeContext {
    pub now: f64,
}

impl Default for TimeContext {
    fn default() -> Self {
        panic!("must be injected...")
    }
}

pub struct RenderSprites;

impl<'a> System<'a> for RenderSprites {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, SpriteConfig>,
        Read<'a, TimeContext>,
        Write<'a, RenderContext>,
    );

    fn run(
        &mut self,
        (position_storage, sprite_storage, time_ctx_resource, mut render_ctx_resource): Self::SystemData,
    ) {
        let time_ctx: &TimeContext = &time_ctx_resource;
        let ctx: &mut RenderContext = &mut render_ctx_resource;
        // ctx.gfx.clear(Color::from_rgba(200,200,200,1.));
        trace!("Running RenderSprites");
        for (position, sprite) in (&position_storage, &sprite_storage).join() {
            let sprite_offset: u32 = if let Some(animation) = &sprite.animation {
                let sprite_loop_elapsed = (time_ctx.now - animation.loop_start_time) as u32;
                let total_frames: u32 = animation.frames.iter().sum();
                let cycle: u32 = sprite_loop_elapsed % total_frames;
                let mut offset = 0;
                let mut x = 0;
                for frame_length in animation.frames.iter() {
                    x += frame_length;
                    if cycle < x {
                        break;
                    };
                    offset += 1;
                }
                offset
            } else {
                0
            };
            let sprite_offset = sprite_offset * sprite.width;
            let sprite_position = Rectangle::new((sprite_offset, 0), (sprite.width, sprite.height));
            let location_size = sprite.width as f32 * sprite.scale;
            let location = Rectangle::new((position.x, position.y), (location_size, location_size));
            trace!(
                "Drawing dude from sprite {:?} at {:?}",
                sprite_position, location
            );
            ctx.gfx
                .draw_subimage(&sprite.image, sprite_position, location);
        }
        // ctx.gfx.present(&ctx.window).expect("present"); // FIXME probably a separate system?
    }
}

// Could replace this with specific events
#[derive(Default)]
pub struct EventBuffer {
    pub events: Vec<Event>,
}

pub struct WasdMovement {
    pub event_cache: EventCache,
}

impl<'a> System<'a> for WasdMovement {
    type SystemData = (
        Read<'a, EventBuffer>,
        ReadStorage<'a, PlayerInputFlag>,
        WriteStorage<'a, Position>,
    );

    fn run(
        &mut self,
        (eventbuffer_resource, player_input_flag_storage, mut position_storage): Self::SystemData,
    ) {
        trace!("Running WasdMovement");
        let eventbuffer: &EventBuffer = &eventbuffer_resource;

        let speed = 3.; // configurable per entity, and should also take into account tick delta
        let mut velocity = [0., 0.];

        for event in eventbuffer.events.iter() {
            self.event_cache.process_event(event)
        }

        if self.event_cache.key(Key::W) {
            velocity[1] = -speed;
        }

        if self.event_cache.key(Key::A) {
            velocity[0] = -speed;
        }

        if self.event_cache.key(Key::S) {
            velocity[1] = speed;
        }

        if self.event_cache.key(Key::D) {
            velocity[0] = speed;
        }

        for (_flag, position) in (&player_input_flag_storage, &mut position_storage).join() {
            position.x += velocity[0];
            position.y += velocity[1];
        }
    }
}
