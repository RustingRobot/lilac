use druid::{Color, Point};

struct TitleBarTheme {
    TitleTextColor: Color,
    TitleTextShadow: Color,
    TitleStripes: Color,
    BarColor: [Color; 2],
    TitlePos: Point,
    BarHeight: f32,
    ButtonSize: Point,
    IconsOnly: bool,
}

struct WindowTheme {
    Text: Color,
    PlaceholderText: Color,
    Accent: Color,
    Base: Color,
    ThreedHighlight: Color,
    ThreedShadow1: Color,
    ThreedShadow2: Color,
    HoverHighlight: Color,
}

pub struct serenityTheme {
    tileBarTheme: TitleBarTheme,
    InactiveWindow: TitleBarTheme,
    ActiveWindow: WindowTheme,
    DisabledWindow: WindowTheme,
}

const DAFAULT: serenityTheme = serenityTheme {
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
