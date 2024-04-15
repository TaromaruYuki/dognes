use cpu::PAL_PALETTE;
use raylib::prelude::*;
use std::{cell::RefCell, rc::Rc};

const PAL_WIDTH: i32 = 256;
const PAL_HEIGHT: i32 = 240;
const SCALE: i32 = 1;

fn main() {
    let cart = cartridge::Cartridge::new("dk.nes".to_string());
    let mut nes = cpu::NES::default();
    nes.attach_cart(Rc::new(RefCell::new(cart)));
    nes.reset();

    // loop {
    //     nes.tick();

    //     if nes.cpu.get_state() == cpu::CPUState::Fetch {
    //         break;
    //     }
    // }

    let mut residual_time = 0.0_f32;
    let mut emulation_run = false;

    let (mut rl, thread) = raylib::init()
        .size(PAL_WIDTH * SCALE, PAL_HEIGHT * SCALE)
        .title("DogNES")
        .build();

    // rl.set_target_fps(60);

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

                        if nes.cpu_complete() {
                            break;
                        }
                    }

                    loop {
                        nes.tick();

                        if !nes.cpu_complete() {
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

                        if nes.cpu_complete() {
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
                let color_raw = PAL_PALETTE[nes.ppu.buf[y as usize][x as usize] as usize];
                d.draw_pixel(x, y, Color::new(color_raw.0, color_raw.1, color_raw.2, 255));
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
