mod device;

use crate::device::{Button, Device};

fn main() {
    let mut device = Device::new();
    device.read();
    let a = device.is_pressed(Button::A);
    let b = device.is_pressed(Button::B);
    println!("A is pressed: {}", a);
    println!("B is pressed: {}", b);
}
