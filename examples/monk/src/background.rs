use super::global::Global;
use log::trace;
use quicksilver::geom::{Rectangle, Vector};
use specs::prelude::*;

use quicksilver_utils_ecs::*;

pub struct BackgroundRender;

impl<'a> System<'a> for BackgroundRender {
    type SystemData = (Write<'a, Global>, Write<'a, RenderContext>);

    fn run(&mut self, (global, mut render_ctx_resource): Self::SystemData) {
        trace!("Drawing background");
        if let Some(background) = &global.background {
            let ctx: &mut RenderContext = &mut render_ctx_resource;
            let full: Rectangle = Rectangle::new(Vector::new(0., 0.), Vector::new(800., 600.));
            ctx.gfx.draw_image(background, full);
        }
    }
}
