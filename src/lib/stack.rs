use lib::memory::Memory;

pub struct Stack {
  stack_pointer: u8,
}

impl Stack {
    pub fn new() -> Stack {
      Stack{stack_pointer: 0x01FF}
    }

    pub fn push(&mut self, memory: &mut Memory, value: u8) {
    memory.write(self.stack_pointer, value);
    self.stack_pointer -= 1
  }

  pub fn pop(&mut self, memory: &mut Memory) -> u8 {
    let value = memory.read(self.stack_pointer);
    self.stack_pointer += 1;
    value
  }
}
