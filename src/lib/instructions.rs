use lib::memory::Memory;

pub enum InstructionType {
  LDA,
}

pub enum AddressingMode {
  Immediate,
  ZeroPage,
}

pub struct Instruction {
  instruction_type: InstructionType,
  addressingMode: AddressingMode,
  operation: Box<Fn(&Instruction, &mut Memory, u8)>,
}

impl Instruction {
  fn new(
    instruction_type: InstructionType,
    addressingMode: AddressingMode,
    operation: Box<Fn(&Instruction, &mut Memory, u8)>,
  ) -> Instruction {
    Instruction {
      instruction_type: instruction_type,
      addressingMode: addressingMode,
      operation: operation,
    }
  }
}
