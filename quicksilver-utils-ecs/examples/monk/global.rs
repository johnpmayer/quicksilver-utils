
use specs::prelude::*;
use crate::interact::Objects;

// Single threaded for now, don't both splitting into multiple. Each system can take this as mutable...
#[derive(Default)]
pub struct Global {
    pub player: Option<Entity>,
    pub focus: Option<Objects>,
}

impl Global {
    pub fn new(player_entity: Entity) -> Self {
        let player = Some(player_entity);
        let focus = None;
        Global{player,focus}
    }
}