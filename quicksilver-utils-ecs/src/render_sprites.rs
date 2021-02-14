use log::{debug, trace};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Graphics, Image},
    Window,
};
use send_wrapper::SendWrapper;
use specs::{prelude::*, Component, System, Write};

use super::Position;

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
            trace!(
                "Drawing from sprite {:?} at {:?}",
                sprite_position,
                location
            );
            ctx.gfx
                .draw_subimage(&sprite.image, sprite_position, location);
        }
    }
}
