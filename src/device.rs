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
    pub fn is_pressed(&self, buf: &[u8; 256]) -> bool {
        (self.mask.clone() & buf[self.index.clone()]) > 0
    }
}

pub struct Device {
    target: HidDevice,
    buf1: [u8; 256],
    buf2: [u8; 256],
    buf_flipped: bool,
    key_info_map: HashMap<Button, KeyInfo>,
}

impl Device {
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
            target: target.unwrap(),
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

    pub fn is_pressed(&self, button: Button) -> bool {
        let key_info = self.key_info_map.get(&button).unwrap();
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
        !key_info.is_pressed(prev_buf) && key_info.is_pressed(now_buf)
    }

    pub fn read(&mut self) {
        self.buf_flipped = self.buf_flipped.not();
        let buf = if self.buf_flipped {
            &mut self.buf1
        } else {
            &mut self.buf2
        };
        let _ = self.target.read(&mut buf[..]);
    }
}