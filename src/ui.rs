use iced::{
    Application, Command, Element, Subscription, Size, Length,
    executor, window, theme, subscription, keyboard, Event,
};
use iced::widget::{column, container, svg};

use crate::player::Player;

pub struct Ui {
    display_image: svg::Handle,
    player: Player,
}

#[derive(Debug, Clone)]
pub enum Message {
    First,
    Tick,
    KeyPressed(keyboard::KeyCode),
    KeyReleased(keyboard::KeyCode),
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

        let player = Player::new();

        let startup_cmd = Command::perform(
            sleep_for_first(),
            |_| Message::First,
        );

        (
            Ui {
                display_image,
                player,
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
                self.player.update();
            },
            Message::KeyPressed(key_code) => {
                self.player.gamepad.set_key(key_code);
                self.player.update();
            },
            Message::KeyReleased(key_code) => {
                self.player.gamepad.unset_key(key_code);
                self.player.update();
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
        if self.player.gamepad.exists() {
            iced::time::every(std::time::Duration::from_millis(2))
                .map(|_| Message::Tick)
        } else {
            subscription::events_with(|event, _status| {
                match event {
                    Event::Keyboard(
                        keyboard::Event::KeyPressed {
                            key_code, ..
                        }) => Some(Message::KeyPressed(key_code)),
                    Event::Keyboard(
                        keyboard::Event::KeyReleased {
                          key_code, ..
                        }) => Some(Message::KeyReleased(key_code)),
                    _ => None,
                }
            })
        }
    }
}