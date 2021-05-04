extern crate sdl2;

mod chip8;
use sdl2::pixels::Color;
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("CHIP-E", 800, 400)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    // Defining memory etc.
    let mut cpu = chip8::CPU::new();
    cpu.initialize();
    cpu.load_program("picture.ch8");

    loop {
        cpu.emulate_cycle();

        canvas.clear();
        canvas.present();
    }
}
