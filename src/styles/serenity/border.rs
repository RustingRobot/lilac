use druid::{kurbo::Line, theme, Color, Data, RenderContext, Widget};
use piet_common::{LineCap, StrokeStyle};

use crate::styles::stylizer::Stylizer;

pub struct Border {}

impl Border {
    pub fn new() -> Self {
        Border {}
    }
}

impl<T: Data, W: Widget<T>> Stylizer<T, W> for Border {
    fn layout(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &druid::Env,
    ) -> druid::Size {
        child.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, child: &mut W, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {
        let size = ctx.size();

        let base_color = Color::rgba8(212, 208, 200, 255);
        let highlight_color = Color::rgba8(255, 255, 255, 255);
        let dark_color = Color::rgba8(128, 128, 128, 255);
        let shadow_color = Color::rgba8(64, 64, 64, 255);

        ctx.fill(size.to_rect(), &base_color);
        ctx.stroke(
            Line::new((1.5, 1.5), (1.5, size.height - 1.5)),
            &highlight_color,
            1.0,
        );
        ctx.stroke(
            Line::new((1.5, 1.5), (size.width - 1.5, 1.5)),
            &highlight_color,
            1.0,
        );
        ctx.stroke(
            Line::new((1.5, size.height - 1.5), (size.width, size.height - 1.5)),
            &dark_color,
            1.0,
        );
        ctx.stroke(
            Line::new((size.width - 1.5, 1.5), (size.width - 1.5, size.height)),
            &dark_color,
            1.0,
        );
        ctx.stroke(
            Line::new((0.0, size.height - 0.5), (size.width, size.height - 0.5)),
            &shadow_color,
            1.0,
        );
        ctx.stroke(
            Line::new((size.width - 0.5, 0.0), (size.width - 0.5, size.height)),
            &shadow_color,
            1.0,
        );
        //let clip_rect = size.to_rect().inset(-0.5);
        //ctx.stroke(clip_rect, &Color::RED, 1.0);
    }
}
