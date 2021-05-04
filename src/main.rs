use std::fs;

struct chip8 {
    opcode: u16,
    memory: [u8; 4096],
    V: [u8; 16],
    I: u16,
    pc: u16,
    gfx: [u8; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
    key: [u8; 16],
    drawFlag: bool,
}

fn decode_opcode(opcode: u16) {
    if opcode == 0x0000 {
        return;
    }
    let code = opcode & 0xF000;
    if let 0x1000 = code {
        println!("jmp to {}", opcode & 0x0FFF);
    } else if let 0x2000 = code {
        println!("call {}", opcode & 0x0FFF);
    } else {
        println!("Unknown opcode {:#06x}", opcode)
    }
}

impl chip8 {
    fn new() -> chip8 {
        chip8 {
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
            drawFlag: false,
        }
    }
    fn load_program(&mut self, fname: &str) {
        let bytes = fs::read(fname).expect("Something went wrong");
        for (i, &val) in bytes.iter().enumerate() {
            self.memory[0x200 + i] = val;
        }
    }
    fn initialize(&mut self) {
        // INitialize registers and memory once
        self.pc = 0x200;
        self.opcode = 0;
        self.I = 0;
        self.sp = 0;
    }
    fn read_mem_byte(&self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }

    fn read_mem_word(&self, pc: u16) -> u16 {
        return (self.read_mem_byte(pc) as u16) << 8 | (self.read_mem_byte(pc + 1) as u16);
    }
    fn emulate_cycle(&mut self) {
        // Fetch opcode
        self.opcode = self.read_mem_word(self.pc);
        // Decode opcode
        decode_opcode(self.opcode);
        // Execute opcode

        //Update timers
        self.pc += 2;
    }
    fn set_keys() {}
}
fn main() {
    // Defining memory etc.
    let mut cpu = chip8::new();
    cpu.initialize();
    cpu.load_program("picture.ch8");

    loop {
        cpu.emulate_cycle();
    }
}
