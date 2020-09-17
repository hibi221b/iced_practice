use iced::{
    Column, Text, Font, HorizontalAlignment,
    Container, Element, Length, Sandbox, Settings, VerticalAlignment, Color
};

pub fn main() {
    let mut settings = Settings::default();
    settings.window.size = (500u32, 500u32);
    CustomButtonExample::run(settings);
}

// icomoon使い方
// https://www.webantena.net/service/iconfont-icomoon/

// zipファイル展開後fontsフォルダにXXX.ttfが入っている
// https://icomoon.io/app/#/select

// https://docs.rs/iced/0.1.1/iced/enum.Font.html
const TWITTER_ICON_FONT: Font = Font::External {
    name: "TwitterIcon",
    bytes: include_bytes!("../fonts/twitter.ttf"),
};

fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string())
        .height(Length::Units(400))
        .width(Length::Units(400))
        .horizontal_alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Center)
        .size(240)
        .color(Color::from_rgb8(29, 161, 242))
        .font(TWITTER_ICON_FONT)
}

fn text_twitter_icon() -> Text {
    icon('\u{ea96}')
}

#[derive(Debug, Default)]
struct CustomButtonExample {
}

#[derive(Debug, Clone)]
enum Message {
}

impl Sandbox for CustomButtonExample {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("")
    }

    fn update(&mut self, _message: Self::Message) {
    }

    fn view(&mut self) -> Element<Self::Message> {

        let content = Column::new()
            .push(text_twitter_icon());

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}