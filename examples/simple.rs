#![windows_subsystem = "windows"]

use std::sync::Arc;

use druid::widget::{Flex, TextBox};
use druid::{AppLauncher, Data, Lens, PlatformError, Widget, WidgetExt, WindowDesc};
use lilac::styles::serenity::button::Button;

#[derive(Clone, Data, Lens)]
struct AppState {
    single: Arc<String>,
    data: u32,
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());

    let initial_state = AppState {
        single: "".to_string().into(),
        data: 0,
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
}

fn ui_builder() -> impl Widget<AppState> {
    let button = Button::new("increment");

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
