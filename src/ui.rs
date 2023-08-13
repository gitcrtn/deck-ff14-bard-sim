use iced::{
    Application, Command, Element, Subscription, Size, Length,
    executor, window, theme,
};
use iced::widget::{column, container, svg};

use crate::device::{Button, Device};

pub struct Ui {
    display_image: svg::Handle,
    device: Device,
}

#[derive(Debug, Clone)]
pub enum Message {
    First,
    Tick,
}

async fn sleep_for_first() {
    use async_std::task::sleep;
    use std::time::Duration;

    sleep(Duration::from_millis(300)).await;
}

impl Application for Ui {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let display_image = svg::Handle::from_memory(
            include_bytes!("../images/pad.svg").to_vec());

        let mut device = Device::new();
        device.read();
        let a = device.is_pressed(Button::A);
        let b = device.is_pressed(Button::B);
        println!("A is pressed: {}", a);
        println!("B is pressed: {}", b);

        let startup_cmd = Command::perform(
            sleep_for_first(),
            |_| Message::First,
        );

        (
            Ui {
                display_image,
                device,
            },
            Command::batch([
                startup_cmd,
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("ff14 bard simulator")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::First => {
                return window::resize(Size::new(
                    2000,
                    2000,
                ));
            },
            Message::Tick => {
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        container(
            column![
                svg(self.display_image.clone())
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::Svg::Default),
            ]
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(10))
            .map(|_| Message::Tick)
    }
}