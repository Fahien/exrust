// CHIP-8 emulator
//
// OP = operation = implemented in hardware = intrinsic operation
//
// Registers = containers for data directly accessed by the CPU
// Operands must be moved to registers for the operation to function.
//
// Opcode = number that maps an operation
struct Cpu {
    // R15 is used as a carry flag
    registers: [u8; 16],

    // Memory address of the next instruction
    program_counter: usize,

    memory: [u8; 4096],

    // Memory for storing addresses
    stack: [u16; 16],
    stack_pointer: usize,
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "R0 {}\nR1 {}\nPC {}",
            self.registers[0], self.registers[1], self.program_counter
        )
    }
}

impl Cpu {
    fn run(&mut self) {
        loop {
            // Read opcode
            let opcode = self.read_opcode();

            // Read every two bytes (16 bit architecture)
            self.program_counter += 2;

            // Decode instruction (4 nibbles: half of a byte)
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF;

            // Match decoded instruction to known opcodes
            // Dispatch execution of operation to a function
            match (c, x, y, d) {
                // Halt
                (0, 0, 0, 0) => return,

                // Return
                (0, 0, 0xE, 0xE) => self.ret(),

                // Add
                (0x8, _, _, 0x4) => self.add_xy(x, y),

                // Jump
                (0x2, _, _, _) => self.call(nnn),

                // Yet unimplemented
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn read_opcode(&self) -> u16 {
        // Big-endian approach
        let high_byte = self.memory[self.program_counter] as u16;
        let low_byte = self.memory[self.program_counter + 1] as u16;
        (high_byte << 8) | low_byte
    }

    /// Adds x and y, storing the result in x
    fn add_xy(&mut self, x: u8, y: u8) {
        let a = self.registers[x as usize];
        let b = self.registers[y as usize];

        let (v, overflow) = a.overflowing_add(b);

        self.registers[x as usize] = v;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    /// Calling a function is a three step process
    fn call(&mut self, addr: u16) {
        if self.stack_pointer > self.stack.len() {
            panic!("Stack overflow");
        }

        // Store current memory location on the stack
        // so that we can return to that later on
        self.stack[self.stack_pointer] = self.program_counter as u16;

        // Increment the stack pointer, so that we can store
        // another memory location if we need to continue calling functions
        self.stack_pointer += 1;

        // Set the program counter to the address of the call
        self.program_counter = addr as usize;
    }

    // Returning from a function reverses the calling process
    fn ret(&mut self) {
        // Decrement the stack pointer
        self.stack_pointer -= 1;

        // Retrieve the calling memory address from the stack
        let addr = self.stack[self.stack_pointer];

        // Set program counter to previous address
        self.program_counter = addr as usize;
    }
}

fn main() {
    // Init CPU
    let mut cpu = Cpu {
        registers: [0; 16],
        program_counter: 0,
        memory: [0; 4096],
        stack: [0; 16],
        stack_pointer: 0,
    };

    // Load operation in memory pointing by PC register
    cpu.memory[0] = 0x80;
    cpu.memory[1] = 0x14;
    cpu.memory[0] = 0x80;
    cpu.memory[1] = 0x24;

    // Load operands into registers
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    // Perform operation
    cpu.run();
    println!("{}", cpu);
}
