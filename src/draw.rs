use druid::{
    kurbo::Line,
    piet::{LineCap, StrokeStyle},
    Color, PaintCtx, RenderContext,
};

const STYLE: StrokeStyle = StrokeStyle::new().line_cap(LineCap::Square);

pub fn pixel_line(ctx: &mut PaintCtx, mut from: (f64, f64), mut to: (f64, f64), color: Color) {
    //make sure, that the line is drawn at a x.5 position for a crisp pixel line
    if ctx.window_origin().x.fract() != 5.0 {
        from.0 = from.0.trunc() + 0.5;
        to.0 = to.0.trunc() + 0.5;
    }

    if ctx.window_origin().y.fract() != 5.0 {
        from.1 = from.1.trunc() + 0.5;
        to.1 = to.1.trunc() + 0.5;
    }

    ctx.stroke_styled(Line::new(from, to), &color, 1.0, &STYLE);
}
