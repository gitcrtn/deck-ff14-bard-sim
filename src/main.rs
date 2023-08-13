mod device;
mod ui;
mod player;
mod audio;

use iced::{Application, Settings};
use crate::ui::Ui;

fn main() -> iced::Result {
    Ui::run(Settings::default())
}
