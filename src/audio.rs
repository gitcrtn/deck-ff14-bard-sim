use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use cpal::{SampleFormat, SampleRate, SupportedStreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use int_enum::IntEnum;

const SAMPLE_RATE: u32 = 44_100;

// ((flat, none, sharp) x 3 octaves) x 8 notes (A442, C4~C7)
const FREQ_TABLE: [usize; 72] = [
    // C
    248, 263, 278, 496, 526, 557, 992, 1051, 1114,
    // D
    278, 295, 313, 557, 590, 625, 1114, 1180, 1250,
    // E
    313, 331, 351, 625, 662, 702, 1250, 1325, 1403,
    // F
    331, 351, 372, 662, 702, 743, 1325, 1403, 1487,
    // G
    372, 394, 417, 743, 788, 834, 1487, 1575, 1669,
    // A
    417, 442, 468, 834, 884, 937, 1669, 1768, 1873,
    // B
    468, 496, 526, 937, 992, 1051, 1873, 1985, 2103,
    // C2
    496, 526, 557, 992, 1051, 1114, 1985, 2103, 2228,
];

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum Note {
    C = 0,
    D = 1,
    E = 2,
    F = 3,
    G = 4,
    A = 5,
    B = 6,
    C2 = 7,
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum Semitone {
    FLAT = 0,
    NONE = 1,
    SHARP = 2,
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum Octave {
    DOWN = 0,
    NONE = 1,
    UP = 2,
}

pub struct Audio {
    mixer: Arc<Mutex<usfx::Mixer>>,
    samples: HashMap<usize, usfx::Sample>,
}

fn gen_sample(freq: usize) -> usfx::Sample {
    let mut sample = usfx::Sample::default();
    sample.osc_frequency(freq);
    sample.osc_type(usfx::OscillatorType::Sine);
    sample.env_attack(0.1);
    sample.env_decay(0.1);
    sample.env_sustain(0.5);
    sample.env_release(0.5);
    sample.dis_crunch(0.2);
    sample
}

impl Audio
{
    pub fn new() -> Self {
        let mixer = Arc::new(Mutex::new(usfx::Mixer::new(SAMPLE_RATE as usize)));
        let stream_mixer = mixer.clone();

        std::thread::spawn(move || {
            let host = cpal::default_host();

            let device = host
                .default_output_device()
                .expect("failed to find a default output device");

            let config = device
                .supported_output_configs()
                .expect("no output configs available")
                .find(|config| config.sample_format() == SampleFormat::F32);

            if config.is_none() {
                panic!("no F32 config available");
            }

            let config = config.unwrap();

            if config.min_sample_rate() > SampleRate(SAMPLE_RATE)
                || config.max_sample_rate() < SampleRate(SAMPLE_RATE)
            {
                panic!("44100 Hz not supported");
            }

            let format = SupportedStreamConfig::new(
                config.channels(),
                SampleRate(SAMPLE_RATE),
                config.buffer_size().clone(),
                SampleFormat::F32,
            );

            let stream = device
                .build_output_stream::<f32, _, _>(
                    &format.config(),
                    move |data, _| stream_mixer.lock().unwrap().generate(data),
                    |err| eprintln!("cpal error: {:?}", err),
                )
                .expect("could not build output stream");

            stream.play().unwrap();

            // Park the thread so out noise plays continuously until the app is closed
            std::thread::park();
        });

        let mut samples: HashMap<usize, usfx::Sample> = HashMap::new();

        for freq in FREQ_TABLE.iter() {
            if !samples.contains_key(freq){
                let sample = gen_sample(freq.clone());
                samples.insert(freq.clone(), sample);
            }
        }

        Self {
            mixer,
            samples,
        }
    }

    pub fn play(&mut self, note: Note, semitone: Semitone, octave: Octave) {
        let freq = self.get_freq(note, semitone, octave);
        let sample = self.samples.get(&freq).unwrap();
        self.mixer.lock().unwrap().play(*sample);
    }

    fn get_freq(&self, note: Note, semitone: Semitone, octave: Octave) -> usize {
        let freq_index: usize = 9 * note.int_value() + 3 * octave.int_value() + semitone.int_value();
        FREQ_TABLE[freq_index].clone()
    }
}
