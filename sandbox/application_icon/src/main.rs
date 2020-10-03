use iced::{
    Container, Element, Length, Column, Text,
    Sandbox, Settings
};

#[derive(Debug)]
pub struct AppIcon {
    rgba: Option<Vec<u8>>,
    width: Option<u32>,
    height: Option<u32>
}

impl AppIcon {
    pub fn new(icon_bytes: &[u8]) -> Self {
        let (rgba, width, height) = match image::load_from_memory(icon_bytes) {
            Ok(buffer) => {
                let rgba_img = buffer.to_rgba();
                let width = Some(rgba_img.width());
                let height = Some(rgba_img.height());
                let rgba = Some(image::DynamicImage::ImageRgba8(rgba_img).to_bytes());

                (rgba, width, height)
            },
            Err(_) => (None, None, None)
        };

        Self {
            rgba, width, height
        }
    }

    pub fn load_icon(self) -> Option<iced::window::Icon> {
        match iced::window::Icon::from_rgba(self.rgba?, self.width?, self.height?) {
            Ok(icon) => Some(icon),
            Err(_) => None
        }
    }
 }

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.icon = AppIcon::new(include_bytes!("./crab.png")).load_icon();

    AppIconExample::run(settings)
}

#[derive(Default)]
struct AppIconExample {
}

#[derive(Debug, Clone, Copy)]
enum Message {
}

impl Sandbox for AppIconExample {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("app icon example")
    }

    fn update(&mut self, _message: Message) {
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("app icon example"));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}