use std::collections::HashMap;
use crate::audio::{Audio, Note, Octave, Semitone};
use crate::device::{Button, Device};

pub struct Player {
    device: Device,
    audio: Audio,
    semitone: Semitone,
    octave: Octave,
    note_map: HashMap<Button, Note>,
    semitone_flat: bool,
    semitone_sharp: bool,
    octave_down: bool,
    octave_up: bool,
}

impl Player {
    pub fn new() -> Self {
        let device = Device::new();
        let audio = Audio::new();

        let note_map = vec![
            (Button::LEFT,  Note::C),
            (Button::UP,    Note::D),
            (Button::RIGHT, Note::E),
            (Button::DOWN,  Note::F),
            (Button::X,     Note::G),
            (Button::Y,     Note::A),
            (Button::B,     Note::B),
            (Button::A,     Note::C2),
        ].into_iter().collect();

        Self {
            device,
            audio,
            semitone: Semitone::NONE,
            octave: Octave::NONE,
            note_map,
            semitone_flat: false,
            semitone_sharp: false,
            octave_down: false,
            octave_up: false,
        }
    }

    fn _update_semitone(&mut self) {
        self.semitone_flat = self.device.is_pushed(&Button::LB);
        self.semitone_sharp = self.device.is_pushed(&Button::RB);
        self.semitone = if self.semitone_flat && !self.semitone_sharp {
            Semitone::FLAT
        } else if !self.semitone_flat && self.semitone_sharp {
            Semitone::SHARP
        } else {
            Semitone::NONE
        };
    }

    fn _update_octave(&mut self) {
        self.octave_down = self.device.is_pushed(&Button::LT);
        self.octave_up = self.device.is_pushed(&Button::RT);
        self.octave = if self.octave_down && !self.octave_up {
            Octave::DOWN
        } else if !self.octave_down && self.octave_up {
            Octave::UP
        } else {
            Octave::NONE
        };
    }

    pub fn update(&mut self) {
        self.device.read();
        self._update_semitone();
        self._update_octave();

        for (button, note) in self.note_map.iter() {
            if self.device.is_pressed(button) {
                self.audio.play(note.clone(), self.semitone, self.octave);
            }
        }
    }
}