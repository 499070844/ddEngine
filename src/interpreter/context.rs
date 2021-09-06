use std::collections::HashMap;


pub type RegisterName = u16;


pub struct Context {
  register: HashMap<RegisterName, usize>,
  pc: usize,
  condition: usize,
}