use crate::draw::*;
use crate::styles::prelude::*;
use crate::styles::stylizer::{Stylizer, StylizerHost};
use druid::widget::{self, Click, ControllerHost, Label, LabelText};
use druid::{Affine, Color, Insets};

const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

pub struct Button<T> {
    button: widget::Button<T>,
}

pub struct ButtonStyle;

impl ButtonStyle {
    pub fn new() -> Self {
        ButtonStyle
    }
}

impl<T: Data, W: Widget<T>> Stylizer<T, W> for ButtonStyle {
    fn paint(&mut self, child: &mut W, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {
        let is_active = ctx.is_active() && !ctx.is_disabled();
        let is_hot = ctx.is_hot();
        let size = ctx.size();

        let base_color = Color::rgba8(212, 208, 200, 255);
        let highlight_color = Color::rgba8(255, 255, 255, 255);
        let dark_color = Color::rgba8(128, 128, 128, 255);
        let shadow_color = Color::rgba8(64, 64, 64, 255);

        if ctx.is_hot() {
            ctx.stroke(size.to_rect().inset(-0.5), &Color::WHITE, 1.0);
        }

        ctx.fill(size.to_rect(), &base_color);

        pixel_line(ctx, (1.0, 1.0), (1.0, size.height - 1.0), highlight_color);
        pixel_line(ctx, (1.0, 1.0), (size.width - 1.0, 1.0), highlight_color);
        pixel_line(
            ctx,
            (1.0, size.height - 1.0),
            (size.width, size.height - 1.0),
            dark_color,
        );
        pixel_line(
            ctx,
            (size.width - 1.0, 1.0),
            (size.width - 1.0, size.height),
            dark_color,
        );
        pixel_line(
            ctx,
            (0.0, size.height),
            (size.width, size.height),
            shadow_color,
        );
        pixel_line(
            ctx,
            (size.width, 0.0),
            (size.width, size.height),
            shadow_color,
        );

        //let label_offset = (size.to_vec2() - child.label_size.to_vec2()) / 2.0;

        //ctx.with_save(|ctx| {
        //    ctx.transform(Affine::translate(label_offset));
        //    child.label.paint(ctx, data, env);
        //});
    }
}

impl<T: Data> Button<T> {
    pub fn new(text: impl Into<LabelText<T>>) -> StylizerHost<widget::Button<T>, ButtonStyle> {
        Button::from_label(Label::new(text))
    }

    pub fn from_label(label: Label<T>) -> StylizerHost<widget::Button<T>, ButtonStyle> {
        StylizerHost::new(widget::Button::from_label(label), ButtonStyle::new())
    }

    pub fn dynamic(
        text: impl Fn(&T, &Env) -> String + 'static,
    ) -> StylizerHost<widget::Button<T>, ButtonStyle> {
        let text: LabelText<T> = text.into();
        StylizerHost::new(widget::Button::new(text), ButtonStyle::new())
    }

    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, Click<T>> {
        ControllerHost::new(self, Click::new(f))
    }
}
