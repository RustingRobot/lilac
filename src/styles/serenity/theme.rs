use druid::{Color, Point};

struct TitleBarTheme {
    TitleTextColor: Color,
    TitleTextShadow: Color,
    TitleStripes: Color,
    BarColor: [Color; 2],
    TitlePos: Point,
    BarHeight: f32,
    ButtonSize: Point,
    ButtonIcons: i32, //what to do
    IconsOnly: bool,
}

struct WindowTheme {
    Shadow: Color,
    ShadowDistance: f32,
    ShadowSpread: f32,
    Background: Color,
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
    customTitleBar: bool,
    tileBarTheme: TitleBarTheme,
    ActiveWindow: WindowTheme,
    InactiveWindow: WindowTheme,
    DisabledWindow: WindowTheme,
    BorderThickness: f32,
    TooltipBase: Color,
    TooltipText: Color,
    TooltipBorder: Color,
}
