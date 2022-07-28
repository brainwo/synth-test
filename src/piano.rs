use macroquad::prelude::*;

pub struct Piano;

const WHITE_ROW: [u8; 7] = [3, 5, 7, 8, 10, 12, 14];
const BLACK_ROW: [u8; 5] = [4, 6, 9, 11, 13];

const KEY_WIDTH: f32 = 80.;

impl Piano {
    pub fn init() -> Self {
        Piano
    }

    pub fn draw(&self, pressed_key: Option<u8>) {
        for (i, v) in WHITE_ROW.iter().enumerate() {
            draw_rectangle(
                i as f32 * KEY_WIDTH,
                screen_height() / 2.,
                KEY_WIDTH,
                screen_height() / 2.,
                if let Some(n) = pressed_key {
                    if n == *v {
                        LIGHTGRAY
                    } else {
                        WHITE
                    }
                } else {
                    WHITE
                },
            );
            draw_line(
                i as f32 * KEY_WIDTH,
                screen_height() / 2.,
                i as f32 * KEY_WIDTH,
                screen_height(),
                1.,
                BLACK,
            )
        }
        for (i, v) in (0..7)
            .filter(|x| [0, 1, 3, 4, 5].contains(x))
            .zip(BLACK_ROW)
        {
            draw_rectangle(
                (7. / 10. * KEY_WIDTH) + i as f32 * KEY_WIDTH,
                screen_height() / 2.,
                6. / 10. * KEY_WIDTH,
                screen_height() / 4. * 4. / 3.,
                if let Some(n) = pressed_key {
                    if n == v {
                        DARKGRAY
                    } else {
                        BLACK
                    }
                } else {
                    BLACK
                },
            );
        }
    }
}
