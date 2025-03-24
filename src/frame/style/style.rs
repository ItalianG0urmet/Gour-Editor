use iced::{Background, Border, Color, Theme};
use iced::widget::button::{Status, Style};
use crate::frame::style::colors;

pub fn directory_button_style(theme: &Theme, _status: Status) -> Style {
    Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: theme.palette().text,
        border: Border {
            color: Color::TRANSPARENT,
            width: 1.0,
            radius: 2.0.into(),
        },
        ..Default::default()
    }
}
pub fn button_style(theme: &Theme, _status: Status) -> Style {
    Style {
        background: Some(Background::Color(colors::gray())),
        text_color: theme.palette().text,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn sub_button_style(theme: &Theme, _status: Status) -> Style {
    Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: theme.palette().text,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn button_active_style(theme: &Theme, _status: Status) -> Style {
    Style {
        background: Some(Background::Color(colors::dark_gray())),
        text_color: theme.palette().text,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}
pub fn text_editor_style(theme: &Theme, _status: iced::widget::text_editor::Status) -> iced::widget::text_editor::Style {
    iced::widget::text_editor::Style {
        background: Background::Color(colors::gray()),
        border: Border {
            color: Color::BLACK,
            width: 2.8,
            radius: 0.5.into(),
        },
        icon: Default::default(),
        placeholder: Color::WHITE,
        value: Color::WHITE,
        selection: colors::blue(),
    }
}


