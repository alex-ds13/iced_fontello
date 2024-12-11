// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/example-icons.toml
// f4128e932b53ccb52ba93153f13439077cb9e7a031c1a61407f9449dfa949718
use iced::widget::{text, Text};

pub const FONT: &[u8] = include_bytes!("../fonts/example-icons.ttf");

pub fn edit<'a>() -> Text<'a> {
    icon("\u{270E}")
}

pub fn iced<'a>() -> Text<'a> {
    icon("\u{E800}")
}

pub fn iced_logo<'a>() -> Text<'a> {
    icon("\u{E801}")
}

pub fn rust<'a>() -> Text<'a> {
    icon("\u{E802}")
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
