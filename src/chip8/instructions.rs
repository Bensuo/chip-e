use crate::chip8::CPU;

pub fn decode_opcode(cpu: &mut CPU, opcode: u16) {
    if opcode == 0x0000 {
        return;
    }
    let code = opcode & 0xF000;
    if let 0x0000 = code {
        if let 0x00E0 = opcode {
            println!("00E0 disp_clear()");
        } else if let 0x00EE = opcode {
            println!("00EE return;");
        } else {
            println!("0NNN Call machine code at {}", opcode & 0x0FFF);
        }
    } else if let 0x1000 = code {
        println!("1NNN jmp to {}", opcode & 0x0FFF);
        cpu.I = opcode & 0x0FFF;
        cpu.pc += 2;
    } else if let 0x2000 = code {
        println!("2NNN call *({})()", opcode & 0x0FFF);
    } else if let 0x3000 = code {
        println!("3XNN if(V{} == {})", opcode & 0x0F00 >> 8, opcode & 0x00FF);
    } else if let 0x4000 = code {
        println!("4XNN if(V{} != {})", opcode & 0x0F00 >> 8, opcode & 0x00FF);
    } else if let 0x5000 = code {
        println!(
            "5XY0 if(V{} == V{})",
            opcode & 0x0F00 >> 8,
            opcode & 0x00F0 >> 4
        );
    } else if let 0x6000 = code {
        println!("6XNN V{} = {}", opcode & 0x0F00 >> 8, opcode & 0x00FF);
    } else if let 0x7000 = code {
        println!("7XNN V{} += {}", opcode & 0x0F00 >> 8, opcode & 0x00FF);
    } else if let 0x8000 = code {
        let code = opcode & 0x000F;
        let X = opcode & 0x0F00 >> 8;
        let Y = opcode & 0x00F0 >> 4;
        if let 0x0000 = code {
            println!("8XY0 V{} = V{}", X, Y);
        } else if let 0x0001 = code {
            println!("8XY1 V{} = V{} | V{}", X, X, Y);
        } else if let 0x0002 = code {
            println!("8XY2 V{} = V{} & V{}", X, X, Y);
        } else if let 0x0003 = code {
            println!("8XY3 V{} = V{} ^ V{}", X, X, Y);
        } else if let 0x0004 = code {
            println!("8XY4 V{} += V{}", X, Y);
        } else if let 0x0005 = code {
            println!("8XY5 V{} -= V{}", X, Y);
        } else if let 0x0006 = code {
            println!("8XY6 V{} >>= 1", X);
        } else if let 0x0007 = code {
            println!("8XY7 V{} = V{} - V{}", X, Y, X);
        } else if let 0x000E = code {
            println!("8XYE V{} <<= 1", X);
        }
    } else if let 0x9000 = code {
        println!(
            "9XY0 if (V{} != V{})",
            opcode & 0x0F00 >> 8,
            opcode & 0x00F0 >> 4
        );
    } else if let 0xA000 = code {
        println!("ANNN I = {}", opcode & 0x0FFF);
    } else if let 0xB000 = code {
        println!("BNNN PC = V0 + {}", opcode & 0x00FF);
    } else if let 0xC000 = code {
        println!(
            "CXNN V{} = rand() & {}",
            opcode & 0x0F00 >> 8,
            opcode & 0x00FF
        );
    } else if let 0xD000 = code {
        println!(
            "DXYN draw(V{}, V{}, {})",
            opcode & 0x0F00 >> 8,
            opcode & 0x00F0 >> 4,
            opcode & 0x000F
        );
    } else if let 0xE000 = code {
        let code = opcode & 0x000F;
        if let 0x000E = code {
            println!("EX9E if(key() == V{}", opcode & 0x0F00 >> 8);
        } else if let 0x0001 = code {
            println!("EXA1 if(key() != V{}", opcode & 0x0F00 >> 8);
        }
    } else if let 0xF000 = code {
        let code = opcode & 0x00FF;
        if let 0x0007 = code {
            println!("FX07 V{} = get_delay()", opcode & 0x0F00 >> 8);
        } else if let 0x000A = code {
            println!("FX0A V{} = get_key()", opcode & 0x0F00 >> 8);
        } else if let 0x0015 = code {
            println!("FX15 delay_timer(V{})", opcode & 0x0F00 >> 8);
        } else if let 0x0018 = code {
            println!("FX18 sound_timer(V{})", opcode & 0x0F00 >> 8);
        } else if let 0x001E = code {
            println!("FX1E I += V{}", opcode & 0x0F00 >> 8);
        } else if let 0x0029 = code {
            println!("FX29 I = sprite_addr[V{}]", opcode & 0x0F00 >> 8);
        } else if let 0x0033 = code {
            println!("FX33 set_BCD(V{})", opcode & 0x0F00 >> 8);
        } else if let 0x0055 = code {
            println!("FX55 reg_dump(V{}, &I)", opcode & 0x0F00 >> 8);
        } else if let 0x0065 = code {
            println!("FX65 reg_load(V{}, &I)", opcode & 0x0F00 >> 8);
        }
    } else {
        panic!("Unknown opcode {:#06x}", opcode);
    }
}
