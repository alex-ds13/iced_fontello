mod icon;

use iced::widget::{center, row};
use iced::Element;

fn main() -> iced::Result {
    iced::application("Fontello Icons", Example::update, Example::view)
        .font(icon::FONT)
        .run()
}

#[derive(Default)]
struct Example;

impl Example {
    pub fn update(&mut self, _message: ()) {}

    pub fn view(&self) -> Element<()> {
        center(row![icon::edit(), icon::save(), icon::trash()].spacing(10))
            .padding(20)
            .into()
    }
}
