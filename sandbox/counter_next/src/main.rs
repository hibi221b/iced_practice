mod button_style;
mod container_style;

use button_style::CountButtonTheme;
use container_style::ContainerTheme;

use iced::{
    Element, Sandbox, Settings, Text, button, Button,
    HorizontalAlignment, Row, Container, Color, Column,
    Length, Align
};

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (560u32, 320u32);
    settings.window.resizable = false; //リサイズ禁止
    Counter::run(settings);
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed
}

#[derive(Debug, Default)]
struct Counter {
    theme: CountButtonTheme,
    vaule: i32,
    increment_button_state: button::State,
    decrement_button_state: button::State
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter App Next")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::IncrementPressed => self.vaule += 1,
            Message::DecrementPressed => self.vaule -= 1
        }
    }

    fn view(&mut self) -> Element<Self::Message> {

        let btn = |state, text| {
            Button::new(
                state,
                Text::new(text)
                    .size(40)
                    .horizontal_alignment(HorizontalAlignment::Center)
            )
            .min_width(80) //ボタンの幅の最小サイズ
        };

        let count_buttons_row = Row::new()
            .push(btn(&mut self.increment_button_state, "+").style(self.theme).on_press(Message::IncrementPressed))
            .push(btn(&mut self.decrement_button_state, "-").style(self.theme).on_press(Message::DecrementPressed))
            .spacing(48);

        let content = Column::new()
            .push(
                Text::new(&self.vaule.to_string())
                    .size(160)
                    .color(Color::from_rgb8(238, 232, 213)) //カウントされる数値の色をオレンジに変更
            )
            .align_items(Align::Center) 
            .spacing(32) //数値とボタンのスペースを開ける
            .push(count_buttons_row); //IncrementとDecrementのボタンを追加

        Container::new(content)
            .width(Length::Fill) 
            .height(Length::Fill)
            .center_x() //横軸方向にセンタリング
            .center_y() //縦軸方向にセンタリング
            .style(ContainerTheme::Standard)  //直接カスタムスタイルを指定
            .into()
    }
}