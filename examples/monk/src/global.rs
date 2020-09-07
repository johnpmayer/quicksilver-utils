
use specs::prelude::*;
use super::{interact::Objects, room::Room, dialog::Dialog};
use quicksilver::graphics::{Image, FontRenderer};
use send_wrapper::SendWrapper;

#[derive(Default, Clone, Copy)]
pub struct GameProgression {
    pub delegated_wheat: bool,
    pub growing_wheat: bool,

    pub delegated_baking: bool,
    pub baking_bread: bool,

    pub gave_to_charity: bool,
    pub charity_inspiration: bool,

    pub delegated_papermaking: bool,
    pub making_paper: bool,

    pub has_paper_today: bool,

    // learn purpose
    pub know_eastern_monestary: bool,
    pub sent_eastern_monestary: bool,
    pub reply_eastern_purpose: bool,

    // learn recipe
    pub know_northern_monestary: bool,
    pub sent_northern_monestary: bool,
    pub reply_northern_beer: bool,

    // get seeds
    pub know_southern_monestary: bool,
    pub sent_southern_monestary: bool,
    pub reply_southern_hops: bool,

    pub delegated_hops: bool,
    pub growing_hops: bool,

    pub delegated_beer: bool,
    pub brewing_beer: bool,

    pub know_invite: bool,
    pub sent_invite: bool,
    pub guests: bool,
}

// Single threaded for now, don't both splitting into multiple. Each system can take this as mutable...
pub struct Global {
    pub player: Option<Entity>,
    pub focus: Option<Objects>,
    pub font: SendWrapper<FontRenderer>,
    pub background: Option<SendWrapper<Image>>,
    pub pending_room: Option<Room>,
    pub dialog: Option<Dialog>,
    pub progress: GameProgression
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
        let dialog = Some(Dialog::Welcome);
        let progress = GameProgression::default();
        Global{player, focus, font, background, pending_room, dialog, progress}
    }
}