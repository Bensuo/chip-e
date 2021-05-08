extern crate sdl2;

#[macro_use]
mod log;

mod chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::time::{Duration, Instant};

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
    screen_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        let val: u8 = 128;
        for y in 0..32 {
            for x in 0..64 {
                let offset = y * pitch + x * 3;
                buffer[offset] = val;
                buffer[offset + 1] = val;
                buffer[offset + 2] = val;
            }
        }
    })?;

    canvas.clear();
    canvas.copy(&screen_texture, None, None)?;
    canvas.present();

    let mut cpu = chip8::CPU::new();
    cpu.initialize();
    cpu.load_program(&rom_path);

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_start = Instant::now();
    let mut frame_end = Instant::now();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        cpu.emulate_cycle();
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
        }
        canvas.clear();
        canvas.copy(&screen_texture, None, None)?;
        canvas.present();
        frame_end = Instant::now();
        let frame_time = 1000000.0 / (frame_end - frame_start).as_nanos() as f64;
        println!("Frame time: {}ms/{}fps", frame_time, 1000.0 / frame_time);
        frame_start = frame_end;
        // std::thread::sleep(std::time::Duration::from_millis(100));
    }

    return Ok(());
}
