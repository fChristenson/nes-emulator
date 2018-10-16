pub struct P {
  pub value: u8
}

impl P {
  fn get_bit(&self, n: u8) -> u8 {
    let shift = self.value >> n; //TODO: how to impl >>>
    shift & 0x1
  }

  fn toggle_bit(&mut self, n: u8) {
    self.value = self.value ^ (1 << n)
  }

  fn set_bit(&mut self, n: u8, v: u8) {
    if self.get_bit(n) != v {
      self.toggle_bit(n)
    }
  }
}

pub struct Cpu {
  pub x: u8,
  pub y: u8,
  pub a: u8,
  pub p: P,
  pub program_counter: u8,
  pub stack_pointer: u8,
}
