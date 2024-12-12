// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/example-icons.toml
// 1977ed694f963366e3d65fb47eda9896c38d2b6d11620a0c29c91ce41a31f9c0
use iced::widget::{Text, text};

pub const FONT: &[u8] = include_bytes!("../fonts/example-icons.ttf");

pub fn edit<'a>() -> Text<'a> {
    icon("\u{270E}")
}

pub fn iced<'a>() -> Text<'a> {
    icon("\u{E800}")
}

pub fn rust<'a>() -> Text<'a> {
    icon("\u{E801}")
}

pub fn save<'a>() -> Text<'a> {
    icon("\u{1F4BE}")
}

pub fn trash<'a>() -> Text<'a> {
    icon("\u{E10A}")
}

fn icon(codepoint: &str) -> Text<'_> {
    text(codepoint).font("example-icons")
}
