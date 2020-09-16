use iced::{
    text_input, Column, Align, HorizontalAlignment, Text,
    Container, Element, Length, Sandbox, Settings, TextInput,
    button, Button
};

pub fn main() {
    let mut settings = Settings::default();
    settings.window.size = (560u32, 320u32);
    settings.window.resizable = false;
    BMI::run(settings);
}

#[derive(Default)]
struct BMI {
    input_height_state: text_input::State,
    input_height_value: String,
    input_weight_state: text_input::State,
    input_weight_value: String,
    button_judge_state: button::State,
    result_value: String
}

#[derive(Debug, Clone)]
enum Message {
    InputHeight(String),
    InputWeight(String),
    JudgeButtonPressed
}

impl Sandbox for BMI {
    type Message = Message;

    fn new() -> Self {
        Self {
            result_value: "calculate your bmi".to_string(),
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("BMI Calculator")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputHeight(value) => self.input_height_value = value,
            Message::InputWeight(value) => self.input_weight_value = value,
            Message::JudgeButtonPressed => {
                if self.input_height_value.parse::<f32>().is_ok() && self.input_weight_value.parse::<f32>().is_ok() {
                    //cmからmになおす
                    let height = self.input_height_value.parse::<f32>().unwrap() / 100.0;
                    let weight = self.input_weight_value.parse::<f32>().unwrap();
                    //bmi = weight(kg) / (height(m) * height(m))
                    let result = weight / (height * height);
                    self.result_value = result.to_string()

                }  else {
                    self.result_value = "unknown".to_string()
                }
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {

        let input_height = Column::new()
            .push(Text::new("Height (cm) :"))
            .push(
                TextInput::new(
                    &mut self.input_height_state, //state
                    "type your height...", //placeholder
                    &self.input_height_value, //value
                    Message::InputHeight //on_change
                )
                .padding(8)
                .size(16)
            )
            .spacing(8);

        let input_weight = Column::new()
            .push(Text::new("Weight (kg) :"))
            .push(
                TextInput::new(
                    &mut self.input_weight_state,
                    "type your weight...",
                    &self.input_weight_value,
                    Message::InputWeight
                )
                .padding(8)
                .size(16)
            )
            .spacing(8);

        let content = Column::new()
            .push(input_height)
            .push(input_weight)
            .push(
                Button::new(
                    &mut self.button_judge_state, 
                    Text::new("Judge")
                        .horizontal_alignment(HorizontalAlignment::Center)
                )
                .padding(8)
                .on_press(Message::JudgeButtonPressed)
            )
            .push(
                Text::new(&self.result_value)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(24)
            )
            .align_items(Align::Center)
            .spacing(24)
            .max_width(240); //conentの幅の最大値

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}