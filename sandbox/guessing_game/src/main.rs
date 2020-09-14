use iced::{
    text_input, Column, Align, HorizontalAlignment, Text,
    Container, Element, Length, Sandbox, Settings, TextInput,
    button, Button
};

use rand::Rng;
use std::cmp::Ordering;

pub fn main() {
    let mut settings = Settings::default();
    settings.window.size = (560u32, 320u32);
    settings.window.resizable = false;
    GuessingGame::run(settings);
}

#[derive(Debug, Default)]
struct GuessingGame {
    reset_button_state: button::State,
    input_guess_state: text_input::State,
    input_guess_value: String,
    //total_countとrandom_numberをStringからu32に変更
    total_count: u32, //プレイ回数
    random_number: u32, //ランダムに生成された値
    result_message: String //予測した値があたったかどうか
}

#[derive(Debug, Clone)]
enum Message {
    InputGuessNumberChanged(String),
    InputGuessOnSubmit, 
    ResetButtonPressed
}

//range of random number
//この例ではランダムな値、1~5を生成する
const GEN_RANGE_LOW: u32 = 1;
const GEN_RANGE_HIGH: u32 = 6;

fn generate_random_value(low: u32, high: u32) -> u32 {
    rand::thread_rng().gen_range(low, high) as u32
}

impl Sandbox for GuessingGame {
    type Message = Message;

    fn new() -> Self {
        Self {
            random_number: generate_random_value(GEN_RANGE_LOW, GEN_RANGE_HIGH),
            result_message: "Guess the Number!".to_string(),
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("Guessing Game")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputGuessNumberChanged(value) => self.input_guess_value = value,

            //予測値を入力後エンターが押されたら
            Message::InputGuessOnSubmit => {
                // - total_count(プレイ回数)を増やす
                self.total_count += 1;

                // - 予測値とランダムな値を比較
                if self.input_guess_value.parse::<u32>().is_ok() {
                    let input_val: u32 = self.input_guess_value.parse().unwrap();

                    let result = match input_val.cmp(&self.random_number) {
                        Ordering::Less => "Too small!",
                        Ordering::Greater => "Too big!",
                        Ordering::Equal => "You win!"
                    };

                    self.result_message = result.to_string();

                } else {
                    self.result_message = "type the number...".to_string();
                }
            }

            Message::ResetButtonPressed => {
                // - 新しくランダムな値をつくる
                self.random_number = generate_random_value(GEN_RANGE_LOW, GEN_RANGE_HIGH);

                // - プレイカウントを0にする
                self.total_count = 0;

                //結果のメッセージを変更
                self.result_message = "Guess the Number!".to_string();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {

        let reset_button_col = Column::new()
            .push(
                Button::new(
                    &mut self.reset_button_state,
                    Text::new("Reset random number")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(16)
                )
                .padding(8)
                .on_press(Message::ResetButtonPressed)
            )
            .padding(16);

        let text_input_col = Column::new()
            .push(
                TextInput::new(
                    &mut self.input_guess_state,
                    "type number and press enter...",
                    &self.input_guess_value,
                    Message::InputGuessNumberChanged
                )
                .padding(8)
                .size(16)
                //https://docs.rs/iced_native/0.2.2/iced_native/widget/text_input/struct.TextInput.html#method.on_submit
                .on_submit(Message::InputGuessOnSubmit)
            )
            .padding(16)
            .max_width(260);

        let play_count_col = Column::new()
            .push(
                Text::new(&format!("Play Count: {}", self.total_count))
            )
            .padding(8);

        let result_col = Column::new()
            .push(
                Text::new(&self.result_message)
                    .size(48)   
            )
            .padding(8);

        let content = Column::new()
            .push(reset_button_col)
            .push(text_input_col)
            .push(play_count_col)
            .push(result_col)
            .align_items(Align::Center);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}