mod lib;
use lib::memory::Memory;

fn main() {
    let mut memory_block = Memory::new();
    memory_block.write(0x0000, 111);
    println!("{}", memory_block.read(0x0000));
}
