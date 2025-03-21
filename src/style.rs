use iced::{Background, Border, Color, Theme};
use iced::widget::button::{Status, Style};


pub fn transparent_style(theme: &Theme, _status: Status) -> Style {
    Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: theme.palette().text,
        border: Border {
            color: Color::TRANSPARENT,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}


