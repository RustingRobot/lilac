// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of a custom drawing widget.
//! We draw an image, some text, a shape, and a curve.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use druid::piet::{ImageFormat, InterpolationMode};
use druid::widget::{prelude::*, Click, ControllerHost, Label, LabelText};
use druid::EventCtx;
use druid::{theme, AppLauncher, Insets, LocalizedString, Point, Rect, WindowDesc};

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

    /// Provide a closure to be called when this button is clicked.
    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, Click<T>> {
        ControllerHost::new(self, Click::new(f))
    }
}
// If this widget has any child widgets it should call its event, update and layout
// (and lifecycle) methods as well to make sure it works. Some things can be filtered,
// but a general rule is to just pass it through unless you really know you don't want it.
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
        // HACK: to make sure we look okay at default sizes when beside a textbox,
        // we make sure we will have at least the same height as the default textbox.
        let min_height = env.get(theme::BORDERED_WIDGET_HEIGHT);
        let baseline = self.label.baseline_offset();
        ctx.set_baseline_offset(baseline + LABEL_INSETS.y1);

        let button_size = bc.constrain(Size::new(
            self.label_size.width + padding.width,
            (self.label_size.height + padding.height).max(min_height),
        ));
        button_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = ctx.size();
        let img = make_outline(size.width as usize, size.height as usize);
        let image = ctx
            .make_image(
                size.width as usize,
                size.height as usize,
                &img,
                ImageFormat::RgbaSeparate,
            )
            .unwrap();

        // When piet draws our image it will stretch it automatically.
        // We'll fix this later by giving our widget a fixed size.
        ctx.draw_image(
            &image,
            Rect::from_origin_size(Point::ORIGIN, size),
            InterpolationMode::Bilinear,
        );

        for x in 0..1 {
            for y in 0..1 {
                ctx.draw_image(
                    &image,
                    Rect::from_origin_size(
                        (
                            Point::ORIGIN.x + (size.width * x as f64),
                            Point::ORIGIN.y + size.height * y as f64,
                        ),
                        size,
                    ),
                    InterpolationMode::Bilinear,
                );
            }
        }
    }
}

pub fn main() {
    let window = WindowDesc::new(build_ui).title(LocalizedString::new("Fancy Colors"));
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(AppState {})
        .expect("launch failed");
}

fn make_outline(width: usize, height: usize) -> Vec<u8> {
    let base_color = (212, 208, 200, 255);
    let highlight_color = (255, 255, 255, 255);
    let dark_color = (128, 128, 128, 255);
    let shadow_color = (64, 64, 64, 255);
    let mut color = base_color;

    let mut image_data = vec![0; width * height * 4];

    for y in 0..height {
        for x in 0..width {
            let i = (y * width + x) * 4;

            if x == width - 1 || y == height - 1 {
                color = shadow_color
            } else if x >= 1 && y == height - 2 || x == width - 2 && y >= 1 {
                color = dark_color
            } else if x >= 1 && y == 1 || x == 1 && y >= 1 {
                color = highlight_color
            } else {
                color = base_color
            }

            image_data[i + 0] = color.0;
            image_data[i + 1] = color.1;
            image_data[i + 2] = color.2;
            image_data[i + 3] = color.3;
        }
    }
    image_data
}

#[derive(Clone, Data)]
struct AppState {}

fn build_ui() -> impl Widget<AppState> {
    Button::new("test")
}
