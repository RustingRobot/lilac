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

        //ctx.fill(size.to_rect(), &Color::GRAY);
        ctx.stroke(
            Line::new(
                (5.0, size.height - 5.0),
                (size.width - 5.0, size.height - 5.0),
            ),
            &Color::RED,
            1.0,
        );

        //ctx.stroke(size.to_rect(), &Color::WHITE, 1.0);

        let border_width = env.get(theme::TEXTBOX_BORDER_WIDTH);

        let clip_rect = size.to_rect().inset(-0.5);

        ctx.stroke(clip_rect, &Color::RED, border_width);

        ctx.stroke_styled(
            Line::new((2.5, 2.5), (size.width - 2.5, 2.5)),
            &Color::RED,
            1.0,
            &StrokeStyle {
                line_join: None,
                line_cap: Some(LineCap::Square),
                dash: None,
                miter_limit: None,
            },
        );
    }
}
