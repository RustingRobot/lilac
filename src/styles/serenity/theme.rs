use druid::{theme::TEXT_COLOR, widget::EnvScope, Color, Data, Key, Point, Widget};
use replace_with::replace_with_or_abort;
pub struct TitleBarTheme {
    pub TitleTextColor: Color,
    pub TitleTextShadow: Color,
    pub TitleStripes: Color,
    pub BarColor: [Color; 2],
    pub TitlePos: Point,
    pub BarHeight: f32,
    pub ButtonSize: Point,
    pub IconsOnly: bool,
}

pub struct WindowTheme {
    pub Text: Color,
    pub PlaceholderText: Color,
    pub Accent: Color,
    pub Base: Color,
    pub ThreedHighlight: Color,
    pub ThreedShadow1: Color,
    pub ThreedShadow2: Color,
    pub HoverHighlight: Color,
}

pub struct serenityTheme {
    pub tileBarTheme: TitleBarTheme,
    pub InactiveWindow: TitleBarTheme,
    pub ActiveWindow: WindowTheme,
    pub DisabledWindow: WindowTheme,
}

pub fn default<T: Data, W: Widget<T>>(widget: W) -> EnvScope<T, W> {
    EnvScope::new(
        |env, _| {
            env.set(TEXT_COLOR, Color::rgba8(0x00, 0x00, 0x00, 0xff));
            replace_with_or_abort(env, |env_| {
                env_.adding(BASE, Color::rgba8(0xd4, 0xd0, 0xc8, 0xff))
                    .adding(THREED_HIGHLIGHT, Color::rgba8(0xff, 0xff, 0xff, 0xff))
                    .adding(THREED_SHADOW1, Color::rgba8(0x80, 0x80, 0x80, 0xff))
                    .adding(THREED_SHADOW2, Color::rgba8(0x40, 0x40, 0x40, 0xff))
            });
        },
        widget,
    )
}

pub const BASE: Key<Color> = Key::new("lilac.theme.base");
pub const THREED_HIGHLIGHT: Key<Color> = Key::new("lilac.theme.threed_highlight");
pub const THREED_SHADOW1: Key<Color> = Key::new("lilac.theme.threed_shadow1");
pub const THREED_SHADOW2: Key<Color> = Key::new("lilac.theme.threed_shadow2");

pub const DEFAULT: serenityTheme = serenityTheme {
    tileBarTheme: TitleBarTheme {
        TitleTextColor: Color::rgba8(0xff, 0xff, 0xff, 0xff),
        TitleTextShadow: Color::rgba8(0x42, 0x14, 0x05, 0xff),
        TitleStripes: Color::rgba8(0x6e, 0x22, 0x09, 0xff),
        BarColor: [
            Color::rgba8(0x6e, 0x22, 0x09, 0xff),
            Color::rgba8(0xf4, 0xca, 0x9e, 0xff),
        ],
        TitlePos: Point { x: 10.0, y: 10.0 },
        BarHeight: 20.0,
        ButtonSize: Point { x: 5.0, y: 5.0 },
        IconsOnly: false,
    },
    ActiveWindow: WindowTheme {
        Text: Color::rgba8(0x00, 0x00, 0x00, 0xff),
        PlaceholderText: Color::rgba8(0x80, 0x80, 0x80, 0xff),
        Accent: Color::rgba8(0xab, 0x6e, 0x4a, 0xff),
        Base: Color::rgba8(0xd4, 0xd0, 0xc8, 0xff),
        ThreedHighlight: Color::rgba8(0xff, 0xff, 0xff, 0xff),
        ThreedShadow1: Color::rgba8(0x80, 0x80, 0x80, 0xff),
        ThreedShadow2: Color::rgba8(0x40, 0x40, 0x40, 0xff),
        HoverHighlight: Color::rgba8(0xe3, 0xdf, 0xdb, 0xff),
    },
    InactiveWindow: TitleBarTheme {
        TitleTextColor: Color::rgba8(0xd5, 0xd0, 0xc7, 0xff),
        TitleTextShadow: Color::rgba8(0x4c, 0x4c, 0x4c, 0xff),
        TitleStripes: Color::rgba8(0x80, 0x80, 0x80, 0xff),
        BarColor: [
            Color::rgba8(0x80, 0x80, 0x80, 0xff),
            Color::rgba8(0xc0, 0xc0, 0xc0, 0xff),
        ],
        TitlePos: Point { x: 10.0, y: 10.0 },
        BarHeight: 20.0,
        ButtonSize: Point { x: 5.0, y: 5.0 },
        IconsOnly: false,
    },
    DisabledWindow: WindowTheme {
        Text: Color::rgba8(0x00, 0x00, 0x00, 0xff),
        PlaceholderText: Color::rgba8(0x80, 0x80, 0x80, 0xff),
        Accent: Color::rgba8(0xab, 0x6e, 0x4a, 0xff),
        Base: Color::rgba8(0xa4, 0xa0, 0x98, 0xff),
        ThreedHighlight: Color::rgba8(0xdf, 0xdf, 0xdf, 0xff),
        ThreedShadow1: Color::rgba8(0x50, 0x50, 0x50, 0xff),
        ThreedShadow2: Color::rgba8(0x10, 0x10, 0x10, 0xff),
        HoverHighlight: Color::rgba8(0xa4, 0xa0, 0x98, 0xff),
    },
};
