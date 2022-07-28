use crate::piano::Piano;
use macroquad::prelude::*;
use synth::Synthesizer;

mod piano;
mod synth;
mod wav;

#[macroquad::main("Synth")]
async fn main() {
    let mut synthesizer = Synthesizer::new();
    let piano = Piano::init();

    loop {
        let mut pressed_key: Option<u8> = None;

        if is_key_pressed(KeyCode::A) {
            pressed_key = Some(3);
        } else if is_key_pressed(KeyCode::W) {
            pressed_key = Some(4);
        } else if is_key_pressed(KeyCode::S) {
            pressed_key = Some(5);
        } else if is_key_pressed(KeyCode::E) {
            pressed_key = Some(6);
        } else if is_key_pressed(KeyCode::D) {
            pressed_key = Some(7);
        } else if is_key_pressed(KeyCode::F) {
            pressed_key = Some(8);
        } else if is_key_pressed(KeyCode::T) {
            pressed_key = Some(9);
        } else if is_key_pressed(KeyCode::G) {
            pressed_key = Some(10);
        } else if is_key_pressed(KeyCode::Y) {
            pressed_key = Some(11);
        } else if is_key_pressed(KeyCode::H) {
            pressed_key = Some(12);
        } else if is_key_pressed(KeyCode::U) {
            pressed_key = Some(13);
        } else if is_key_pressed(KeyCode::J) {
            pressed_key = Some(14);
        }

        if let Some(n) = pressed_key {
            synthesizer.play(n as i32);
        }
        draw_checkerboard();
        piano.draw(pressed_key);

        next_frame().await
    }
}

pub fn draw_checkerboard() {
    for i in 0..=(screen_width() / 20.) as u32 {
        for j in 0..=(screen_height() / 20.) as u32 {
            draw_rectangle(
                i as f32 * 20. - 10.,
                j as f32 * 20. - 10.,
                20.,
                20.,
                match (i + j) % 2 {
                    0 => Color::from_rgba(43, 46, 51, 255),
                    _ => Color::from_rgba(59, 62, 67, 255),
                },
            )
        }
    }
}
