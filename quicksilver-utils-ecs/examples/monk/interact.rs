
use specs::{prelude::*, Component, System, Write};
use quicksilver_utils_ecs::*;
use super::global::Global;
use log::debug;

#[derive(Component)]
pub struct PlayerInteract {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum Objects {
    Bed
}

impl Objects {
    pub fn label(&self) -> &'static str {
        match self {
            Objects::Bed => "bed"
        }
    }
}

#[derive(Component)]
pub struct ObjectInteract {
    pub object: Objects,
    pub width: f32,
    pub height: f32,
}

pub struct DetectInteractionRange;

struct BoundingBox<'a> {
    pub position: &'a Position,
    pub width: f32,
    pub height: f32
}

fn overlaps(a: &BoundingBox, b: &BoundingBox) -> bool {
    let out_left = a.position.x + a.width < b.position.x;
    let out_right = a.position.x > b.position.x + b.width;
    let out_up = a.position.y + a.height < b.position.y;
    let out_down = a.position.y > b.position.y + b.height;
    !(out_left || out_right || out_up || out_down)
}

impl<'a> System<'a> for DetectInteractionRange {
    type SystemData = (
        Write<'a, Global>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, PlayerInteract>,
        ReadStorage<'a, ObjectInteract>,
    );

    fn run(
        &mut self,
        (mut global, position_storage, player_interact_storage, object_interact_storage,): Self::SystemData,
    ) {
        let player: Entity = global.player.expect("player entity is unset");
        let player_position: &Position = position_storage.get(player).expect("player entity has no position");
        let player_interact: &PlayerInteract = player_interact_storage.get(player).expect("player entity has no player interact");
        let player_bounding_box = BoundingBox {
            position: player_position,
            width: player_interact.width,
            height: player_interact.height,
        };

        global.focus = None;
        for (object_position, object_interact) in (&position_storage, &object_interact_storage).join() {
            let object_bounding_box = BoundingBox {
                position: object_position,
                width: object_interact.width,
                height: object_interact.height,
            };
            if overlaps(&player_bounding_box, &object_bounding_box) {
                global.focus = Some(object_interact.object);
                break
            }
        }

        if let Some(focus) = global.focus {
            debug!("We have a focus: {:?}", focus)
        }
    }
}