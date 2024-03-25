use raylib::prelude::*;

const NTSC_WIDTH: i32 = 256;
const NTSC_HEIGHT: i32 = 224;
const SCALE: i32 = 3;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(NTSC_WIDTH * SCALE, NTSC_HEIGHT * SCALE)
        .title("DogNES")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_rectangle(0, 0, SCALE, SCALE, color::Color::RED);
        d.draw_rectangle(1 * SCALE, 0, SCALE, SCALE, color::Color::GREEN);
        d.draw_rectangle(2 * SCALE, 0, SCALE, SCALE, color::Color::BLUE);
    }
}
