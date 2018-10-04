use lib::memory::{MemoryAddress, Memory};

pub type CPUCycles = u8;

pub enum AddressingMode {
  Immediate,
  ZeroPage
}

pub trait Instruction {
  mode: AddressingMode;
  parameter: MemoryAddress;
  operation: (&self, memory: &mut Memory);
  cycles: CPUCycles;
}
