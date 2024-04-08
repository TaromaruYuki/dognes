use std::{cell::RefCell, collections::HashMap, rc::Rc};

use raylib::prelude::*;

const PAL_WIDTH: i32 = 256;
const PAL_HEIGHT: i32 = 240;
const SCALE: i32 = 1;

const PAL_PALETTE: [Color; 0x40] = [
    Color::new(0, 30, 116, 255),
    Color::new(84, 84, 84, 255),
    Color::new(8, 16, 144, 255),
    Color::new(48, 0, 136, 255),
    Color::new(68, 0, 100, 255),
    Color::new(92, 0, 48, 255),
    Color::new(84, 4, 0, 255),
    Color::new(60, 24, 0, 255),
    Color::new(32, 42, 0, 255),
    Color::new(8, 58, 0, 255),
    Color::new(0, 64, 0, 255),
    Color::new(0, 60, 0, 255),
    Color::new(0, 50, 60, 255),
    Color::new(0, 0, 0, 255),
    Color::new(0, 0, 0, 255),
    Color::new(0, 0, 0, 255),
    Color::new(152, 150, 152, 255),
    Color::new(8, 76, 196, 255),
    Color::new(48, 50, 236, 255),
    Color::new(92, 30, 228, 255),
    Color::new(136, 20, 176, 255),
    Color::new(160, 20, 100, 255),
    Color::new(152, 34, 32, 255),
    Color::new(120, 60, 0, 255),
    Color::new(84, 90, 0, 255),
    Color::new(40, 114, 0, 255),
    Color::new(8, 124, 0, 255),
    Color::new(0, 118, 40, 255),
    Color::new(0, 102, 120, 255),
    Color::new(0, 0, 0, 255),
    Color::new(0, 0, 0, 255),
    Color::new(0, 0, 0, 255),
    Color::new(236, 238, 236, 255),
    Color::new(76, 154, 236, 255),
    Color::new(120, 124, 236, 255),
    Color::new(176, 98, 236, 255),
    Color::new(228, 84, 236, 255),
    Color::new(236, 88, 180, 255),
    Color::new(236, 106, 100, 255),
    Color::new(212, 136, 32, 255),
    Color::new(160, 170, 0, 255),
    Color::new(116, 196, 0, 255),
    Color::new(76, 208, 32, 255),
    Color::new(56, 204, 108, 255),
    Color::new(56, 180, 204, 255),
    Color::new(60, 60, 60, 255),
    Color::new(0, 0, 0, 255),
    Color::new(0, 0, 0, 255),
    Color::new(236, 238, 236, 255),
    Color::new(168, 204, 236, 255),
    Color::new(188, 188, 236, 255),
    Color::new(212, 178, 236, 255),
    Color::new(236, 174, 236, 255),
    Color::new(236, 174, 212, 255),
    Color::new(236, 180, 176, 255),
    Color::new(228, 196, 144, 255),
    Color::new(204, 210, 120, 255),
    Color::new(180, 222, 120, 255),
    Color::new(168, 226, 144, 255),
    Color::new(152, 226, 180, 255),
    Color::new(160, 214, 228, 255),
    Color::new(160, 162, 160, 255),
    Color::new(0, 0, 0, 255),
    Color::new(0, 0, 0, 255),
];

fn main() {
    let cart = cartridge::Cartridge::new("nestest.nes".to_string());
    let mut nes = cpu::NES::default();
    nes.attach_cart(Rc::new(RefCell::new(cart)));
    nes.reset();

    loop {
        nes.tick();

        if nes.cpu.get_state() == cpu::CPUState::Fetch {
            break;
        }
    }

    let mut residual_time = 0.0_f32;
    let mut emulation_run = false;

    let (mut rl, thread) = raylib::init()
        .size(PAL_WIDTH * SCALE, PAL_HEIGHT * SCALE)
        .title("DogNES")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let fps = rl.get_fps();
        let key = rl.get_key_pressed();

        if emulation_run {
            let delta = rl.get_frame_time();
            if residual_time > 0.0 {
                residual_time -= delta;
            } else {
                residual_time += (1.0 / 60.0) - delta;
                loop {
                    nes.tick();
                    if nes.ppu.frame_complete {
                        break;
                    }
                }
                nes.ppu.frame_complete = false;
            }
        } else if let Some(input) = key {
            match input {
                raylib::consts::KeyboardKey::KEY_C => {
                    loop {
                        nes.tick();

                        if nes.cpu.is_complete() {
                            break;
                        }
                    }

                    loop {
                        nes.tick();

                        if !nes.cpu.is_complete() {
                            break;
                        }
                    }
                }
                raylib::consts::KeyboardKey::KEY_F => {
                    loop {
                        nes.tick();

                        if nes.ppu.frame_complete {
                            break;
                        }
                    }

                    loop {
                        nes.tick();

                        if nes.cpu.is_complete() {
                            break;
                        }
                    }

                    nes.ppu.frame_complete = false;
                }
                _ => {}
            }
        }

        if let Some(input) = key {
            match input {
                raylib::consts::KeyboardKey::KEY_SPACE => {
                    emulation_run = !emulation_run;
                }
                raylib::consts::KeyboardKey::KEY_R => {
                    nes.reset();
                }
                _ => {}
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for y in 0..PAL_HEIGHT {
            for x in 0..PAL_WIDTH {
                d.draw_pixel(
                    x,
                    y,
                    PAL_PALETTE[nes.ppu.buf[y as usize][x as usize] as usize],
                );
                // d.draw_rectangle(
                //     x * SCALE,
                //     y * SCALE,
                //     SCALE,
                //     SCALE,
                //     PAL_PALETTE[buf[y as usize][x as usize] as usize],
                // );
            }
        }

        d.draw_text(
            &format!("{fps} FPS"),
            2,
            2,
            8 * SCALE,
            if fps < 60 { Color::RED } else { Color::GREEN },
        );
    }
}
