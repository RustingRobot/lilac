use druid::{
    widget::{self, Click, ControllerHost, Label, LabelText},
    Data, Env, EventCtx,
};

use crate::styles::stylizer::{Stylizer, StylizerHost};

use super::border::Border;

pub struct Button<T> {
    button: widget::Button<T>,
}

impl<T: Data> Button<T> {
    pub fn new(text: impl Into<LabelText<T>>) -> StylizerHost<widget::Button<T>, Border> {
        Button::from_label(Label::new(text))
    }

    pub fn from_label(label: Label<T>) -> StylizerHost<widget::Button<T>, Border> {
        StylizerHost::new(widget::Button::from_label(label), Border::new())
    }

    pub fn dynamic(
        text: impl Fn(&T, &Env) -> String + 'static,
    ) -> StylizerHost<widget::Button<T>, Border> {
        let text: LabelText<T> = text.into();
        StylizerHost::new(widget::Button::new(text), Border::new())
    }

    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, Click<T>> {
        ControllerHost::new(self, Click::new(f))
    }
}
