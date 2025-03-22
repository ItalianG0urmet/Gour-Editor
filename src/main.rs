mod window;
mod style;
mod view;
mod colors;

use iced::{Result, Theme};

fn main() -> Result {
    iced::application("Gour - Editor", window::update, view::view).theme(theme).run()
}

fn theme(state: &window::State) -> Theme {
    Theme::KanagawaDragon
}




