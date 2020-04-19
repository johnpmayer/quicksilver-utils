
use specs::prelude::*;
use log::trace;
use super::global::Global;
use quicksilver::{geom::Rectangle};

use crate::*;

pub struct BackgroundRender;

impl<'a> System<'a> for BackgroundRender {
    type SystemData = (Write<'a, Global>, Write<'a, RenderContext>,);

    fn run(&mut self, (global, mut render_ctx_resource): Self::SystemData) {
        trace!("Drawing background");
        if let Some(background) = &global.background {
            let ctx: &mut RenderContext = &mut render_ctx_resource;
            let full: Rectangle = Rectangle::new((0.,0.,),(800.,600.));
            ctx.gfx.draw_image(background, full);
        }   
    }
}