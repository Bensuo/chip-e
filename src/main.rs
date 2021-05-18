extern crate glium;
extern crate imgui;
extern crate imgui_glium_renderer;
extern crate imgui_winit_support;
extern crate winit;

#[macro_use]
mod log;

mod chip8;
mod emulator;

fn main() -> Result<(), String> {
    let rom_path = std::env::args().nth(1).expect("No ROM path provided!");

    return emulator::run(&rom_path);
    // let _gl_context = window
    //     .gl_create_context()
    //     .expect("Couldn't create GL context");

    // let mut imgui = imgui::Context::create();
    // imgui.set_ini_filename(None);

    // let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);

    // let renderer = imgui_glium_renderer::Renderer::new(&mut imgui, |s| {
    //     video_subsystem.gl_get_proc_address(s) as _
    // });
}
