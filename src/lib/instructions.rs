use lib::memory::{Memory, Byte, MemoryAddress};
use lib::cpu::Cpu;

pub enum InstructionType {
  LDA,
}

pub enum AddressingMode {
  Implicit,
  Accumulator,
  Immediate,
  ZeroPage,
  ZeroPageX,
  ZeroPageY,
  Relative,
  Absolute,
  AbsoluteX,
  AbsoluteY,
  Indirect,
  IndexedIndirect,
  IndirectIndexed
}

pub struct Instruction {
  op_code: Byte,
  cycles: u8,
  byte_size: Byte,
  instruction_type: InstructionType,
  addressing_mode: AddressingMode,
  operation: Box<Fn(&Instruction, &mut Memory, u8)>,
}

struct Parameter {
  value: Byte,
  address: MemoryAddress,
  extra_cycles: u8
}

impl Instruction {
  pub fn new(
    instruction_type: InstructionType,
    addressing_mode: AddressingMode,
    operation: Box<Fn(&Instruction, &mut Memory, u8)>,
  ) -> Instruction {
    
    let byte_size = match addressing_mode {
      AddressingMode::Accumulator | AddressingMode::Implicit => 1,
      AddressingMode::Immediate | AddressingMode::ZeroPage | AddressingMode::ZeroPageX | AddressingMode::ZeroPageY | AddressingMode::IndexedIndirect | AddressingMode::IndirectIndexed | AddressingMode::Relative => 2,
      AddressingMode::Absolute | AddressingMode::AbsoluteX | AddressingMode::AbsoluteY | AddressingMode::Indirect => 3
    };

    Instruction {
      byte_size: byte_size,
      cycles: 0,
      op_code: 0,
      instruction_type: instruction_type,
      addressing_mode: addressing_mode,
      operation: operation,
    }
  }

  fn high_param(memory: &mut Memory, low_address: MemoryAddress) -> Byte {
    let shift = memory.read(low_address + 1) << 8;
    memory.read(low_address) | shift
  }

  fn calculate_page_crossed_penalty(address_before: MemoryAddress, address_after: MemoryAddress) -> u8 {
    if (address_after & 0xFF00) != (address_before & 0xFF00) {
      1
    } else {
      0
    }
  }

  fn execute(&self, cpu: &Cpu, memory: &mut Memory, starting_address: MemoryAddress) -> Byte {
    let low_address = starting_address + 1;

    let parameter = match &self.addressing_mode {
      AddressingMode::Implicit => Parameter{value: 0, address: 0, extra_cycles: 0},
      AddressingMode::Immediate => Parameter{value: memory.read(low_address), address: low_address, extra_cycles: 0},
      AddressingMode::ZeroPage => {
        let address = memory.read(low_address);
        Parameter{value: memory.read(address), address: address, extra_cycles: 0}
      },
      AddressingMode::ZeroPageX => {
        let address = memory.read(low_address) + cpu.x;
        Parameter{value: memory.read(address & 0xFF), address: address, extra_cycles: 0}
      },
      AddressingMode::ZeroPageY => {
        let address = memory.read(low_address) + cpu.y;
        Parameter{value: memory.read(address), address: address, extra_cycles: 0}
      },
      AddressingMode::Absolute => {
        let address = Instruction::high_param(memory, low_address);
        Parameter{value: memory.read(address), address: address, extra_cycles: 0}
      },
      AddressingMode::AbsoluteX => {
        let address = Instruction::high_param(memory, low_address) + cpu.x;
        let value = memory.read(address);
        Parameter{value: value, address: address, extra_cycles: Instruction::calculate_page_crossed_penalty(value, address)}
      },
      AddressingMode::AbsoluteY => {
        let address = Instruction::high_param(memory, low_address) + cpu.y;
        let value = memory.read(address);
        Parameter{value: value, address: address, extra_cycles: Instruction::calculate_page_crossed_penalty(value, address)}
      },
      AddressingMode::Relative => {
        let value = memory.read(low_address);
        Parameter{value: value, address: starting_address + value, extra_cycles: 0}
      }
    };

    0
  }
}
