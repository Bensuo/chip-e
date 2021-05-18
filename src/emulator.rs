use crate::chip8::{ChipKey, CPU};
use glium::index::PrimitiveType;
use glium::texture::{pixel_buffer::PixelBuffer, Texture2d};
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use glium::{implement_vertex, program, uniform, IndexBuffer, Surface, VertexBuffer};
use imgui::Context;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::{Duration, Instant};
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

const IPS_CAP: u128 = 700;
const TIMER_CAP: u128 = 60;
const CPU_NS_PER_FRAME: u128 = 1_000_000_000 / IPS_CAP;
const TIMER_NS_PER_FRAME: u128 = 1_000_000_000 / TIMER_CAP;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub fn run(rom_path: &str) -> Result<(), String> {
    let vertices = [
        Vertex {
            position: [-1.0, -1.0],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [-1.0, 1.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [1.0, 1.0],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [1.0, -1.0],
            tex_coords: [1.0, 1.0],
        },
    ];

    let mut cpu = CPU::new();
    cpu.initialize();
    cpu.load_program(&rom_path);
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new()
        .with_title("Chip-E")
        .with_inner_size(winit::dpi::PhysicalSize::new(1280, 640))
        .with_min_inner_size(winit::dpi::PhysicalSize::new(640, 320));
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(builder, cb, &event_loop).unwrap();
    // let gl_window = display.gl_window();
    // let window = gl_window.window();
    let mut imgui = Context::create();
    let mut platform = WinitPlatform::init(&mut imgui); // step 1
    platform.attach_window(
        imgui.io_mut(),
        display.gl_window().window(),
        HiDpiMode::Default,
    ); // step 2
    let renderer = imgui_glium_renderer::Renderer::init(&mut imgui, &display).unwrap();
    let mut frame_start = Instant::now();
    let mut timer_counter = 0;
    let mut frame_accumulator = 0;
    let mut frame_count = 0;
    let pixel_buf: PixelBuffer<(u8, u8, u8, u8)> = PixelBuffer::new_empty(&display, 64 * 32);
    // let mut raw_pixels: [u32; 64 * 32] = [0; 64 * 32];
    let mut raw_pixels: [(u8, u8, u8, u8); 64 * 32] = [(0, 0, 0, 0); 64 * 32];
    pixel_buf.write(&raw_pixels);
    let mipmap = glium::texture::MipmapsOption::NoMipmap;
    let game_texture = Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        mipmap,
        64,
        32,
    )
    .unwrap();
    let vertex_buffer = VertexBuffer::immutable(&display, &vertices).unwrap();
    let index_buffer =
        IndexBuffer::immutable(&display, PrimitiveType::TriangleStrip, &[1u16, 2, 0, 3]).unwrap();
    let program = program!(&display, 140=>{vertex: include_str!("shader/vert.glsl"), fragment: include_str!("shader/frag.glsl")}).unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::NewEvents(_) => {
                // other application-specific logic
                // last_frame = self.imgui.io_mut().update_delta_time(last_frame);
            }
            Event::MainEventsCleared => {
                // other application-specific logic
                platform
                    .prepare_frame(imgui.io_mut(), display.gl_window().window()) // step 4
                    .expect("Failed to prepare frame");

                //Game loop stuff, maybe not the right place?
                cpu.emulate_cycle();
                if timer_counter >= TIMER_NS_PER_FRAME {
                    cpu.update_timers();
                    timer_counter = 0;
                }

                if cpu.draw_flag {
                    update_pixels(&cpu, &mut raw_pixels, &pixel_buf);
                    let mut frame = display.draw();
                    frame.clear_color(0.0, 0.0, 1.0, 1.0);
                    game_texture.main_level().raw_upload_from_pixel_buffer(
                        pixel_buf.as_slice(),
                        0..64,
                        0..32,
                        0..1,
                    );
                    let behaviour = glium::uniforms::SamplerBehavior {
                        minify_filter: MinifySamplerFilter::Nearest,
                        magnify_filter: MagnifySamplerFilter::Nearest,
                        ..Default::default()
                    };
                    let uniforms = uniform! {
                        matrix: [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [ 0.0 , 0.0, 0.0, 1.0f32],
                        ],
                        tex: glium::uniforms::Sampler(&game_texture, behaviour),
                    };
                    frame
                        .draw(
                            &vertex_buffer,
                            &index_buffer,
                            &program,
                            &uniforms,
                            &Default::default(),
                        )
                        .unwrap();
                    frame.finish().unwrap();
                    //         let gfx = &self.cpu.gfx;
                    //         self.screen_texture
                    //             .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                    //                 for y in 0..32 {
                    //                     for x in 0..64 {
                    //                         let tex_offset = y * pitch + x * 3;
                    //                         let gfx_offset = y * pitch / 3 + x;
                    //                         let val = if gfx[gfx_offset] == 1 { 255 } else { 0 };
                    //                         buffer[tex_offset] = val;
                    //                         buffer[tex_offset + 1] = val;
                    //                         buffer[tex_offset + 2] = val;
                    //                     }
                    //                 }
                    //             })
                    //             .unwrap();
                    //         self.cpu.draw_flag = false;
                    //         self.canvas.clear();
                    //         self.canvas.copy(&self.screen_texture, None, None)?;
                    //         self.canvas.present();
                    //     }
                }
                // Check the frame time, if it's too fast then spin until we reach CPU_NS_PER_FRAME
                // Sleep seems very inaccurate, rounding to 1ms maybe?
                let frame_time = frame_start.elapsed().as_nanos();
                if frame_time < CPU_NS_PER_FRAME {
                    while frame_start.elapsed().as_nanos() < CPU_NS_PER_FRAME {
                        std::thread::yield_now();
                    }
                }
                //Frame has really ended, get the new time and update some counters/print fps
                let frame_time = frame_start.elapsed().as_nanos();
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
                frame_start = Instant::now();
                // self.window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let ui = imgui.frame();
                // application-specific rendering *under the UI*

                // construct the UI

                platform.prepare_render(&ui, &display.gl_window().window()); // step 5
                                                                             // render the UI with a renderer
                let draw_data = ui.render();
                // renderer.render(..., draw_data).expect("UI rendering failed");

                // application-specific rendering *over the UI*
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            // other application-specific event handling
            event => {
                platform.handle_event(imgui.io_mut(), &display.gl_window().window(), &event);
                // step 3
                // other application-specific event handling
                match event {
                    Event::DeviceEvent {
                        event:
                            DeviceEvent::Key(KeyboardInput {
                                state: ElementState::Released,
                                virtual_keycode: Some(key),
                                ..
                            }),
                        ..
                    } => {
                        if let Some(key) = map_keycode(key) {
                            cpu.handle_key_event(key, false);
                        }
                    }
                    Event::DeviceEvent {
                        event:
                            DeviceEvent::Key(KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(key),
                                ..
                            }),
                        ..
                    } => {
                        if let Some(key) = map_keycode(key) {
                            cpu.handle_key_event(key, true);
                        }
                    }
                    _ => {}
                }
            }
        }
    })
    // 'running: loop {
    //     for event in self.event_pump.poll_iter() {
    //         // imgui_sdl2.handle_event(&mut imgui, &event);
    //         // if imgui_sdl2.ignore_event(&event) {
    //         //     continue;
    //         // }
    //         match event {
    //             Event::Quit { .. } => break 'running,
    //             Event::KeyDown {
    //                 keycode: Some(keycode),
    //                 ..
    //             } if keycode == Keycode::Escape => break 'running,
    //             Event::KeyDown {
    //                 keycode: Some(keycode),
    //                 ..
    //             } => {
    //                 if let Some(key) = map_keycode(keycode) {
    //                     self.cpu.handle_key_event(key, true);
    //                 }
    //             }
    //             Event::KeyUp {
    //                 keycode: Some(keycode),
    //                 ..
    //             } => {
    //                 if let Some(key) = map_keycode(keycode) {
    //                     self.cpu.handle_key_event(key, false);
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
    //     self.cpu.emulate_cycle();
    //     if timer_counter >= TIMER_NS_PER_FRAME {
    //         self.cpu.update_timers();
    //         timer_counter = 0;
    //     }
    //     if self.cpu.draw_flag {
    //         let gfx = &self.cpu.gfx;
    //         self.screen_texture
    //             .with_lock(None, |buffer: &mut [u8], pitch: usize| {
    //                 for y in 0..32 {
    //                     for x in 0..64 {
    //                         let tex_offset = y * pitch + x * 3;
    //                         let gfx_offset = y * pitch / 3 + x;
    //                         let val = if gfx[gfx_offset] == 1 { 255 } else { 0 };
    //                         buffer[tex_offset] = val;
    //                         buffer[tex_offset + 1] = val;
    //                         buffer[tex_offset + 2] = val;
    //                     }
    //                 }
    //             })
    //             .unwrap();
    //         self.cpu.draw_flag = false;
    //         self.canvas.clear();
    //         self.canvas.copy(&self.screen_texture, None, None)?;
    //         self.canvas.present();
    //     }
    //     // imgui_sdl2.prepare_frame(imgui.io_mut(), canvas.window(), &event_pump.mouse_state());
    //     // Check the frame time, if it's too fast then spin until we reach IPS_CAP
    //     // Sleep seems very inaccurate, rounding to 1ms maybe?
    //     frame_time = frame_start.elapsed().as_nanos();
    //     if frame_time < CPU_NS_PER_FRAME {
    //         while frame_start.elapsed().as_nanos() < CPU_NS_PER_FRAME {
    //             std::thread::yield_now();
    //         }
    //     }
    //     //Frame has really ended, get the new time and update some counters/print fps
    //     frame_time = frame_start.elapsed().as_nanos();
    //     timer_counter += frame_time;
    //     if frame_count >= IPS_CAP {
    //         println!(
    //             "FPS: {:.2}ms/{:.2}fps avg",
    //             (frame_accumulator as f64 / IPS_CAP as f64) / 1000000.0,
    //             1000.0 / ((frame_accumulator as f64 / IPS_CAP as f64) / 1000000.0)
    //         );
    //         frame_accumulator = 0;
    //         frame_count = 0;
    //     } else {
    //         frame_count += 1;
    //         frame_accumulator += frame_time;
    //     }
    //     frame_start = Instant::now();
    // }
}

