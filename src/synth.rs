use fon::chan::Ch16;
use fon::{Audio, Frame};
use macroquad::audio::{load_sound_from_bytes, play_sound, stop_sound, PlaySoundParams};
use macroquad::prelude::coroutines::start_coroutine;
use macroquad::prelude::debug;
use twang::noise::White;
use twang::ops::Gain;
use twang::osc::Sine;
use twang::Synth;

use crate::wav::convert_wav;

const HARMONICS: [f32; 10] = [
    0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
];

const VOLUME: f32 = 1.0 / 4.0;

fn get_hertz(n: i32) -> f32 {
    debug!("{}", 2f32.powf(n as f32 / 12.) * 440.);

    2f32.powf(n as f32 / 12.) * 220.
}

#[derive(Default, Clone)]
struct Processors {
    white: White,
    piano: [[Sine; 10]; 3],
}

pub struct Synthesizer {
    audio: Audio<Ch16, 2>,
    proc: Processors,
}

impl Synthesizer {
    pub fn new() -> Self {
        let audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 / 2);
        let mut proc = Processors::default();
        for pitch in proc.piano.iter_mut() {
            for harmonic in pitch.iter_mut() {
                harmonic.shift(proc.white.step());
            }
        }
        Synthesizer { audio, proc }
    }

    pub fn play(&mut self, n: i32) {
        let hertz = get_hertz(n);
        let mut synth = Synth::new(self.proc.clone(), move |proc, mut frame: Frame<_, 2>| {
            for (s, pitch) in proc.piano.iter_mut().zip([hertz].iter()) {
                for ((i, o), v) in s.iter_mut().enumerate().zip(HARMONICS.iter()) {
                    let sample = o.step(pitch * (i + 1) as f32);
                    frame = frame.pan(Gain.step(sample, (v * VOLUME).into()), 0.0);
                }
            }
            frame
        });

        synth.stream(self.audio.sink());
        let audio = convert_wav(&self.audio);

        start_coroutine(async move {
            let piano = load_sound_from_bytes(&audio).await.unwrap();

            stop_sound(piano);

            play_sound(
                piano,
                PlaySoundParams {
                    ..Default::default()
                },
            );
        });
    }
}
