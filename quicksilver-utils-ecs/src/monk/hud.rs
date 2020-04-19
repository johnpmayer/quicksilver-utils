
use specs::prelude::*;
use quicksilver::{geom::Vector, graphics::Color};
use log::trace;
use super::global::Global;

use crate::*;

pub struct HudRender;

impl<'a> System<'a> for HudRender {
    type SystemData = (Write<'a, Global>, Write<'a, RenderContext>,);

    fn run(&mut self, (mut global, mut render_ctx_resource): Self::SystemData) {
        let ctx: &mut RenderContext = &mut render_ctx_resource;
        if global.dialog.is_some() {
            // Don't display interaction options
            return;
        }
        if let Some(focus_object) = global.focus {
            let focus_text = format!("'E' to {}.", focus_object.label());
            trace!("We have some text to render: {}", focus_text);
            global.font.draw(&mut ctx.gfx, &focus_text, Color::BLACK, Vector::new(100., 500.)).expect("draw text");
        }
    }
}