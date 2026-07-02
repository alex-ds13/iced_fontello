// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/example-icons.toml
// a97e62f97ea2e7b8c39e7b06a062e6128864bd44cdaa773927c8d3fbe6bf07c8
use iced::widget::{Text, text};

pub const FONT: &[u8] = include_bytes!("../fonts/example-icons.ttf");

pub fn edit<'a>() -> Text<'a> {
    icon("\u{270E}")
}

pub fn iced<'a>() -> Text<'a> {
    icon("\u{E800}")
}

pub fn magnifier<'a>() -> Text<'a> {
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
