use iced::{
    button, Background, Color, Vector
};

#[derive(Debug, Clone, Copy)]
pub enum CountButtonTheme {
    Primary 
}

impl Default for CountButtonTheme {
    fn default() -> Self {
        Self::Primary
    }
}

// https://docs.rs/iced/0.1.1/iced/widget/button/trait.StyleSheet.html
impl button::StyleSheet for CountButtonTheme {
    //常時
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(131,148,150))),
            border_radius: 4,
            text_color: Color::BLACK,
            ..button::Style::default()
        }
    }

    //カーソルがボタンの上をhoverした時
    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(88, 110, 117))),
            text_color: Color::WHITE,
            shadow_offset: Vector::new(5.0, -5.0), //影を上側と右側に表示させる
            ..self.active()
        }
    }
}