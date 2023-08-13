use std::collections::HashMap;
use std::ops::Not;
use hidapi::{HidApi, HidDevice};
use iced::keyboard::KeyCode;

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

    pub fn set_buf(&self, buf: &mut [u8; 256]) {
        buf[self.index.clone()] |= self.mask.clone();
    }

    pub fn unset_buf(&self, buf: &mut [u8; 256]) {
        buf[self.index.clone()] &= !self.mask.clone();
    }
}

pub struct Gamepad {
    device: Option<HidDevice>,
    buf1: [u8; 256],
    buf2: [u8; 256],
    buf_flipped: bool,
    key_info_map: HashMap<Button, KeyInfo>,
    key_map: HashMap<KeyCode, Button>,
}


fn _set_buf(key_info: &KeyInfo, now_buf: &mut [u8; 256]) {
    key_info.set_buf(now_buf);
}

fn _unset_buf(key_info: &KeyInfo, now_buf: &mut [u8; 256]) {
    key_info.unset_buf(now_buf);
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
            println!("Steam controller not found.");
        }

        Self {
            device: target,
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
            key_map: vec![
                (KeyCode::W, Button::UP),
                (KeyCode::A, Button::LEFT),
                (KeyCode::S, Button::DOWN),
                (KeyCode::D, Button::RIGHT),
                (KeyCode::Q, Button::LT),
                (KeyCode::E, Button::LB),
                (KeyCode::I, Button::Y),
                (KeyCode::J, Button::X),
                (KeyCode::K, Button::A),
                (KeyCode::L, Button::B),
                (KeyCode::U, Button::RB),
                (KeyCode::O, Button::RT),
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

    pub fn exists(&self) -> bool {
        self.device.is_some()
    }

    fn _update_key_buf(&mut self, key_code: KeyCode, update_fn: fn(&KeyInfo, &mut [u8; 256])) {
        let button = self.key_map.get(&key_code);
        if button.is_none() {
            return;
        }
        let key_info = self.key_info_map.get(button.unwrap()).unwrap();
        self.buf_flipped = self.buf_flipped.not();
        let new_buf = if self.buf_flipped {
            self.buf1[8] = self.buf2[8].clone();
            self.buf1[9] = self.buf2[9].clone();
            &mut self.buf1
        } else {
            self.buf2[8] = self.buf1[8].clone();
            self.buf2[9] = self.buf1[9].clone();
            &mut self.buf2
        };
        update_fn(key_info, new_buf);
    }

    pub fn set_key(&mut self, key_code: KeyCode) {
        self._update_key_buf(key_code, _set_buf);
    }

    pub fn unset_key(&mut self, key_code: KeyCode) {
        self._update_key_buf(key_code, _unset_buf);
    }

    pub fn read(&mut self) {
        if self.device.is_none() {
            return;
        }
        self.buf_flipped = self.buf_flipped.not();
        let buf = if self.buf_flipped {
            &mut self.buf1
        } else {
            &mut self.buf2
        };
        let _ = self.device.as_mut().unwrap().read(&mut buf[..]);
    }
}