use std::{cell::RefCell, rc::Rc};

use raylib::prelude::*;

const PAL_WIDTH: i32 = 256;
const PAL_HEIGHT: i32 = 240;
const SCALE: i32 = 1;

const PAL_PALETTE: [Color; 0x40] = [
    Color::new(0, 30, 116, 0),
    Color::new(84, 84, 84, 0),
    Color::new(8, 16, 144, 0),
    Color::new(48, 0, 136, 0),
    Color::new(68, 0, 100, 0),
    Color::new(92, 0, 48, 0),
    Color::new(84, 4, 0, 0),
    Color::new(60, 24, 0, 0),
    Color::new(32, 42, 0, 0),
    Color::new(8, 58, 0, 0),
    Color::new(0, 64, 0, 0),
    Color::new(0, 60, 0, 0),
    Color::new(0, 50, 60, 0),
    Color::new(0, 0, 0, 0),
    Color::new(0, 0, 0, 0),
    Color::new(0, 0, 0, 0),
    Color::new(152, 150, 152, 0),
    Color::new(8, 76, 196, 0),
    Color::new(48, 50, 236, 0),
    Color::new(92, 30, 228, 0),
    Color::new(136, 20, 176, 0),
    Color::new(160, 20, 100, 0),
    Color::new(152, 34, 32, 0),
    Color::new(120, 60, 0, 0),
    Color::new(84, 90, 0, 0),
    Color::new(40, 114, 0, 0),
    Color::new(8, 124, 0, 0),
    Color::new(0, 118, 40, 0),
    Color::new(0, 102, 120, 0),
    Color::new(0, 0, 0, 0),
    Color::new(0, 0, 0, 0),
    Color::new(0, 0, 0, 0),
    Color::new(236, 238, 236, 0),
    Color::new(76, 154, 236, 0),
    Color::new(120, 124, 236, 0),
    Color::new(176, 98, 236, 0),
    Color::new(228, 84, 236, 0),
    Color::new(236, 88, 180, 0),
    Color::new(236, 106, 100, 0),
    Color::new(212, 136, 32, 0),
    Color::new(160, 170, 0, 0),
    Color::new(116, 196, 0, 0),
    Color::new(76, 208, 32, 0),
    Color::new(56, 204, 108, 0),
    Color::new(56, 180, 204, 0),
    Color::new(60, 60, 60, 0),
    Color::new(0, 0, 0, 0),
    Color::new(0, 0, 0, 0),
    Color::new(236, 238, 236, 0),
    Color::new(168, 204, 236, 0),
    Color::new(188, 188, 236, 0),
    Color::new(212, 178, 236, 0),
    Color::new(236, 174, 236, 0),
    Color::new(236, 174, 212, 0),
    Color::new(236, 180, 176, 0),
    Color::new(228, 196, 144, 0),
    Color::new(204, 210, 120, 0),
    Color::new(180, 222, 120, 0),
    Color::new(168, 226, 144, 0),
    Color::new(152, 226, 180, 0),
    Color::new(160, 214, 228, 0),
    Color::new(160, 162, 160, 0),
    Color::new(0, 0, 0, 0),
    Color::new(0, 0, 0, 0),
];

fn draw_pixel(img: &mut Image, x: i32, y: i32, color: color::Color) {
    img.draw_rectangle(x * SCALE, y * SCALE, SCALE, SCALE, color);
}

fn main() {
    let cart = cartridge::Cartridge::new("nestest.nes".to_string());
    let mut nes = cpu::NES::default();
    nes.attach_cart(Rc::new(RefCell::new(cart)));
    nes.reset();

    let mut buf = Image::gen_image_color(PAL_WIDTH * SCALE, PAL_HEIGHT * SCALE, Color::BLACK);

    let (mut rl, thread) = raylib::init()
        .size(PAL_WIDTH * SCALE, PAL_HEIGHT * SCALE)
        .title("DogNES")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        nes.tick();

        let (x, y) = nes.ppu_pos();

        if (0..PAL_WIDTH).contains(&x) && (0..PAL_HEIGHT).contains(&y) {
            draw_pixel(
                &mut buf,
                x,
                y,
                if rand::random::<bool>() {
                    Color::WHITE
                } else {
                    Color::BLACK
                },
            );
        }

        let texture = rl.load_texture_from_image(&thread, &buf).unwrap();
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLUE);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}
