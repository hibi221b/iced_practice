use iced::{
    button, Background, Color, Vector
};

#[derive(Debug, Clone, Copy)]
pub enum ButtonStyle {
    Numbers, //1, 2, 3, 4...
    Operators, //+, -, *, /
    Characters, //=
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self::Numbers
    }
}

impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                ButtonStyle::Numbers => Color::from_rgb8(50, 50, 50),
                ButtonStyle::Operators => Color::from_rgb8(251, 140, 0),
                ButtonStyle::Characters => Color::from_rgb8(95, 106, 106)
            })),
            border_radius: 4,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                ButtonStyle::Numbers => Color::from_rgb8(101, 101, 101),
                ButtonStyle::Operators => Color::from_rgb8(255, 183, 77),
                ButtonStyle::Characters => Color::from_rgb8(158, 158, 158)
            })),
            border_radius: 4,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    }
}