use druid::{theme::TEXT_COLOR, AppLauncher, Color, Data, Key, Point};
use replace_with::replace_with_or_abort;

pub enum Themes {
    Default,
}

pub fn serenity_theme<T: Data>(theme: Themes, app: AppLauncher<T>) -> AppLauncher<T> {
    match theme {
        Themes::Default => default(app),
    }
}

fn default<T: Data>(app: AppLauncher<T>) -> AppLauncher<T> {
    app.configure_env(|env, _| {
        env.set(TEXT_COLOR, Color::rgba8(0x00, 0x00, 0x00, 0xff));
        env.set(
            druid::theme::WINDOW_BACKGROUND_COLOR,
            Color::rgb8(0xd4, 0xd0, 0xc8),
        );
        replace_with_or_abort(env, |env_| {
            env_.adding(BASE, Color::rgba8(0xd4, 0xd0, 0xc8, 0xff))
                .adding(THREED_HIGHLIGHT, Color::rgba8(0xff, 0xff, 0xff, 0xff))
                .adding(THREED_SHADOW_1, Color::rgba8(0x80, 0x80, 0x80, 0xff))
                .adding(THREED_SHADOW_2, Color::rgba8(0x40, 0x40, 0x40, 0xff))
        });
    })
}
pub const TITLE_TEXT_COLOR: Key<Color> = Key::new("lilac.theme.title_text_color");
pub const TITLE_TEXT_SHADOW: Key<Color> = Key::new("lilac.theme.title_text_shadow");
pub const TITLE_STRIPES: Key<Color> = Key::new("lilac.theme.title_stripes");
pub const BAR_COLOR: Key<(Color, Color)> = Key::new("lilac.theme.bar_color");
pub const TITLE_POS: Key<Point> = Key::new("lilac.theme.title_pos");
pub const BAR_HEIGHT: Key<f32> = Key::new("lilac.theme.bar_height");
pub const BUTTON_SIZE: Key<Point> = Key::new("lilac.theme.button_size");
pub const ICONS_ONLY: Key<bool> = Key::new("lilac.theme.icons_only");

pub const TEXT: Key<Color> = Key::new("lilac.theme.text");
pub const PLACEHOLDER_TEXT: Key<Color> = Key::new("lilac.theme.placeholder_text");
pub const ACCENT: Key<Color> = Key::new("lilac.theme.accent");
pub const BASE: Key<Color> = Key::new("lilac.theme.base");
pub const THREED_HIGHLIGHT: Key<Color> = Key::new("lilac.theme.threed_highlight");
pub const THREED_SHADOW_1: Key<Color> = Key::new("lilac.theme.threed_shadow_1");
pub const THREED_SHADOW_2: Key<Color> = Key::new("lilac.theme.threed_shadow_2");
pub const HOVER_HIGHLIGHT: Key<Color> = Key::new("lilac.theme.hover_highlight");

pub const DISABLED_TEXT: Key<Color> = Key::new("lilac.theme.disabled_text");
pub const DISABLED_PLACEHOLDER_TEXT: Key<Color> = Key::new("lilac.theme.disabled_placeholder_text");
pub const DISABLED_ACCENT: Key<Color> = Key::new("lilac.theme.disabled_accent");
pub const DISABLED_BASE: Key<Color> = Key::new("lilac.theme.disabled_base");
pub const DISABLED_THREED_HIGHLIGHT: Key<Color> = Key::new("lilac.theme.disabled_threed_highlight");
pub const DISABLED_THREED_SHADOW_1: Key<Color> = Key::new("lilac.theme.disabled_threed_shadow_1");
pub const DISABLED_THREED_SHADOW_2: Key<Color> = Key::new("lilac.theme.disabled_threed_shadow_2");
pub const DISABLED_HOVER_HIGHLIGHT: Key<Color> = Key::new("lilac.theme.disabled_hover_highlight");

pub const INACTIVE_TITLE_TEXT_COLOR: Key<Color> = Key::new("lilac.theme.inactive_title_text_color");
pub const INACTIVE_TITLE_TEXT_SHADOW: Key<Color> =
    Key::new("lilac.theme.inactive_title_text_shadow");
pub const INACTIVE_TITLE_STRIPES: Key<Color> = Key::new("lilac.theme.inactive_title_stripes");
pub const INACTIVE_TITLE_BAR_COLOR: Key<(Color, Color)> =
    Key::new("lilac.theme.inactive_title_bar_color");
