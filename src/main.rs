mod lib;
use lib::memory_block::MemoryBlock;

fn main() {
    let mut memory_block = MemoryBlock::new();
    memory_block.write(0x0000, 111);
    println!("{}", memory_block.read(0x0000));
}
