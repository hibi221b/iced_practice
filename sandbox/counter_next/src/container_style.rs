use iced::{
    container, Color, Background
};

#[derive(Debug, Clone, Copy)]
pub enum ContainerTheme {
    Standard
}

impl Default for ContainerTheme {
    fn default() -> Self {
        Self::Standard
    }
}

// https://docs.rs/iced/0.1.1/iced/widget/container/trait.StyleSheet.html
impl container::StyleSheet for ContainerTheme {
    fn style(&self) -> container::Style {
        container::Style {
            //背景色
            background: Some(Background::Color(Color::from_rgb8(0, 43, 54))),
            ..container::Style::default()
        }
    }
}