use std::collections::HashMap;
use std::ops::Not;
use hidapi::{HidApi, HidDevice};

const VID: u16 = 10462;
const PID: u16 = 4613;
const DEVICE_PATH: &str = "/dev/hidraw3";

#[derive(Eq, Hash, PartialEq)]
pub enum Button {
    A,
    B,
    X,
    Y,
    UP,
    DOWN,
    LEFT,
    RIGHT,
    LB,
    LT,
    RB,
    RT,
}

struct KeyInfo {
    pub mask: u8,
    pub index: usize,
}

impl KeyInfo {
    pub fn is_pushed(&self, buf: &[u8; 256]) -> bool {
        (self.mask.clone() & buf[self.index.clone()]) > 0
    }
}

pub struct Gamepad {
    device: HidDevice,
    buf1: [u8; 256],
    buf2: [u8; 256],
    buf_flipped: bool,
    key_info_map: HashMap<Button, KeyInfo>,
}


fn _is_pressed(key_info: &KeyInfo, now_buf: &[u8; 256], prev_buf: &[u8; 256]) -> bool {
    !key_info.is_pushed(prev_buf) && key_info.is_pushed(now_buf)
}

// fn _is_pulled(key_info: &KeyInfo, now_buf: &[u8; 256], prev_buf: &[u8; 256]) -> bool {
//     key_info.is_pushed(prev_buf) && !key_info.is_pushed(now_buf)
// }

fn _is_pushed(key_info: &KeyInfo, now_buf: &[u8; 256], _prev_buf: &[u8; 256]) -> bool {
    key_info.is_pushed(now_buf)
}

impl Gamepad {
    pub fn new() -> Self {
        let api = HidApi::new().unwrap();
        let mut target: Option<HidDevice> = None;

        for device in api.device_list() {
            if device.vendor_id() == VID && device.product_id() == PID && device.path().to_str().unwrap() == DEVICE_PATH {
                target = Some(device.open_device(&api).expect("Failed to open device"));
            }
        }

        if target.is_none() {
            panic!("Target device not found.");
        }

        Self {
            device: target.unwrap(),
            buf1: [0u8; 256],
            buf2: [0u8; 256],
            buf_flipped: false,
            key_info_map: vec![
                (Button::A, KeyInfo { mask: 128, index: 8 }),
                (Button::B, KeyInfo { mask: 32, index: 8 }),
                (Button::X, KeyInfo { mask: 64, index: 8 }),
                (Button::Y, KeyInfo { mask: 16, index: 8 }),
                (Button::LB, KeyInfo { mask: 8, index: 8 }),
                (Button::RB, KeyInfo { mask: 4, index: 8 }),
                (Button::LT, KeyInfo { mask: 2, index: 8 }),
                (Button::RT, KeyInfo { mask: 1, index: 8 }),
                (Button::UP, KeyInfo { mask: 1, index: 9 }),
                (Button::DOWN, KeyInfo { mask: 8, index: 9 }),
                (Button::LEFT, KeyInfo { mask: 4, index: 9 }),
                (Button::RIGHT, KeyInfo { mask: 2, index: 9 }),
            ].into_iter().collect(),
        }
    }

    fn _check_button(&self, button: &Button, checker_fn: fn(&KeyInfo, &[u8; 256], &[u8; 256]) -> bool) -> bool {
        let key_info = self.key_info_map.get(button).unwrap();
        let now_buf = if self.buf_flipped {
            &self.buf1
        } else {
            &self.buf2
        };
        let prev_buf = if self.buf_flipped {
            &self.buf2
        } else {
            &self.buf1
        };
        checker_fn(key_info, now_buf, prev_buf)
    }

    pub fn is_pressed(&self, button: &Button) -> bool {
        self._check_button(button, _is_pressed)
    }

    // pub fn is_pulled(&self, button: &Button) -> bool {
    //     self._check_button(button, _is_pulled)
    // }

    pub fn is_pushed(&self, button: &Button) -> bool {
        self._check_button(button, _is_pushed)
    }

    pub fn read(&mut self) {
        self.buf_flipped = self.buf_flipped.not();
        let buf = if self.buf_flipped {
            &mut self.buf1
        } else {
            &mut self.buf2
        };
        let _ = self.device.read(&mut buf[..]);
    }
}