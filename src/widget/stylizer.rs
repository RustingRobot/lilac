use super::prelude::*;

//just the controller widget but for paint / layout

pub trait Stylized<T, W: Widget<T>> {
    fn layout(
        &mut self,
        child: &mut W,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        child.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, child: &mut W, ctx: &mut PaintCtx, data: &T, env: &Env) {
        child.paint(ctx, data, env)
    }
}

pub struct Stylizer<W, S> {
    widget: W,
    stylizer: S,
}

impl<W, S> Stylizer<W, S> {
    pub fn new(widget: W, stylizer: S) -> Stylizer<W, S> {
        Stylizer { widget, stylizer }
    }
}

impl<T, W: Widget<T>, S: Stylized<T, W>> Widget<T> for Stylizer<W, S> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.widget.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.widget.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.widget.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.stylizer.layout(&mut self.widget, ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.stylizer.paint(&mut self.widget, ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.widget.id()
    }
}
