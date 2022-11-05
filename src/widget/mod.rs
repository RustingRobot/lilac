//pub mod button;
pub mod stylizer;

pub mod prelude {
    #[doc(hidden)]
    pub use druid::{
        BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
        RenderContext, Size, UpdateCtx, Widget, WidgetId,
    };
}

#[cfg(test)]
mod tests {

    pub struct Test<T> {
        paint_fn: Option<Box<dyn FnMut(&mut String, &T)>>,
    }

    impl<T> Test<T> {
        fn new(func: impl FnMut(&mut String, &T) + 'static) -> Self {
            Test {
                paint_fn: Some(Box::new(func)),
            }
        }
    }

    #[test]
    fn main() {
        let testing = Test::new(|a: &mut String, t: &bool| println!("test: {a} test2: {t}"));
        println!("test start:");
        testing.paint_fn.unwrap()(&mut "this is a test".to_string(), &true);
    }
}
