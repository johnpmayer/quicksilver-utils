extern crate log;
extern crate quicksilver;
extern crate send_wrapper;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use log::{debug, trace};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Graphics, Image},
    input::{Input, Key},
    Window,
};
use send_wrapper::SendWrapper;
use specs::{prelude::*, Component, System, Write};
use std::sync::{Arc, Mutex};

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
    pub row: u32,
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
            let sprite_row = sprite.row * sprite.height;
            let sprite_position = Rectangle::new(
                Vector::new(sprite_offset as f32, sprite_row as f32),
                Vector::new(sprite.width as f32, sprite.height as f32),
            );
            let location_size = sprite.width as f32 * sprite.scale;
            let location = Rectangle::new(
                Vector::new(position.x, position.y),
                Vector::new(location_size, location_size),
            );
            debug!(
                "Drawing dude from sprite {:?} at {:?}",
                sprite_position, location
            );
            ctx.gfx
                .draw_subimage(&sprite.image, sprite_position, location);
        }
    }
}

#[derive(Clone)]
pub struct InputContext {
    pub input: Arc<Mutex<SendWrapper<Input>>>, // quicksilver EventStream uses RefCell
}

impl InputContext {
    // mutability is not required, but this helps enforce the ECS system has a write lock on the object
    pub fn with_locked_input<T, F>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Input) -> T,
    {
        let input_arc: &Arc<Mutex<SendWrapper<Input>>> = &self.input;
        let mut input_wrapper = input_arc.lock().unwrap();
        let input: &mut Input = &mut input_wrapper;
        f(input)
    }
}

impl Default for InputContext {
    fn default() -> Self {
        panic!("must be injected...")
    }
}

pub struct WasdMovement;

impl<'a> System<'a> for WasdMovement {
    type SystemData = (
        Write<'a, InputContext>,
        ReadStorage<'a, PlayerInputFlag>,
        WriteStorage<'a, Position>,
    );

    fn run(
        &mut self,
        (mut input_ctx_resource, player_input_flag_storage, mut position_storage): Self::SystemData,
    ) {
        trace!("Running WasdMovement");

        let input_ctx: &mut InputContext = &mut input_ctx_resource;

        let speed = 3.; // TODO, configurable per entity, and should also take into account tick delta
        let mut velocity = [0., 0.];

        let input_arc: &Arc<Mutex<SendWrapper<Input>>> = &input_ctx.input;
        let mut input_wrapper = input_arc.lock().unwrap();
        let input: &mut Input = &mut input_wrapper;

        // input_ctx.with_locked_input(move |input| {
        if input.key_down(Key::W) {
            velocity[1] = -speed;
        }
        if input.key_down(Key::A) {
            velocity[0] = -speed;
        }
        if input.key_down(Key::S) {
            velocity[1] = speed;
        }
        if input.key_down(Key::D) {
            velocity[0] = speed;
        }
        // });

        for (_flag, position) in (&player_input_flag_storage, &mut position_storage).join() {
            position.x += velocity[0];
            position.y += velocity[1];
        }
    }
}
