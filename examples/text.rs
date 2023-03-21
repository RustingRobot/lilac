// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use std::sync::Arc;

use druid::widget::{Flex, Label, TextBox};
use druid::{
    AppLauncher, Color, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc, WindowId,
};

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Text Options");

const EXPLAINER: &str = "\
    This example demonstrates some of the possible configurations \
    of the TextBox widget.\n\
    The top textbox allows a single line of input, with horizontal scrolling \
    but no scrollbars. The bottom textbox allows multiple lines of text, wrapping \
    words to fit the width, and allowing vertical scrolling when it runs out \
    of room to grow vertically.";

#[derive(Clone, Data, Lens)]
struct AppState {
    multi: Arc<String>,
    single: Arc<String>,
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 600.0));

    // create the initial app state
    let initial_state = AppState {
        single: "".to_string().into(),
        multi: "".to_string().into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
        .with_spacer(24.0)
        .with_child(
            TextBox::new()
                .with_placeholder("Single")
                .lens(AppState::single),
        )
}
