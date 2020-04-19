
use specs::prelude::*;
use crate::*;
use super::{global::Global, interact::*};
use log::info;
use quicksilver::graphics::Image;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Room {
    Bedroom,
    Hall
}

pub struct RoomData {
    pub player_sprite: Image,
    pub bedroom_background: Image,
    pub bedroom_bed_sprite: Image,
    pub hall_background: Image,
}

pub struct RoomSystem {
    pub room_data: SendWrapper<RoomData>
}

impl RoomSystem {
    pub fn setup_new_room(&self, world: &mut World) {

        let pending_room = world.fetch::<Global>().pending_room;

        if let Some(room) = pending_room {

            info!("Switching rooms");

            {
                let entities = world.system_data::<Entities>();
                for entity in entities.join() {
                    entities.delete(entity).expect("deletion");
                }
            }
            world.maintain();

            match room {
                Room::Bedroom => {
                    let player_sprite = SpriteConfig {
                        image: SendWrapper::new(self.room_data.player_sprite.clone()),
                        width: 32,
                        height: 32,
                        scale: 2.,
                        animation: None,
                    };

                    let player_entity = world
                        .create_entity()
                        .with(Position { x: 550., y: 400. })
                        .with(player_sprite)
                        .with(PlayerInputFlag)
                        .with(PlayerInteract { width: 64., height: 64.})
                        .build();
                        
                    let bed_sprite = SpriteConfig {
                        image: SendWrapper::new(self.room_data.bedroom_bed_sprite.clone()),
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

                    world
                        .create_entity()
                        .with(Position{x: 50., y: 140.})
                        .with(ObjectInteract{object: Objects::EnterHall, width: 100., height: 220.})
                        .build();

                    let mut global = world.get_mut::<Global>().expect("global resource");
                    global.player = Some(player_entity);
                    global.background = Some(SendWrapper::new(self.room_data.bedroom_background.clone()))
                }
                Room::Hall => {
                    let player_sprite = SpriteConfig {
                        image: SendWrapper::new(self.room_data.player_sprite.clone()),
                        width: 32,
                        height: 32,
                        scale: 2.,
                        animation: None,
                    };

                    let player_entity = world
                        .create_entity()
                        .with(Position { x: 700., y: 300. })
                        .with(player_sprite)
                        .with(PlayerInputFlag)
                        .with(PlayerInteract { width: 64., height: 64.})
                        .build();

                    world
                        .create_entity()
                        .with(Position{x: 700., y: 180.})
                        .with(ObjectInteract{object: Objects::EnterBedroom, width: 100., height: 200.})
                        .build();

                    let mut global = world.get_mut::<Global>().expect("global resource");
                    global.player = Some(player_entity);
                    global.background = Some(SendWrapper::new(self.room_data.hall_background.clone()))
                }
            }

            let global: &mut Global = world.get_mut::<Global>().expect("global resource");

            global.pending_room = None
        }
    }
}
