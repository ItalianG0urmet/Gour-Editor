use iced::Color;

pub fn dark_gray() -> Color {
    let factor = 0.7;
    let base = gray();
    Color::from_rgb(
        base.r * factor,
        base.g * factor,
        base.b * factor,
    )
}

pub fn gray() -> Color {
    Color::from_rgb(0.14, 0.14, 0.14)
}

