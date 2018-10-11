use lib::cpu::Cpu;
use lib::memory::Memory;

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
  IndirectIndexed,
}

pub struct Instruction {
  op_code: u8,
  cycles: u8,
  byte_size: u8,
  instruction_type: InstructionType,
  addressing_mode: AddressingMode,
  operation: Box<Fn(&Cpu, u8, &Parameter) -> u8>,
}

pub struct Parameter {
  value: u8,
  address: u8,
  extra_cycles: u8,
}

impl Instruction {
  pub fn new(
    instruction_type: InstructionType,
    addressing_mode: AddressingMode,
    operation: Box<Fn(&Cpu, u8, &Parameter) -> u8>,
  ) -> Instruction {
    let byte_size = match addressing_mode {
      AddressingMode::Accumulator | AddressingMode::Implicit => 1,
      AddressingMode::Immediate
      | AddressingMode::ZeroPage
      | AddressingMode::ZeroPageX
      | AddressingMode::ZeroPageY
      | AddressingMode::IndexedIndirect
      | AddressingMode::IndirectIndexed
      | AddressingMode::Relative => 2,
      AddressingMode::Absolute
      | AddressingMode::AbsoluteX
      | AddressingMode::AbsoluteY
      | AddressingMode::Indirect => 3,
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

  fn high_param(memory: &mut Memory, low_address: u8) -> u8 {
    let shift = memory.read(low_address + 1) << 8;
    memory.read(low_address) | shift
  }

  fn calculate_page_crossed_penalty(address_before: u8, address_after: u8) -> u8 {
    if (address_after & 0xFF00) != (address_before & 0xFF00) {
      1
    } else {
      0
    }
  }

  fn execute(&self, cpu: &Cpu, memory: &mut Memory, starting_address: u8) -> u8 {
    let low_address = starting_address + 1;

    let parameter = match &self.addressing_mode {
      AddressingMode::Implicit => Parameter {
        value: 0,
        address: 0,
        extra_cycles: 0,
      },
      AddressingMode::Immediate => Parameter {
        value: memory.read(low_address),
        address: low_address,
        extra_cycles: 0,
      },
      AddressingMode::ZeroPage => {
        let address = memory.read(low_address);
        Parameter {
          value: memory.read(address),
          address: address,
          extra_cycles: 0,
        }
      }
      AddressingMode::ZeroPageX => {
        let address = memory.read(low_address) + cpu.x;
        Parameter {
          value: memory.read(address & 0xFF),
          address: address,
          extra_cycles: 0,
        }
      }
      AddressingMode::ZeroPageY => {
        let address = memory.read(low_address) + cpu.y;
        Parameter {
          value: memory.read(address),
          address: address,
          extra_cycles: 0,
        }
      }
      AddressingMode::Absolute => {
        let address = Instruction::high_param(memory, low_address);
        Parameter {
          value: memory.read(address),
          address: address,
          extra_cycles: 0,
        }
      }
      AddressingMode::AbsoluteX => {
        let address = Instruction::high_param(memory, low_address) + cpu.x;
        let value = memory.read(address);
        Parameter {
          value: value,
          address: address,
          extra_cycles: Instruction::calculate_page_crossed_penalty(value, address),
        }
      }
      AddressingMode::AbsoluteY => {
        let address = Instruction::high_param(memory, low_address) + cpu.y;
        let value = memory.read(address);
        Parameter {
          value: value,
          address: address,
          extra_cycles: Instruction::calculate_page_crossed_penalty(value, address),
        }
      }
      AddressingMode::Relative => {
        let value = memory.read(low_address);
        Parameter {
          value: value,
          address: starting_address + value,
          extra_cycles: 0,
        }
      }
      AddressingMode::Accumulator => Parameter {
        value: cpu.a,
        address: low_address,
        extra_cycles: 0,
      },
      AddressingMode::Indirect => {
        let address_low = Instruction::high_param(memory, low_address);
        let value_low = memory.read(address_low);
        let mut address_high = address_low + 1;

        if (address_low & 0xFF) == 0xFF {
          address_high = address_low & 0xFF00;
        }

        let value_high = memory.read(address_high);

        Parameter {
          value: (value_high << 8) | value_low,
          address: address_low,
          extra_cycles: 0,
        }
      }
      AddressingMode::IndexedIndirect => {
        let value = memory.read(low_address) + cpu.x;
        let extra_cycles = Instruction::calculate_page_crossed_penalty(low_address, value);
        let value_low = memory.read(value & 0xFF);
        let value_high = memory.read((value + 1) & 0xFF);
        let value_true = memory.read((value_high << 8) | value_low);

        Parameter {
          value: value_true,
          address: (value_high << 8) | value_low,
          extra_cycles: extra_cycles,
        }
      }
      AddressingMode::IndirectIndexed => {
        let value = memory.read(low_address);
        let value_low = memory.read(value & 0xFF);
        let value_high = memory.read((value + 1) & 0xFF);
        let address_true = ((value_high << 8) | value_low) + 1;
        let value_true = memory.read(address_true);
        let extra_cycles =
          Instruction::calculate_page_crossed_penalty((value_high << 8) | value_low, address_true);

        Parameter {
          value: value_true,
          address: address_true,
          extra_cycles: extra_cycles,
        }
      }
    };

    &self.cycles + parameter.extra_cycles + (&self.operation)(cpu, starting_address, &parameter)
  }
}
