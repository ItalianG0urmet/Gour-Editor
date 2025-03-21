mod window;
mod style;
mod colors;

use iced::{Result, Theme};

fn main() -> Result {
    iced::application("Editor", window::update, window::view).theme(theme).run()
}

fn theme(state: &window::State) -> Theme {
    Theme::Ferra
}




