use iced::{Background, Border, Color, Theme};
use iced::widget::button::{Status, Style};
use crate::colors;

pub fn transparent_style(theme: &Theme, _status: Status) -> Style {
    Style {
        background: Some(Background::Color(colors::dark_brunet())),
        text_color: theme.palette().text,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn transparent_text_editor_style(theme: &Theme, _status: iced::widget::text_editor::Status) -> iced::widget::text_editor::Style {
    iced::widget::text_editor::Style {
        background: Background::Color(colors::brunet()),
        border: Border {
            color: Color::TRANSPARENT,
            width: 5.0,
            radius: 0.5.into(),
        },
        icon: Default::default(),
        placeholder: Color::WHITE,
        value: Color::WHITE,
        selection: Default::default(),
    }
}


