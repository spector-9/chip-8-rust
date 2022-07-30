#[allow(dead_code)]
struct CPU {
    registers: [u8; 16],
    memory: [u8; 4096],
    program_counter: usize,
    stack: [u16; 16],
    stack_pointer: usize,
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

    fn set_register(&mut self, register_number: u8, value: u8) {
        self.registers[register_number as usize] = value;
    }

    fn get_register(&mut self, register_number: u8) -> u8 {
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

            let c = ((opcode & 0xF000) >> (2 * 6)) as u8;
            let x = ((opcode & 0x0F00) >> (2 * 4)) as u8;
            let y = ((opcode & 0x00F0) >> (2 * 2)) as u8;
            let d = ((opcode & 0x000F) >> (2 * 0)) as u8;

            let nnn = opcode & 0x0FFF;

            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (0x2, _, _, _) => self.call(nnn),
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x8, x, y, 0x4) => self.add_code(x, y),
                _ => todo!(),
            }
        }
    }

    fn add_code(&mut self, x: u8, y: u8) {
        let val1 = self.get_register(x);
        let val2 = self.get_register(y);
        let (result, bool) = val1.overflowing_add(val2);
        self.set_register(x, result);
        if bool {
            self.set_register(0xF, 1);
        } else {
            self.set_register(0xF, 0);
        }
    }
}

fn main() {
    let mut x: CPU = CPU {
        registers: [0x0; 16],
        memory: [0x0; 4096],
        program_counter: 0,
        stack: [0x0; 16],
        stack_pointer: 0,
    };

    x.set_register(0x0, 5);
    x.set_register(0x1, 10);

    x.set_memory(0x0, 0x21); x.set_memory(0x1, 0x00);
    x.set_memory(0x2, 0x21); x.set_memory(0x3, 0x00);
    x.set_memory(0x4, 0x00); x.set_memory(0x5, 0x00);

    x.set_memory(0x100, 0x80); x.set_memory(0x101, 0x14);
    x.set_memory(0x102, 0x00); x.set_memory(0x103, 0xEE);

    x.run();

    println!("{}", x.get_register(0));
}
