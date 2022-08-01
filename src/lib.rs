#[allow(dead_code)]
// Technical reference for chip-8 http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
struct CPU {
    registers: [u8; 16],
    memory: [u8; 4096],
    program_counter: usize,
    stack: [u16; 16],
    stack_pointer: usize,
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            registers: [0; 16],
            memory: [0; 4096],
            program_counter: 0,
            stack: [0; 16],
            stack_pointer: 0,
        }
    }
}

// Stack Methods
#[allow(dead_code)]
impl CPU {
    fn call(&mut self, address: u16) {
        let sp = &mut self.stack_pointer;
        let stack = &mut self.stack;

        if *sp > stack.len() {
            panic!("Stack Overflow");
        }

        stack[*sp] = self.program_counter as u16;
        *sp += 1;

        self.program_counter = address as usize;
    }

    fn ret(&mut self) {
        let sp = &mut self.stack_pointer;
        let stack = &mut self.stack;

        if *sp == 0 {
            panic!("Stack Underflow");
        }

        *sp -= 1;
        let call_addr = stack[*sp];
        self.program_counter = call_addr as usize;
    }
}

// Reading and writing memory/registers
#[allow(dead_code)]
impl CPU {
    fn read_opcode(&self) -> u16 {
        let high_byte = self.memory[self.program_counter as usize];
        let low_byte = self.memory[(self.program_counter + 1) as usize];
        (high_byte as u16) << (2 * 4) | low_byte as u16
    }

    fn set_memory(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }


    fn get_register(&self, register_number: u8) -> u8 {
        self.registers[register_number as usize]
    }
}

// Running the CPU
#[allow(dead_code)]
impl CPU {
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;

            let x = ((opcode & 0x0F00) >> (2 * 4)) as u8; // Lower bits of High byte
            let y = ((opcode & 0x00F0) >> (2 * 2)) as u8; // Upper bits of low byte

            let kk = (opcode & 0x00FF) as u8; // a byte
            let n = opcode & 0x000F; // a nibble
            let nnn = opcode & 0x0FFF; // an addr

            match opcode {
                0x0000 => return,
                0x00EE => self.ret(),
                0x1000..=0x1FFF => self.jp(1, nnn),
                0x2000..=0x2FFF => self.call(nnn),
                0x3000..=0x3FFF => self.se(x, kk),
                0x4000..=0x4FFF => self.sne(x, kk),
                0x5000..=0x5FFF => self.se(x, kk),
                0x6000..=0x6FFF => self.ld(x, kk),
                0x7000..=0x7FFF => self.add(x, kk),
                0x8000..=0x8FFF => match n {
                    0x0 => self.ld(x, self.get_register(y)),
                    0x1 => self.or(x, y),
                    0x2 => self.and(x, y),
                    0x3 => self.xor(x, y),
                    0x4 => self.add_carry(x, y),
                    0x5 => self.sub(x, y),
                    0x6 => self.shr(x),
                    0xE => self.shl(x),
                    _ => todo!(),
                },
                0x9000..=0x9FF0 => self.sne(x, self.get_register(y)),
                0xB000..=0xBFFF => self.jp(0, nnn),
                _ => todo!(),
            }
        }
    }

    fn add(&mut self, x: u8, kk: u8) {
        let val1 = self.get_register(x);
        let val2 = kk;
        let (result, _) = val1.overflowing_add(val2);
        self.ld(x, result);
    }

    fn add_carry(&mut self, x: u8, y: u8) {
        let val1 = self.get_register(x);
        let val2 = self.get_register(y);
        let (result, bool) = val1.overflowing_add(val2);
        self.ld(x, result);
        if bool {
            self.ld(0xF, 1);
        } else {
            self.ld(0xF, 0);
        }
    }

    fn sub(&mut self, x: u8, y: u8) {
        let val1 = self.get_register(x);
        let val2 = self.get_register(y);
        let (result, bool) = val1.overflowing_sub(val2);
        self.ld(x, result);
        if bool {
            self.ld(0xF, 1);
        } else {
            self.ld(0xF, 0);
        }
    }

    fn or(&mut self, x: u8, y: u8) {
        let val1 = self.get_register(x);
        let val2 = self.get_register(y);
        let result = val1 | val2;
        self.ld(x, result);
    }

    fn and(&mut self, x: u8, y: u8) {
        let val1 = self.get_register(x);
        let val2 = self.get_register(y);
        let result = val1 & val2;
        self.ld(x, result);
    }

    fn xor(&mut self, x: u8, y: u8) {
        let val1 = self.get_register(x);
        let val2 = self.get_register(y);
        let result = val1 ^ val2;
        self.ld(x, result);
    }

    fn shr(&mut self, x: u8) {
        let val = self.get_register(x);
        if (val & 0x01) == 1 {
            self.ld(0xF, 1);
        } else {
            self.ld(0xF, 0);
        }
        let result = self.get_register(x) >> 1;
        self.ld(x, result);
    }

    fn subn(&mut self, x: u8, y: u8) {
        let val1 = self.get_register(y);
        let val2 = self.get_register(x);
        let (result, bool) = val1.overflowing_sub(val2);
        self.ld(x, result);
        if bool {
            self.ld(0xF, 1);
        } else {
            self.ld(0xF, 0);
        }
    }

    fn shl(&mut self, x: u8) {
        let val = self.get_register(x);
        if (val & 0x80) == 1 {
            self.ld(0xF, 1);
        } else {
            self.ld(0xF, 0);
        }
        let result = self.get_register(x) << 1;
        self.ld(x, result);
    }

    fn jp(&mut self, flag: u8, nnn: u16) {
        let mut reg_0 = 0;
        if flag == 0 {
            reg_0 = self.get_register(0) as u16;
        }
        self.program_counter = (reg_0 + nnn) as usize;
    }

    fn se(&mut self, x: u8, kk: u8){
        if self.get_register(x) == kk {
            self.program_counter += 2;
        }
    }

    fn sne(&mut self, x: u8, kk: u8){
        if self.get_register(x) != kk {
            self.program_counter += 2;
        }
    }

    fn ld(&mut self, register_number: u8, value: u8) {
        self.registers[register_number as usize] = value;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut x = CPU::default();
        x.ld(0x0, 5);
        x.ld(0x1, 10);

        x.set_memory(0x0, 0x80); x.set_memory(0x1, 0x14);
        x.run();

        assert_eq!(15, x.get_register(0));
    }

    #[test]
    fn add_using_stack() {
        let mut x = CPU::default();
        x.ld(0x0, 5);
        x.ld(0x1, 10);

        x.set_memory(0x0, 0x21); x.set_memory(0x1, 0x00);
        x.set_memory(0x2, 0x21); x.set_memory(0x3, 0x00);
        x.set_memory(0x4, 0x00); x.set_memory(0x5, 0x00);

        x.set_memory(0x100, 0x80); x.set_memory(0x101, 0x14);
        x.set_memory(0x102, 0x00); x.set_memory(0x103, 0xEE);

        x.run();

        assert_eq!(25, x.get_register(0));
    }
}
