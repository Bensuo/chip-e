extern crate sdl2;

#[macro_use]
mod log;

mod chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::convert::TryInto;
use std::time::Duration;
use std::time::Instant;
const IPS_CAP: u128 = 700;
const TIMER_CAP: u128 = 60;
const CPU_NS_PER_FRAME: u128 = 1_000_000_000 / IPS_CAP;
const TIMER_NS_PER_FRAME: u128 = 1_000_000_000 / TIMER_CAP;

fn main() -> Result<(), String> {
    let rom_path = std::env::args().nth(1).expect("No ROM path provided!");
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("CHIP-E", 800, 400)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut screen_texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 64, 32)
        .map_err(|e| e.to_string())?;

    canvas.clear();
    canvas.copy(&screen_texture, None, None)?;
    canvas.present();

    let mut cpu = chip8::CPU::new();
    cpu.initialize();
    cpu.load_program(&rom_path);

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_start = Instant::now();
    let mut frame_end;
    let mut frame_time;
    let mut timer_counter = 0;
    let mut frame_accumulator = 0;
    let mut frame_count = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } if keycode == Keycode::Escape => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = map_keycode(keycode) {
                        cpu.handle_key_event(key, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = map_keycode(keycode) {
                        cpu.handle_key_event(key, false);
                    }
                }
                _ => {}
            }
        }
        cpu.emulate_cycle();
        if timer_counter >= TIMER_NS_PER_FRAME {
            cpu.update_timers();
            timer_counter = 0;
        }
        if cpu.draw_flag {
            screen_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..32 {
                    for x in 0..64 {
                        let tex_offset = y * pitch + x * 3;
                        let gfx_offset = y * pitch / 3 + x;
                        let val = if cpu.gfx[gfx_offset] == 1 { 255 } else { 0 };
                        buffer[tex_offset] = val;
                        buffer[tex_offset + 1] = val;
                        buffer[tex_offset + 2] = val;
                    }
                }
            })?;
            cpu.draw_flag = false;
            canvas.clear();
            canvas.copy(&screen_texture, None, None)?;
            canvas.present();
        }
        // Check the frame time, if it's too fast then spin until we reach IPS_CAP
        // Sleep seems very inaccurate, rounding to 1ms maybe?
        frame_end = Instant::now();
        frame_time = (frame_end - frame_start).as_nanos();
        if frame_time < CPU_NS_PER_FRAME {
            // std::thread::sleep(Duration::from_nanos(
            //     (CPU_NS_PER_FRAME - frame_time).try_into().unwrap(),
            // ));
            while frame_time < CPU_NS_PER_FRAME {
                frame_end = Instant::now();
                frame_time = (frame_end - frame_start).as_nanos();
            }
        }
        //Frame has really ended, get the new time and update some counters/print fps
        frame_end = Instant::now();
        frame_time = (frame_end - frame_start).as_nanos();
        timer_counter += frame_time;
        if frame_count >= IPS_CAP {
            println!(
                "FPS: {:.2}ms/{:.2}fps avg",
                (frame_accumulator as f64 / IPS_CAP as f64) / 1000000.0,
                1000.0 / ((frame_accumulator as f64 / IPS_CAP as f64) / 1000000.0)
            );
            frame_accumulator = 0;
            frame_count = 0;
        } else {
            frame_count += 1;
            frame_accumulator += frame_time;
        }
        frame_start = frame_end;
    }

    return Ok(());
}

fn map_keycode(key: Keycode) -> Option<chip8::ChipKey> {
    match key {
        Keycode::X => Some(chip8::ChipKey::K0),
        Keycode::Num1 => Some(chip8::ChipKey::K1),
        Keycode::Num2 => Some(chip8::ChipKey::K2),
        Keycode::Num3 => Some(chip8::ChipKey::K3),
        Keycode::Q => Some(chip8::ChipKey::K4),
        Keycode::W => Some(chip8::ChipKey::K5),
        Keycode::E => Some(chip8::ChipKey::K6),
        Keycode::A => Some(chip8::ChipKey::K7),
        Keycode::S => Some(chip8::ChipKey::K8),
        Keycode::D => Some(chip8::ChipKey::K9),
        Keycode::Z => Some(chip8::ChipKey::KA),
        Keycode::C => Some(chip8::ChipKey::KB),
        Keycode::Num4 => Some(chip8::ChipKey::KC),
        Keycode::R => Some(chip8::ChipKey::KD),
        Keycode::F => Some(chip8::ChipKey::KE),
        Keycode::V => Some(chip8::ChipKey::KF),
        _ => None,
    }
}
