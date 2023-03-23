use crate::draw::*;
use crate::styles::prelude::*;
use crate::styles::stylizer::{Stylizer, StylizerHost};
use druid::widget::{self, Click, ControllerHost, Label, LabelText};
use druid::{Affine, Color, Insets};

const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

pub struct Button<T> {
    label: Label<T>,
    label_size: Size,
}

impl<T: Data> Button<T> {
    pub fn new(text: impl Into<LabelText<T>>) -> Button<T> {
        Button::from_label(Label::new(text))
    }

    pub fn from_label(label: Label<T>) -> Button<T> {
        Button {
            label,
            label_size: Size::ZERO,
        }
    }

    pub fn dynamic(text: impl Fn(&T, &Env) -> String + 'static) -> Self {
        let text: LabelText<T> = text.into();
        Button::new(text)
    }

    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, Click<T>> {
        ControllerHost::new(self, Click::new(f))
    }
}

impl<T: Data> Widget<T> for Button<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                if !ctx.is_disabled() {
                    ctx.set_active(true);
                    ctx.request_paint();
                }
            }
            Event::MouseUp(_) => {
                if ctx.is_active() && !ctx.is_disabled() {
                    ctx.request_paint();
                }
                ctx.set_active(false);
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }
        self.label.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.label.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("Button");
        let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
        let label_bc = bc.shrink(padding).loosen();
        self.label_size = self.label.layout(ctx, &label_bc, data, env);
        // HACK
        let min_height = 24.0;
        let baseline = self.label.baseline_offset();
        ctx.set_baseline_offset(baseline + LABEL_INSETS.y1);

        let button_size = bc.constrain(Size::new(
            self.label_size.width + padding.width,
            (self.label_size.height + padding.height).max(min_height),
        ));
        button_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let is_active = ctx.is_active() && !ctx.is_disabled();
        let is_hot = ctx.is_hot();
        let size = ctx.size();

        let base_color = Color::rgba8(212, 208, 200, 255);
        let highlight_color = Color::rgba8(255, 255, 255, 255);
        let dark_color = Color::rgba8(128, 128, 128, 255);
        let shadow_color = Color::rgba8(64, 64, 64, 255);

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

        if ctx.is_hot() {
            ctx.stroke(size.to_rect().inset(-0.5), &Color::WHITE, 1.0);
        }

        let label_offset = (size.to_vec2() - self.label_size.to_vec2()) / 2.0;

        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(label_offset));
            self.label.paint(ctx, data, env);
        });
    }
}
