const MEMORY_SIZE: usize = 65535;

pub struct Memory {
  memory_block: [u8; MEMORY_SIZE],
}

impl Memory {
  pub fn new() -> Memory {
    Memory {
      memory_block: [0; MEMORY_SIZE],
    }
  }

  pub fn write(&mut self, address: u8, value: u8) {
    self.memory_block[address as usize] = value;
  }

  pub fn read(&mut self, address: u8) -> u8 {
    self.memory_block[address as usize]
  }
}
