mod nes;

use raylib::prelude::*;

const NTSC_WIDTH: i32 = 256;
const NTSC_HEIGHT: i32 = 224;
const SCALE: i32 = 3;

fn draw_pixel(d: &mut RaylibDrawHandle<'_>, x: i32, y: i32, color: color::Color) {
    d.draw_rectangle(x * SCALE, y * SCALE, SCALE, SCALE, color);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(NTSC_WIDTH * SCALE, NTSC_HEIGHT * SCALE)
        .title("DogNES")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        draw_pixel(&mut d, 0, 0, color::Color::RED);
        draw_pixel(&mut d, 1, 0, color::Color::GREEN);
        draw_pixel(&mut d, 2, 0, color::Color::BLUE);
    }
}
