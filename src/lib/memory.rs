const MEMORY_SIZE: usize = 65535;

pub type MemoryAddress = u8;
pub type Byte = u8;
type MemoryBlock = [Byte; MEMORY_SIZE];

pub struct Memory {
  memory_block: MemoryBlock,
}

impl Memory {
  pub fn new() -> Memory {
    Memory {
      memory_block: [0; MEMORY_SIZE],
    }
  }

  pub fn write(&mut self, address: MemoryAddress, value: Byte) {
    self.memory_block[address as usize] = value;
  }

  pub fn read(&mut self, address: MemoryAddress) -> Byte {
    self.memory_block[address as usize]
  }
}
