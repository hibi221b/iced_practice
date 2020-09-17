mod styles;

use styles::container::ContainerStyle;
use styles::button::ButtonStyle;

use iced::{
    Element, Sandbox, Settings, Text, button, Button,
    HorizontalAlignment, Row, Container, Column,
    Length, Align, VerticalAlignment, Space, Color
};

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (700u32, 374u32);
    settings.window.resizable = false;
    Calc::run(settings);
}

#[derive(Debug, Default)]
struct ButtonState {
    // 一段目 
    button_l_paren_state: button::State,
    button_r_paren_state: button::State,
    button_delete_state: button::State,
    button_clear_state: button::State,

    // 二段目
    button_7_state: button::State,
    button_8_state: button::State,
    button_9_state: button::State,
    button_div_state: button::State,

    // 三段目
    button_4_state: button::State,
    button_5_state: button::State,
    button_6_state: button::State,
    button_mul_state: button::State,

    // 四段目
    button_1_state: button::State,
    button_2_state: button::State,
    button_3_state: button::State,
    button_sub_state: button::State,

    // 五段目
    button_0_state: button::State,
    button_point_state: button::State,
    button_equal_state: button::State,
    button_plus_state: button::State,
}

#[derive(Debug, Default)]
struct Calc {
    buttons: ButtonState,
    expression: String,
    tmp_message: String,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed(char)
}

fn parser(expression: &str) -> u32 {
    0
}

impl Calc {
    fn push_expression(&mut self, ch: char) {
        //入力範囲を越えたら、Error表示
        //その後、その後Errorを消し、式を入力できる状態にする
        if self.expression.contains("Error") {
            self.expression.clear();
        }

        //電卓の幅を越えるとErrorメッセージを表示する
        if self.expression.len() > 38 {
            self.expression.clear();
            self.expression = "Error : The range is too large".to_string();
        } else {

            self.expression.push(ch);
        }
    }
}

