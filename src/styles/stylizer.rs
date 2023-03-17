use super::prelude::*;

//just the controller widget but for paint / layout

pub trait Stylizer<T, W: Widget<T>> {
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

pub struct StylizerHost<W, S> {
    widget: W,
    stylizer: S,
}

impl<W, S> StylizerHost<W, S> {
    pub fn new(widget: W, stylizer: S) -> StylizerHost<W, S> {
        StylizerHost { widget, stylizer }
    }
}

impl<T, W: Widget<T>, S: Stylizer<T, W>> Widget<T> for StylizerHost<W, S> {
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
