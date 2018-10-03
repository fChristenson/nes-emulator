const MEMORY_SIZE: usize = 65535;

fn main() {
    let mut memory_buffer: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
    println!("Set: {}", write_to_memory(&mut memory_buffer, 0, 1));
    println!("Found: {}", read_from_memory(&memory_buffer, 0));

    println!("Set: {}", write_to_memory(&mut memory_buffer, 0, 0));
    println!("Found: {}", read_from_memory(&memory_buffer, 0))
}

fn write_to_memory(memory_buffer: &mut [u8], index: usize, value: u8) -> u8 {
    memory_buffer[index] = value;
    value
}

fn read_from_memory(memory_buffer: &[u8], index: usize) -> u8 {
    memory_buffer[index]
}
