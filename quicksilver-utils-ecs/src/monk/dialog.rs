
use specs::prelude::*;
use super::global::Global;
use crate::*;
use quicksilver::{geom::Rectangle, graphics::Color};

#[derive(Clone, Copy)]
pub enum Dialog {
    SleepConfirm,
}

impl Dialog {
    fn text(&self) -> &'static str {
        match self {
            Dialog::SleepConfirm => "Are you sure you want to go to sleep?\n(Y/N)",
        }
    }

    pub fn process(&self, global: &mut Global, event_cache: &EventCache) -> bool {
        let mut should_close = false;

        match self {
            Dialog::SleepConfirm => {
                if event_cache.key(Key::Y) {
                    should_close = true
                } else if event_cache.key(Key::N) {
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