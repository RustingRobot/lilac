use super::prelude::*;
use druid::WidgetPod;
use tracing::instrument;

pub struct Stylizer<T, W> {
    paint_fn: Box<dyn FnMut(&mut PaintCtx, &T, &Env)>,
    child: WidgetPod<T, W>,
}

impl<T, W: Widget<T>> Stylizer<T, W> {
    pub fn new(paint_fn: impl FnMut(&mut PaintCtx, &T, &Env) + 'static, child: W) -> Self {
        Stylizer {
            paint_fn: Box::new(paint_fn),
            child: WidgetPod::new(child),
        }
    }
}

impl<T: Data, W: Widget<T>> Widget<T> for Stylizer<T, W> {
    #[instrument(name = "Stylizer", level = "trace", skip(self, ctx, event, data, env))]
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.child.event(ctx, event, data, env)
    }

    #[instrument(name = "Stylizer", level = "trace", skip(self, ctx, event, data, env))]
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.child.lifecycle(ctx, event, data, env)
    }

    #[instrument(name = "Stylizer", level = "trace", skip(self, ctx, _old, data, env))]
    fn update(&mut self, ctx: &mut UpdateCtx, _old: &T, data: &T, env: &Env) {
        self.child.update(ctx, data, env);
    }

    #[instrument(name = "Stylizer", level = "trace", skip(self, ctx, bc, data, env))]
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.child.layout(ctx, bc, data, env)
    }

    #[instrument(name = "Stylizer", level = "trace", skip(self, ctx, data, env))]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        (self.paint_fn)(ctx, data, env)
    }
}
