const MEMORY_SIZE: usize = 65535;

pub type MemoryAddress = usize;
pub type Byte = u8;

pub struct MemoryBlock {
  memory_block: [Byte; MEMORY_SIZE]
}

impl MemoryBlock {
  pub fn new() -> MemoryBlock {
    MemoryBlock{memory_block: [0; MEMORY_SIZE]}
  }
  
  pub fn write(&mut self, address: MemoryAddress, value: Byte) -> Byte {
    self.memory_block[address] = value;
    value
  }
  
  pub fn read(&mut self, address: MemoryAddress) -> Byte {
    self.memory_block[address]
  }
}
