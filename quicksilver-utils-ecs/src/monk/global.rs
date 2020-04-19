
use specs::prelude::*;
use super::{interact::Objects, room::Room, dialog::Dialog};
use quicksilver::graphics::{Image, FontRenderer};
use send_wrapper::SendWrapper;

// Single threaded for now, don't both splitting into multiple. Each system can take this as mutable...
pub struct Global {
    pub player: Option<Entity>,
    pub focus: Option<Objects>,
    pub font: SendWrapper<FontRenderer>,
    pub background: Option<SendWrapper<Image>>,
    pub pending_room: Option<Room>,
    pub dialog: Option<Dialog>,
}

impl Default for Global {
    fn default() -> Self {
        panic!("Must be injected")
    }
}

impl Global {
    pub fn new(font: FontRenderer, initial_room: Room) -> Self {
        let player = None;
        let focus = None;
        let font = SendWrapper::new(font);
        let background = None;
        let pending_room = Some(initial_room);
        let dialog = None;
        Global{player, focus, font, background, pending_room, dialog}
    }
}