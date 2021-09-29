use super::mmap::Mmap;


pub struct ImplictAlloc {
  pub mmap: Mmap,
}

impl ImplictAlloc {
  pub fn new() -> ImplictAlloc {
    ImplictAlloc {
      mmap: Mmap::new(8 * 1024),
    }
  }
}