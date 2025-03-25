mod filemanager;
mod frame;

use iced::{Result, Theme};
use crate::frame::{view, window};

fn main() -> Result {
    iced::application("Gour - Editor", window::update, view::view).theme(theme).run()
}

fn theme(state: &window::State) -> Theme {
    Theme::KanagawaDragon
}