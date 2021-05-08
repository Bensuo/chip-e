use std::fs;
mod instructions;

static FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct CPU {
    opcode: u16,
    memory: [u8; 4096],
    V: [u8; 16],
    I: u16,
    pc: u16,
    pub gfx: [u8; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
    key: [u8; 16],
    pub draw_flag: bool,
}

pub fn dump_opcodes(cpu: &mut CPU) {
    let pc: u16 = 0x200;

    while pc < 4096 {
        let opcode = read_mem_word(&cpu.memory, pc);
        instructions::decode_opcode(cpu, opcode);
    }
}
fn read_mem_byte(memory: &[u8], addr: u16) -> u8 {
    let byte = memory.get(addr as usize);
    match byte {
        Some(x) => *x,
        None => std::process::exit(0),
    }
}
fn read_mem_word(memory: &[u8], pc: u16) -> u16 {
    return (read_mem_byte(memory, pc) as u16) << 8 | (read_mem_byte(memory, pc + 1) as u16);
}
impl CPU {
    pub fn new() -> CPU {
        CPU {
            opcode: 0x0,
            memory: [0; 4096],
            V: [0; 16],
            I: 0,
            pc: 0,
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            draw_flag: false,
        }
    }
    pub fn load_program(&mut self, fname: &str) {
        let bytes = fs::read(fname).expect("Something went wrong");
        for (i, &val) in bytes.iter().enumerate() {
            self.memory[0x200 + i] = val;
        }
    }
    pub fn initialize(&mut self) {
        // INitialize registers and memory once
        self.pc = 0x200;
        self.opcode = 0;
        self.I = 0;
        self.sp = 0;

        //Copy font set
        for (i, &v) in FONT_SET.iter().enumerate() {
            self.memory[0x50 + i] = v;
        }
        //Filling with text gradient

        // for i in 0..(64 * 32) {
        //     self.gfx[i] = (i as f64 / (64.0 * 32.0) * 255.0) as u8;
        // }
    }
    pub fn emulate_cycle(&mut self) {
        // Fetch opcode
        self.opcode = read_mem_word(&self.memory, self.pc);
        // Decode opcode
        instructions::decode_opcode(self, self.opcode);
        // Execute opcode
        //Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            self.sound_timer -= 1;
        }
        // self.pc += 2;6
    }
    pub fn set_keys() {}

    // OPs
    pub fn clear_display(&mut self) {
        self.gfx.iter_mut().for_each(|m| *m = 0);
        self.draw_flag = true;
    }

    pub fn draw(&mut self, x: u8, y: u8, height: u8) {
        let mut set_vf = false;
        for iy in 0..(height as usize) {
            let sprite_row: u8 = self.memory[(self.I + iy as u16) as usize];
            // println!("Sprite row: {:08b}", sprite_row);
            for ix in 0..8 as usize {
                let idx = (iy + y as usize) * 64 + ix + x as usize;
                let screen_val = self.gfx[idx];
                let sprite_val = sprite_row >> (7 - ix) & 1;
                let new_val = sprite_val ^ screen_val;
                if !set_vf && new_val == 0 && screen_val == 1 {
                    set_vf = true;
                }
                self.gfx[idx] = new_val;
            }
        }
        self.V[15] = if set_vf { 1 } else { 0 };
        self.draw_flag = true;
    }
}
