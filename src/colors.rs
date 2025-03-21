use iced::Color;

pub fn dark_brunet() -> Color {
    let factor = 0.8;
    let base = brunet();
    Color::from_rgb(
        base.r * factor,
        base.g * factor,
        base.b * factor,
    )
}


pub fn brunet() -> Color {
    Color::from_rgb(0.24, 0.22, 0.22)
}
