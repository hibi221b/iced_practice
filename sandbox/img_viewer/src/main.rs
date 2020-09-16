use iced::{
    Align, Container, Element, Length, 
    Sandbox, Settings, Image, Text,
    Scrollable, scrollable, Column
};

pub fn main() {
    ImageExample::run(Settings::default());
}

#[derive(Default)]
struct ImageExample {
    scroll: scrollable::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
}

impl Sandbox for ImageExample {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Image")
    }

    fn update(&mut self, _message: Self::Message) {
    }

    fn view(&mut self) -> Element<Self::Message> {

        let img = |path| {
            //Cargo.tomlのfeaturesにimageを追加する
            //iced = { version = "0.1.1", features = ["image"] }
            Image::new(path)
                .width(Length::Fill)
                .height(Length::Fill)
        };

        //let mut scrollable_content
        let scrollable_content = Scrollable::new(&mut self.scroll)
            .push(img(&format!("{}/images/sunset.jpeg", env!("CARGO_MANIFEST_DIR"))))
            .push(img(&format!("{}/images/mountain.jpeg", env!("CARGO_MANIFEST_DIR"))))
            .push(img(&format!("{}/images/sky.jpeg", env!("CARGO_MANIFEST_DIR"))))
            .padding(8);

        //スクロールの一番したにスペースを作る
        // scrollable_content = scrollable_content.push(Space::with_height(Length::Units(100)));

        let content = Column::new()
            .push(
                Text::new("image viewer")
                    .size(40)   
            )
            .push(scrollable_content)
            .align_items(Align::Center)
            .padding(24)
            .spacing(24);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}