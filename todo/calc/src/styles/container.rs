use iced::{
    container, Background, Color
};

pub enum ContainerStyle {
    Background
}

// https://docs.rs/iced/0.1.1/iced/widget/container/trait.StyleSheet.html
impl container::StyleSheet for ContainerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::BLACK)),
            ..container::Style::default()
        }
    }   
}