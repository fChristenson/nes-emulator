pub struct P {
  pub value: u8
}

pub enum PRegisterFlag {
  C,
  Z,
  I,
  D,
  B,
  U,
  O,
  N
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

  fn get_flag_value(&self, flag: PRegisterFlag) -> u8 {
    match flag {
      PRegisterFlag::C => self.get_bit(0),
      PRegisterFlag::Z => self.get_bit(1),
      PRegisterFlag::I => self.get_bit(2),
      PRegisterFlag::D => self.get_bit(3),
      PRegisterFlag::B => self.get_bit(4),
      PRegisterFlag::U => 1,
      PRegisterFlag::O => self.get_bit(6),
      PRegisterFlag::N => self.get_bit(7),
    }
  }

  fn set_flag_value(&mut self, flag: PRegisterFlag, value: u8) {
    match flag {
      PRegisterFlag::C => self.set_bit(0, value),
      PRegisterFlag::Z => self.set_bit(1, value),
      PRegisterFlag::I => self.set_bit(2, value),
      PRegisterFlag::D => self.set_bit(3, value),
      PRegisterFlag::B => self.set_bit(4, value),
      PRegisterFlag::U => self.set_bit(4, value),
      PRegisterFlag::O => self.set_bit(6, value),
      PRegisterFlag::N => self.set_bit(7, value),
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

impl Cpu {
  pub fn new() -> Cpu {
    let p = P{value: 0};
    Cpu{x: 0, y: 0, a: 0, p: p, program_counter: 0, stack_pointer: 0}
  }
}