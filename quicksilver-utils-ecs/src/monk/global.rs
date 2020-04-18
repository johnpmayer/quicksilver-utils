
use specs::prelude::*;
use super::interact::Objects;
use quicksilver::graphics::{Image, FontRenderer};
use send_wrapper::SendWrapper;

// Single threaded for now, don't both splitting into multiple. Each system can take this as mutable...
pub struct Global {
    pub player: Entity,
    pub focus: Option<Objects>,
    pub font: SendWrapper<FontRenderer>,
    pub background: SendWrapper<Image>,
}

impl Default for Global {
    fn default() -> Self {
        panic!("Must be injected")
    }
}

impl Global {
    pub fn new(player_entity: Entity, font: FontRenderer, background: Image) -> Self {
        let player = player_entity;
        let focus = None;
        let font = SendWrapper::new(font);
        let background = SendWrapper::new(background);
        Global{player, focus, font, background}
    }
}