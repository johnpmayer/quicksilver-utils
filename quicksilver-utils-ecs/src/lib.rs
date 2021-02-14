extern crate log;
extern crate quicksilver;
extern crate send_wrapper;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use specs::{prelude::*, Component};

mod render_sprites;
mod wasd_movement;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub use render_sprites::{
    AnimationConfig, RenderContext, RenderSprites, SpriteConfig, TimeContext,
};
pub use wasd_movement::{InputContext, PlayerInputFlag, WasdMovement};
