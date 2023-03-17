#![windows_subsystem = "windows"]

use std::sync::Arc;

use druid::widget::{Flex, Label, TextBox};
use druid::{
    AppLauncher, Data, Lens, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc,
};
use lilac::styles::serenity::button::Button;

#[derive(Clone, Data, Lens)]
struct AppState {
    multi: Arc<String>,
    single: Arc<String>,
    data: u32,
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder).with_min_size((0.0, 0.0));

    let initial_state = AppState {
        single: "".to_string().into(),
        multi: "".to_string().into(),
        data: 0,
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(initial_state)
}

fn ui_builder() -> impl Widget<AppState> {
    let text = LocalizedString::new("hello-counter")
        .with_arg("count", |data: &AppState, _env| (data.data).into());
    let label = Label::new(text).padding(0.0).center();
    let button = Button::new("increment");
    let txbx: TextBox<String> = TextBox::new().with_placeholder("Type to test clearing");

    Flex::column()
        .with_spacer(10.0)
        .with_child(
            TextBox::new()
                .with_placeholder("Single")
                .lens(AppState::single),
        )
        .with_spacer(10.0)
        .with_child(button)
        .with_spacer(10.0)
}
