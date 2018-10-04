const MEMORY_SIZE: usize = 65535;

pub type MemoryAddress = usize;
pub type Byte = u8;
pub type MemoryBlock = [Byte; MEMORY_SIZE];

pub struct Memory {
  memory_block: MemoryBlock
}

impl Memory {
  pub fn new() -> Memory {
    Memory{memory_block: [0; MEMORY_SIZE]}
  }
  
  pub fn write(&mut self, address: MemoryAddress, value: Byte) -> Byte {
    self.memory_block[address] = value;
    value
  }
  
  pub fn read(&mut self, address: MemoryAddress) -> Byte {
    self.memory_block[address]
  }
}
