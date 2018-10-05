mod lib;
use lib::memory::Memory;

fn main() {
    let mut memory = Memory::new();
    memory.write(0x0000, 111);
    println!("{}", memory.read(0x0000));
}
