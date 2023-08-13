mod device;
mod ui;

use iced::{Application, Settings};
use crate::ui::Ui;

fn main() -> iced::Result {
    Ui::run(Settings::default())
}
