use iced::{
    Element, Sandbox, Settings, Text, button, Button,
    HorizontalAlignment, Row, Container, Color, Column,
    Length, Align
};

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (480u32, 320u32);

    Counter::run(settings);
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed
}

#[derive(Debug, Default)]
struct Counter {
    vaule: i32,
    increment_button_state: button::State,
    decrement_button_state: button::State
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {
            vaule: 0,
            increment_button_state: button::State::new(),
            decrement_button_state: button::State::new()
        }
    }

    fn title(&self) -> String {
        String::from("Counter App")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::IncrementPressed => self.vaule += 1,
            Message::DecrementPressed => self.vaule -= 1
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        // let count_buttons_row = Row::new()
        //     .push(
        //         Button::new(
        //             &mut self.increment_button_state,
        //             Text::new("Increment")
        //                 .horizontal_alignment(HorizontalAlignment::Center)  
        //         )
        //         .padding(8)
        //         .min_width(80)
        //         .on_press(Message::IncrementPressed)
        //     )
        //     .push(
        //         Button::new(
        //             &mut self.decrement_button_state,
        //             Text::new("Decrement")
        //                 .horizontal_alignment(HorizontalAlignment::Center)
        //         )
        //         .padding(8)
        //         .min_width(80)
        //         .on_press(Message::DecrementPressed)
        //     )
        //     .spacing(32);

        let btn = |state, text| {
            Button::new(
                state,
                Text::new(text)
                    .horizontal_alignment(HorizontalAlignment::Center)
            )
            .padding(8) //ボタンとボタン内の文字列の距離を開ける
            .min_width(80) //ボタンの幅の最小サイズ
        };

        let count_buttons_row = Row::new()
            .push(btn(&mut self.increment_button_state, "Increment").on_press(Message::IncrementPressed))
            .push(btn(&mut self.decrement_button_state, "Decrement").on_press(Message::DecrementPressed))
            .spacing(32);

        let content = Column::new()
            .push(
                Text::new(&self.vaule.to_string())
                    .size(160)
                    .color(Color::from_rgb8(255, 99, 71)) //カウントされる数値の色をオレンジに変更
            )
            .align_items(Align::Center) //数値を中央にセンタリング
            .spacing(32) //数値とボタンのスペースを開ける
            .push(count_buttons_row); //IncrementとDecrementのボタンを追加

        Container::new(content)
            .width(Length::Fill) 
            .height(Length::Fill)
            .center_x() //横軸方向にセンタリング
            .center_y() //縦軸方向にセンタリング
            .into()
    }
}