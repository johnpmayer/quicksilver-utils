
use specs::prelude::*;
use super::global::Global;
use crate::*;
use quicksilver::{geom::Rectangle, graphics::Color};

#[derive(Clone, Copy)]
pub enum Dialog {
    Greet,
    SleepConfirm,
    DelegateWheat,
    PendingDelegateWheat,
    NoWheatToBake,
    DelegateBake,
    PendingDelegateBake,
}

impl Dialog {
    fn text(&self) -> &'static str {
        match self {
            Dialog::Greet => "Hello Brother!\n(Enter)",
            Dialog::SleepConfirm => "Are you sure you want to go to sleep?\n(Y/N)",
            Dialog::DelegateWheat => "Hello Brother! Shall I begin planting wheat?\n(Y/N)",
            Dialog::PendingDelegateWheat => "I'll start growing tomorrow!\n(Enter)",
            Dialog::NoWheatToBake => "Hello Brother! We have nothing to bake...\n(Enter)",
            Dialog::DelegateBake => "Hello Borther! Shall I begin baking bread?\n(Y,N)",
            Dialog::PendingDelegateBake => "I'll start baking tomorrow!\n(Enter)",
        }
    }

    pub fn process(&self, global: &mut Global, event_cache: &EventCache) -> bool {
        let mut should_close = false;

        match self {
            Dialog::SleepConfirm => {
                if event_cache.key(Key::Y) {
                    // Effect 'commands'
                    if global.progress.delegated_wheat {
                        global.progress.growing_wheat = true
                    }
                    if global.progress.delegated_baking {
                        global.progress.baking_bread = true
                    }
                    should_close = true
                } else if event_cache.key(Key::N) {
                    should_close = true
                }
            }
            Dialog::DelegateWheat => {
                if event_cache.key(Key::Y) {
                    global.progress.delegated_wheat = true;
                    should_close = true
                } else if event_cache.key(Key::N) {
                    should_close = true
                }
            }
            Dialog::DelegateBake => {
                if event_cache.key(Key::Y) {
                    global.progress.delegated_baking = true;
                    should_close = true
                } else if event_cache.key(Key::N) {
                    should_close = true
                }
            }
            _ => {
                if event_cache.key(Key::Return) {
                    should_close = true
                }
            }
        }

        should_close
    }
}

pub struct DialogRender;

impl<'a> System<'a> for DialogRender {
    type SystemData = (Write<'a, Global>, Write<'a, RenderContext>,);

    fn run(&mut self, (mut global, mut render_ctx_resource): Self::SystemData) {
        let ctx: &mut RenderContext = &mut render_ctx_resource;
        if let Some(dialog) = global.dialog {
            let popup_area = Rectangle::new((100., 100.,), (600., 400.,));
            ctx.gfx.fill_rect(&popup_area, Color::from_rgba(200, 200, 200, 0.9));

            let text_area = Rectangle::new((120., 120.,), (560., 360.,));
            let text = dialog.text();
            global.font.draw_wrapping(&mut ctx.gfx, &text, Some(text_area.size.x), Color::BLACK, text_area.pos).expect("draw text");
        }
    }
}