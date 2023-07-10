use crate::draw::*;
use crate::styles::prelude::*;
use druid::widget::{Click, ControllerHost, Label, LabelText};
use druid::{Affine, Insets, Vec2};

use super::theme;

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
        //let is_hot = ctx.is_hot();
        let size = ctx.size();

        let base_color = env.get(theme::BASE);
        let highlight_color = env.get(theme::THREED_HIGHLIGHT);
        let dark_color = env.get(theme::THREED_SHADOW_1);
        let shadow_color = env.get(theme::THREED_SHADOW_2);

        let positions;
        if is_active {
            positions = [
                (1.0, size.height - 2.0),
                (size.width - 2.0, 1.0),
                (size.width - 2.0, size.height - 2.0),
                (1.0, 1.0),
                (0.0, size.height - 1.0),
                (size.width - 1.0, 0.0),
                (0.0, 0.0),
            ];
        } else {
            positions = [
                (1.0, size.height - 2.0),
                (size.width - 2.0, 1.0),
                (1.0, 1.0),
                (size.width - 2.0, size.height - 2.0),
                (0.0, size.height - 1.0),
                (size.width - 1.0, 0.0),
                (size.width - 1.0, size.height - 1.0),
            ];
        }

        ctx.fill(size.to_rect(), &base_color);
        pixel_line(ctx, positions[0], positions[2], highlight_color);
        pixel_line(ctx, positions[1], positions[2], highlight_color);
        pixel_line(ctx, positions[0], positions[3], dark_color);
        pixel_line(ctx, positions[1], positions[3], dark_color);
        pixel_line(ctx, positions[4], positions[6], shadow_color);
        pixel_line(ctx, positions[5], positions[6], shadow_color);

        let mut label_offset = (size.to_vec2() - self.label_size.to_vec2()) / 2.0;
        if is_active {
            label_offset += Vec2::new(1.0, 1.0);
        }

        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(label_offset));
            self.label.paint(ctx, data, env);
        });
    }
}