impl Sandbox for Calc {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Calc")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ButtonPressed(ch) => {
                match ch {
                    '=' => {
                        //入力した式をコピ
                        self.tmp_message = self.expression.clone();
                        
                        todo!();
                        let result = parser(&self.expression).to_string();
                        println!("{}", self.expression);

                        //入力された式を削除
                        self.expression.clear();
                        //計算結果を表示する
                        self.expression = result.to_string();
                    }

                    // ← delete button
                    '$' => {
                        if let None = self.expression.pop() {
                            self.expression = String::new();
                        }
                    }

                    // AC all clear
                    '#' => {
                        self.expression.clear();
                        self.tmp_message.clear();
                    }

                    '.' => {
                        // 一番初めのドットの入力禁止
                        if self.expression.len() != 0 {
                            self.push_expression(ch);
                        }
                    }

                    '1'..='9' => {

                        if let Some(n0) = self.expression.chars().nth(0) {
                            //初めの文字が0の時
                            if n0 == '0' {

                                if let Some(n1) = self.expression.chars().nth(1) {
                                
                                    if n1 != '.' {
                                        self.expression.clear();
                                    }

                                } else {
                                    //例えば0が入力されその次に1を入力すると 01ではなく1と表示させる
                                    self.expression.clear();
                                }
                            }
                        }
                        
                        // 1~9を式に追加
                        self.push_expression(ch);
                    }

                    _ => {
                        // + - * / を式に追加
                        self.push_expression(ch);
                    }
                }

            } //Message::ButtonPressed(ch)
        }
    }

    fn view(&mut self) -> Element<Self::Message> {

        let btn = |state, text, width, height, message| {
            Button::new(
                state,
                Text::new(text)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center)
                    .size(24)

            )
            .width(Length::Units(width))
            .height(Length::Units(height))
            .on_press(message)
        };

        const BUTTON_WIDTH: u16 = 160;
        const BUTTON_HEIGHT: u16 = 48;
        const BUTTON_ROW_SPACE: u16 = 8;
        const BUTTON_COL_SPACE: u16 = 8;

        let char_l_paren = Column::new().push(btn(&mut self.buttons.button_l_paren_state, "(", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('(')).style(ButtonStyle::Characters));
        let char_r_paren = Column::new().push(btn(&mut self.buttons.button_r_paren_state, ")", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed(')')).style(ButtonStyle::Characters));
        let char_delete = Column::new().push(btn(&mut self.buttons.button_delete_state, "←", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('$')).style(ButtonStyle::Characters));
        let char_clear   = Column::new().push(btn(&mut self.buttons.button_clear_state, "AC", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('#')).style(ButtonStyle::Characters));

        let row_1 = Row::new()
            .push(char_l_paren)
            .push(char_r_paren)
            .push(char_delete)
            .push(char_clear)
            .spacing(BUTTON_ROW_SPACE);

        let num_7_col = Column::new().push(btn(&mut self.buttons.button_7_state, "7", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('7')).style(ButtonStyle::Numbers));
        let num_8_col = Column::new().push(btn(&mut self.buttons.button_8_state, "8", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('8')).style(ButtonStyle::Numbers));
        let num_9_col = Column::new().push(btn(&mut self.buttons.button_9_state, "9", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('9')).style(ButtonStyle::Numbers));
        let char_div_col = Column::new().push(btn(&mut self.buttons.button_div_state, "/", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('/')).style(ButtonStyle::Operators));

        let row_2 = Row::new()
            .push(num_7_col)
            .push(num_8_col)
            .push(num_9_col)
            .push(char_div_col)
            .spacing(BUTTON_ROW_SPACE);

        let num_4_col = Column::new().push(btn(&mut self.buttons.button_4_state, "4", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('4')).style(ButtonStyle::Numbers));
        let num_5_col = Column::new().push(btn(&mut self.buttons.button_5_state, "5", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('5')).style(ButtonStyle::Numbers));
        let num_6_col = Column::new().push(btn(&mut self.buttons.button_6_state, "6", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('6')).style(ButtonStyle::Numbers));
        let char_mul_col = Column::new().push(btn(&mut self.buttons.button_mul_state, "×", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('*')).style(ButtonStyle::Operators));

        let row_3 = Row::new()
            .push(num_4_col)
            .push(num_5_col)
            .push(num_6_col)
            .push(char_mul_col)
            .spacing(BUTTON_ROW_SPACE);

        let num_1_col = Column::new().push(btn(&mut self.buttons.button_1_state, "1", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('1')).style(ButtonStyle::Numbers));
        let num_2_col = Column::new().push(btn(&mut self.buttons.button_2_state, "2", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('2')).style(ButtonStyle::Numbers));
        let num_3_col = Column::new().push(btn(&mut self.buttons.button_3_state, "3", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('3')).style(ButtonStyle::Numbers));
        let char_sub_col = Column::new().push(btn(&mut self.buttons.button_sub_state, "−", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('-')).style(ButtonStyle::Operators));

        let row_4 = Row::new()
            .push(num_1_col)
            .push(num_2_col)
            .push(num_3_col)
            .push(char_sub_col)
            .spacing(BUTTON_ROW_SPACE);

        let num_0_col  = Column::new().push(btn(&mut self.buttons.button_0_state, "0", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('0')).style(ButtonStyle::Numbers));
        let char_point   = Column::new().push(btn(&mut self.buttons.button_point_state, ".", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('.')).style(ButtonStyle::Numbers));
        let char_equal = Column::new().push(btn(&mut self.buttons.button_equal_state, "=", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('=')).style(ButtonStyle::Characters));
        let char_plus  = Column::new().push(btn(&mut self.buttons.button_plus_state, "+", BUTTON_WIDTH, BUTTON_HEIGHT, Message::ButtonPressed('+')).style(ButtonStyle::Operators));

        let row_5 = Row::new()
            .push(num_0_col)
            .push(char_point)
            .push(char_equal)
            .push(char_plus)
            .spacing(BUTTON_ROW_SPACE);

        
        let button_groups = Column::new()
            .push(row_1)
            .push(row_2)
            .push(row_3)   
            .push(row_4)
            .push(row_5)
            .spacing(BUTTON_COL_SPACE); 

        let expression_col = Column::new()
            .push(Space::with_width(Length::Units(660)))
            .push(
                Text::new(&self.tmp_message)
                    .height(Length::Units(20))
                    .size(16)
                    .color(Color::WHITE)
            )
            .push(
                Text::new(&self.expression)
                    .height(Length::Units(32)) //これがないとexpressionが空の時、電卓の部分が動いてしてしまう
                    .size(32)
                    .color(Color::WHITE)
            )
            .max_width(650)
            .spacing(4)
            .align_items(Align::End);

        let content = Column::new()
            .push(Space::with_height(Length::Units(8)))
            .push(expression_col)
            .push(button_groups)
            .spacing(8);

        Container::new(content)
            .width(Length::Fill) 
            .height(Length::Fill)
            .center_x()
            .style(ContainerStyle::Background)
            .into()
    }
}