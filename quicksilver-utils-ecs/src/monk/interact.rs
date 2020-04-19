
use specs::{prelude::*, Component, System, Write};
use crate::*;
use super::{global::Global, room::Room, dialog::Dialog};
use log::{info, trace};
use quicksilver::lifecycle::{Key, EventCache};
use instant::Instant;

#[derive(Component)]
pub struct PlayerInteract {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Objects {
    Bed,
    EnterHall,
    EnterBedroom,
    EnterCellar,
    EnterGarden,
    TalkGardener,
    TalkBaker,
    TalkBeggar,
    TalkArtisan,
}

impl Objects {
    pub fn label(&self) -> &'static str {
        match self {
            Objects::Bed => "go to sleep",
            Objects::EnterHall => "enter the hall",
            Objects::EnterBedroom => "enter the bedroom",
            Objects::EnterCellar => "enter the cellar",
            Objects::EnterGarden => "enter the garden",
            Objects::TalkGardener => "speak with the gardener",
            Objects::TalkBaker => "speak with the baker",
            Objects::TalkBeggar => "speak with the beggar",
            Objects::TalkArtisan => "speak with the artisan",
        }
    }
}

#[derive(Component)]
pub struct ObjectInteract {
    pub object: Objects,
    pub width: f32,
    pub height: f32,
}

pub struct InteractionSystem {
    event_cache: EventCache,
    last_interaction: Option<Instant>,
}

impl InteractionSystem {
    pub fn new() -> Self {
        InteractionSystem { 
            event_cache: EventCache::new(),
            last_interaction: None,
        }
    }
}

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

impl<'a> System<'a> for InteractionSystem {
    type SystemData = (
        Write<'a, Global>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, PlayerInteract>,
        ReadStorage<'a, ObjectInteract>,
        Read<'a, EventBuffer>,
    );

    fn run(
        &mut self,
        (mut global, position_storage, player_interact_storage, object_interact_storage, event_buffer): Self::SystemData,
    ) {
        for event in event_buffer.events.iter() {
            self.event_cache.process_event(event)
        }
        
        let player: Entity = global.player.expect("player entity");
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

        if let Some(dialog) = global.dialog {
            let should_close = dialog.process(&mut global, &self.event_cache);
            if should_close {
                global.dialog = None
            }
        } else {
            if let Some(focus) = global.focus {
                trace!("We have a focus: {:?}", focus);

                if self.event_cache.key(Key::E) {
                    let now = Instant::now();

                    // Debounce!
                    let should_interact = if let Some(last_interaction) = self.last_interaction {
                        now.duration_since(last_interaction).as_millis() > 500
                    } else {
                        true
                    };

                    if should_interact {
                        info!("Interact with {:?}!", focus);

                        if focus == Objects::EnterHall {
                            global.pending_room = Some(Room::Hall)
                        } else if focus == Objects::EnterBedroom {
                            global.pending_room = Some(Room::Bedroom)
                        } else if focus == Objects::EnterCellar {
                            global.pending_room = Some(Room::Cellar)
                        } else if focus == Objects::EnterGarden {
                            global.pending_room = Some(Room::Garden)
                        }

                        if focus == Objects::Bed {

                            global.dialog = Some(Dialog::SleepConfirm)

                        } else if focus == Objects::TalkGardener {

                            if !global.progress.delegated_wheat {
                                global.dialog = Some(Dialog::DelegateWheat)
                            } else if !global.progress.growing_wheat {
                                global.dialog = Some(Dialog::PendingDelegateWheat)
                            } else {
                                global.dialog = Some(Dialog::Greet)
                            }

                        } else if focus == Objects::TalkBaker {

                            if !global.progress.growing_wheat {
                                global.dialog = Some(Dialog::NoWheatToBake)
                            } else if !global.progress.delegated_baking {
                                global.dialog = Some(Dialog::DelegateBake)
                            } else if !global.progress.baking_bread {
                                global.dialog = Some(Dialog::PendingDelegateBake)
                            } else {
                                global.dialog = Some(Dialog::Greet)
                            }

                        } else if focus == Objects::TalkBeggar {

                            if !global.progress.baking_bread {
                                global.dialog = Some(Dialog::NoBreadToGive)
                            } else if !global.progress.gave_to_charity {
                                global.dialog = Some(Dialog::GiveBread)
                            } else {
                                global.dialog = Some(Dialog::ThanksForBread)
                            }

                        } else if focus == Objects::TalkArtisan {

                            if !global.progress.charity_inspiration {
                                global.dialog = Some(Dialog::Uninspired)
                            } else if !global.progress.delegated_papermaking {
                                global.dialog = Some(Dialog::DelegatePaper)
                            } else if !global.progress.making_paper {
                                global.dialog = Some(Dialog::PendingDelegatePaper)
                            } else {
                                global.dialog = Some(Dialog::Greet)
                            }

                        }

                        self.last_interaction = Some(now)
                    }
                }
            }
        }
    }
}