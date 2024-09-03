struct CPU {
    current_operation: u16,
    registers: [u8; 2]
}

fn main() {
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2]
    };

    // 8 signifies that the op involves two registers
    // 0 maps to cpu.registers[0]
    // 1 maps to cpu.registers[1]
    // 4 indicates addition
    cpu.current_operation = 0x8014;
    
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
}
