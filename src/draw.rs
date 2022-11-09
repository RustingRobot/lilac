use piet_common::{kurbo, Color, Error, LineCap, Piet, RenderContext, StrokeStyle};

pub struct RenderBackend<'a, 'b> {
    pub size: (u32, u32),
    pub render_ctx: &'a mut Piet<'b>,
}

impl<'a, 'b> std::fmt::Debug for RenderBackend<'a, 'b> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RenderBackend")
            .field("size", &self.size)
            .finish()
    }
}

impl<'a, 'b> RenderBackend<'a, 'b> {
    fn get_size(&self) -> (u32, u32) {
        self.size
    }

    fn present(&mut self) -> Result<(), Error> {
        self.render_ctx.finish()
    }

    fn draw_pixel(&mut self, point: (i32, i32), color: Color) {
        let x = point.0 as f64;
        let y = point.1 as f64;
        self.render_ctx
            .fill(kurbo::Rect::new(x, y, x + 1., y + 1.), &color);
    }

    fn draw_line(&mut self, from: (f64, f64), to: (f64, f64), color: Color, width: f64) {
        self.render_ctx.stroke_styled(
            kurbo::Line::new(from, to),
            &color,
            width,
            &StrokeStyle {
                line_join: None,
                line_cap: Some(LineCap::Square),
                dash: None,
                miter_limit: None,
            },
        );
    }

    fn draw_rect(
        &mut self,
        upper_left: (f64, f64),
        mut bottom_right: (f64, f64),
        color: Color,
        width: u32,
        fill: bool,
    ) {
        if fill {
            bottom_right.0 += 1.;
            bottom_right.1 += 1.;
            let rect = kurbo::Rect::new(upper_left.0, upper_left.1, bottom_right.0, bottom_right.1);

            self.render_ctx.fill(rect, &color);
        } else {
            let rect = kurbo::Rect::new(upper_left.0, upper_left.1, bottom_right.0, bottom_right.1);

            self.render_ctx.stroke(rect, &color, width as f64);
        }
    }

    fn draw_circle(
        &mut self,
        center: (f64, f64),
        radius: f64,
        color: Color,
        width: f64,
        fill: bool,
    ) {
        let circle = kurbo::Circle::new(center, radius);

        if fill {
            self.render_ctx.fill(circle, &color);
        } else {
            self.render_ctx.stroke(circle, &color, width);
        }
    }
}
