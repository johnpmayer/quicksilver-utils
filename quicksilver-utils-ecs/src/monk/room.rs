
use specs::prelude::*;
use crate::*;
use super::{global::Global, interact::*};
use log::info;
use quicksilver::graphics::Image;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Room {
    Bedroom,
    Hall,
    Cellar,
    Garden,
}

pub struct RoomData {
    pub characters_spritesheet: Image,
    pub bedroom_background: Image,
    pub bedroom_bed_sprite: Image,
    pub bedroom_desk_sprite: Image,
    pub hall_background: Image,
    pub cellar_background: Image,
    pub garden_background: Image,
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

            let progress = world.fetch::<Global>().progress;

            match room {
                Room::Bedroom => {
                    let player_sprite = SpriteConfig {
                        image: SendWrapper::new(self.room_data.characters_spritesheet.clone()),
                        row: 0,
                        width: 32,
                        height: 32,
                        scale: 2.,
                        animation: None,
                    };

                    let player_entity = world
                        .create_entity()
                        .with(Position { x: 100., y: 350. })
                        .with(player_sprite)
                        .with(PlayerInputFlag)
                        .with(PlayerInteract { width: 64., height: 64.})
                        .build();
                        
                    let bed_sprite = SpriteConfig {
                        image: SendWrapper::new(self.room_data.bedroom_bed_sprite.clone()),
                        row: 0,
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

                    let desk_sprite_row = if progress.making_paper { 1 } else { 0};

                    let desk_sprite = SpriteConfig {
                        image: SendWrapper::new(self.room_data.bedroom_desk_sprite.clone()),
                        row: desk_sprite_row,
                        width: 32,
                        height: 32,
                        scale: 3.,
                        animation: None,
                    };

                    world
                        .create_entity()
                        .with(Position { x: 300., y: 300.})
                        .with(desk_sprite)
                        .with(ObjectInteract { object: Objects::Desk, width: 32. * 3., height: 32.*3.,})
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
                        image: SendWrapper::new(self.room_data.characters_spritesheet.clone()),
                        row: 0,
                        width: 32,
                        height: 32,
                        scale: 2.,
                        animation: None,
                    };

                    // TODO: use "global.last_room" to determine start position of player

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

                    world
                        .create_entity()
                        .with(Position{x: 650., y: 500.})
                        .with(ObjectInteract{object: Objects::EnterCellar, width: 100., height: 200.})
                        .build();

                    world
                        .create_entity()
                        .with(Position{x: 50., y: 250.})
                        .with(ObjectInteract{object: Objects::EnterGarden, width: 100., height: 230.})
                        .build();

                    if !progress.growing_wheat {
                        let gardener_sprite = SpriteConfig {
                            image: SendWrapper::new(self.room_data.characters_spritesheet.clone()),
                            row: 2,
                            width: 32,
                            height: 32,
                            scale: 2.,
                            animation: None,
                        };

                        world
                            .create_entity()
                            .with(Position{x: 200., y: 450.})
                            .with(gardener_sprite)
                            .with(ObjectInteract{object: Objects::TalkGardener, width: 64., height: 64.})
                            .build();
                    }

                    if !progress.baking_bread {
                        let baker_sprite = SpriteConfig {
                            image: SendWrapper::new(self.room_data.characters_spritesheet.clone()),
                            row: 1,
                            width: 32,
                            height: 32,
                            scale: 2.,
                            animation: None,
                        };

                        world
                            .create_entity()
                            .with(Position{x: 550., y: 450.})
                            .with(baker_sprite)
                            .with(ObjectInteract{object: Objects::TalkBaker, width: 64., height: 64.})
                            .build();
                    }

                    if progress.baking_bread {
                        let artisan_sprite = SpriteConfig {
                            image: SendWrapper::new(self.room_data.characters_spritesheet.clone()),
                            row: 3,
                            width: 32,
                            height: 32,
                            scale: 2.,
                            animation: None,
                        };

                        world
                            .create_entity()
                            .with(Position{x: 350., y: 375.})
                            .with(artisan_sprite)
                            .with(ObjectInteract{object: Objects::TalkArtisan, width: 64., height: 64.})
                            .build();
                    }

                    if progress.guests {
                        let king_sprite = SpriteConfig {
                            image: SendWrapper::new(self.room_data.characters_spritesheet.clone()),
                            row: 7,
                            width: 32,
                            height: 32,
                            scale: 2.,
                            animation: None,
                        };

                        world
                            .create_entity()
                            .with(Position{x: 300., y: 500.})
                            .with(king_sprite)
                            .with(ObjectInteract{object: Objects::TalkKing, width: 64., height: 64.})
                            .build();
                    }

                    let mut global = world.get_mut::<Global>().expect("global resource");
                    global.player = Some(player_entity);
                    global.background = Some(SendWrapper::new(self.room_data.hall_background.clone()))
                }
                Room::Cellar => {
                    let player_sprite = SpriteConfig {
                        image: SendWrapper::new(self.room_data.characters_spritesheet.clone()),
                        row: 0,
                        width: 32,
                        height: 32,
                        scale: 2.,
                        animation: None,
                    };

                    let player_entity = world
                        .create_entity()
                        .with(Position { x: 650., y: 300. })
                        .with(player_sprite)
                        .with(PlayerInputFlag)
                        .with(PlayerInteract { width: 64., height: 64.})
                        .build();
                    
                        world
                        .create_entity()
                        .with(Position{x: 600., y: 150.})
                        .with(ObjectInteract{object: Objects::EnterHall, width: 100., height: 250.})
                        .build();

                    if progress.baking_bread {
                        let baker_sprite = SpriteConfig {
                            image: SendWrapper::new(self.room_data.characters_spritesheet.clone()),
                            row: 1,
                            width: 32,
                            height: 32,
                            scale: 2.,
                            animation: None,
                        };

                        world
                            .create_entity()
                            .with(Position{x: 300., y: 450.})
                            .with(baker_sprite)
                            .with(ObjectInteract{object: Objects::TalkBaker, width: 64., height: 64.})
                            .build();
                    }
                    
                    let mut global = world.get_mut::<Global>().expect("global resource");
                    global.player = Some(player_entity);
                    global.background = Some(SendWrapper::new(self.room_data.cellar_background.clone()))
                }
                Room::Garden => {
                    let player_sprite = SpriteConfig {
                        image: SendWrapper::new(self.room_data.characters_spritesheet.clone()),
                        row: 0,
                        width: 32,
                        height: 32,
                        scale: 2.,
                        animation: None,
                    };

                    let player_entity = world
                        .create_entity()
                        .with(Position { x: 400., y: 280. })
                        .with(player_sprite)
                        .with(PlayerInputFlag)
                        .with(PlayerInteract { width: 64., height: 64.})
                        .build();
                    
                        world
                        .create_entity()
                        .with(Position{x: 400., y: 200.})
                        .with(ObjectInteract{object: Objects::EnterHall, width: 50., height: 120.})
                        .build();

                    if progress.growing_wheat {
                        let gardener_sprite = SpriteConfig {
                            image: SendWrapper::new(self.room_data.characters_spritesheet.clone()),
                            row: 2,
                            width: 32,
                            height: 32,
                            scale: 2.,
                            animation: None,
                        };

                        world
                            .create_entity()
                            .with(Position{x: 600., y: 350.})
                            .with(gardener_sprite)
                            .with(ObjectInteract{object: Objects::TalkGardener, width: 64., height: 64.})
                            .build();
                    }

                    if !progress.charity_inspiration {
                        let beggar_sprite = SpriteConfig {
                            image: SendWrapper::new(self.room_data.characters_spritesheet.clone()),
                            row: 4,
                            width: 32,
                            height: 32,
                            scale: 2.,
                            animation: None,
                        };

                        world
                            .create_entity()
                            .with(Position{x: 50., y: 500.})
                            .with(beggar_sprite)
                            .with(ObjectInteract{object: Objects::TalkBeggar, width: 64., height: 64.})
                            .build();
                    }

                    let mut global = world.get_mut::<Global>().expect("global resource");
                    global.player = Some(player_entity);
                    global.background = Some(SendWrapper::new(self.room_data.garden_background.clone()))
                }
            }

            let global: &mut Global = world.get_mut::<Global>().expect("global resource");

            global.pending_room = None
        }
    }
}