fn update_pixels(
    cpu: &CPU,
    buffer: &mut [(u8, u8, u8, u8); 64 * 32],
    pixel_buffer: &PixelBuffer<(u8, u8, u8, u8)>,
) {
    let gfx = &cpu.gfx;
    for y in 0..32 {
        for x in 0..64 {
            let idx = y * 64 + x;
            let val = if gfx[idx] == 1 { u8::MAX } else { 0 };
            buffer[idx] = (val, val, val, val);
        }
    }
    pixel_buffer.write(buffer);
}
fn map_keycode(key: VirtualKeyCode) -> Option<ChipKey> {
    match key {
        VirtualKeyCode::X => Some(ChipKey::K0),
        VirtualKeyCode::Key1 => Some(ChipKey::K1),
        VirtualKeyCode::Key2 => Some(ChipKey::K2),
        VirtualKeyCode::Key3 => Some(ChipKey::K3),
        VirtualKeyCode::Q => Some(ChipKey::K4),
        VirtualKeyCode::W => Some(ChipKey::K5),
        VirtualKeyCode::E => Some(ChipKey::K6),
        VirtualKeyCode::A => Some(ChipKey::K7),
        VirtualKeyCode::S => Some(ChipKey::K8),
        VirtualKeyCode::D => Some(ChipKey::K9),
        VirtualKeyCode::Z => Some(ChipKey::KA),
        VirtualKeyCode::C => Some(ChipKey::KB),
        VirtualKeyCode::Key4 => Some(ChipKey::KC),
        VirtualKeyCode::R => Some(ChipKey::KD),
        VirtualKeyCode::F => Some(ChipKey::KE),
        VirtualKeyCode::V => Some(ChipKey::KF),
        _ => None,
    }
}
