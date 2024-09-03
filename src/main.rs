struct CPU {
    registers: [u8; 16],
    program_counter: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.program_counter;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p+1] as u16;

        op_byte1 << 8 | op_byte2 // To create a u16 opcode, we combine two values from memory with the logical OR operation. These need to be cast as u16 to start with; otherwise, the left shift sets all of the bits to 0
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.program_counter += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0xF00F) >> 0) as u8;
            let nnn = opcode & 0x0FFF;
            // let kk = (opcode & 0x00FF) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0)     => { return; },
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _                => todo!("opcode {:04x}", opcode)
            }
        }
    }

    /*
    Calling a function is a three-step process:
        1. Store the current memory location on the stack
        2. Increment the stack pointer
        3. Set the current memory location to the intended memory address

    Our stack has a size of 16 so if the stack exceeds 16 then the CPU will panic
     */
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = & mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    /*
    Return from a function involves reversing the calling process
        1. Decrement the stack pointer.
        2. Retrieve the calling memory address from the stack.
        3. Set the current memory location to the intended memory address
    */
    fn ret(&mut self) {
        if self.program_counter == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.program_counter = call_addr as usize;
    }
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        program_counter: 0,
        memory: [0; 4096],
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00;
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    mem[0x004] = 0x00; mem[0x005] = 0x00;

    /*
    Add twice function
        This sets opcode to 0x8014: ADD register 1's value to register 0
        does this operation twice
        0x00EE is the RETURN opcode

    */
    mem[0x100] = 0x80; mem[0x101] = 0x14;
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    mem[0x104] = 0x00; mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
