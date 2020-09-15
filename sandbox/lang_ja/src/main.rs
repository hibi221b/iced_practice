use iced::{
    text_input, Column, Align, Text,
    Container, Element, Length, Sandbox, Settings, TextInput,
};

pub fn main() {
    let mut settings = Settings::default();

    // https://ipafont.ipa.go.jp/old/ipafont/download.html
    // macでは表示することができました。windows, linuxで表示できるかは確認していません。
    settings.default_font = Some(include_bytes!("../fonts/ipamp.ttf"));
    CustomFontExample::run(settings);
}

#[derive(Debug, Default)]
struct CustomFontExample {
    text_input_state: text_input::State,
    text_input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
}

impl Sandbox for CustomFontExample {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Lang ja")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => self.text_input_value = value,
        }
    }

    fn view(&mut self) -> Element<Message> {

        let text_input_col = Column::new()
            .push(
                TextInput::new(
                    &mut self.text_input_state,
                    "日本語も入力できます",
                    &self.text_input_value,
                    Message::InputChanged
                )
                .padding(8)
                .size(16)
            )
            .padding(16)
            .max_width(260);

        let result_col = Column::new()
            .push(
                Text::new(&format!("test: {}", self.text_input_value))
                    .size(48)   
            )
            .padding(8);

        let content = Column::new()
            .push(text_input_col)
            .push(result_col)
            .spacing(24)
            .align_items(Align::Center);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}