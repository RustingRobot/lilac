use super::prelude::*;
use druid::WidgetPod;
use tracing::instrument;

pub struct Stylizer<T, W> {
    paint_fn: Option<Box<dyn FnMut(&mut PaintCtx, &T, &Env)>>,
    layout_fn: Option<Box<dyn FnMut(&mut LayoutCtx, &BoxConstraints, &T, &Env) -> Size>>,
    child: WidgetPod<T, W>,
}

impl<T, W: Widget<T>> Stylizer<T, W> {
    pub fn new(
        paint_fn: impl FnMut(&mut PaintCtx, &T, &Env) + 'static,
        layout_fn: impl FnMut(&mut LayoutCtx, &BoxConstraints, &T, &Env) -> Size + 'static,
        child: W,
    ) -> Self {
        Stylizer {
            paint_fn: Some(Box::new(paint_fn)),
            layout_fn: Some(Box::new(layout_fn)),
            child: WidgetPod::new(child),
        }
    }

    pub fn new_paint(paint_fn: impl FnMut(&mut PaintCtx, &T, &Env) + 'static, child: W) -> Self {
        Stylizer {
            paint_fn: Some(Box::new(paint_fn)),
            layout_fn: None,
            child: WidgetPod::new(child),
        }
    }

    pub fn new_layout(
        layout_fn: impl FnMut(&mut LayoutCtx, &BoxConstraints, &T, &Env) -> Size + 'static,
        child: W,
    ) -> Self {
        Stylizer {
            paint_fn: None,
            layout_fn: Some(Box::new(layout_fn)),
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
        match self.layout_fn.as_mut() {
            Some(f) => f(ctx, bc, data, env),
            None => self.child.layout(ctx, bc, data, env),
        }
    }

    #[instrument(name = "Stylizer", level = "trace", skip(self, ctx, data, env))]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        match self.paint_fn.as_mut() {
            Some(f) => f(ctx, data, env),
            None => self.child.paint(ctx, data, env),
        }
        //self.paint_fn.as_mut().unwrap()(ctx, data, env);
    }
}
