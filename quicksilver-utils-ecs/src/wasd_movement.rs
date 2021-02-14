use log::trace;
use quicksilver::input::{Input, Key};
use send_wrapper::SendWrapper;
use specs::{prelude::*, Component, System, Write};

use super::Position;

#[derive(Component)]
pub struct PlayerInputFlag;

pub struct InputContext {
    pub input: SendWrapper<Input>, // quicksilver EventStream uses RefCell
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

        let input: &mut Input = &mut input_ctx.input;

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

        for (_flag, position) in (&player_input_flag_storage, &mut position_storage).join() {
            position.x += velocity[0];
            position.y += velocity[1];
        }
    }
}
