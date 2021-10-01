use std::ops::Add;

use super::mmap::Mmap;

/// 8 bytes one header
const HEAD_SIZE: usize = 8;
/// Every address aligned 16 bytes
const ALIGNMENT: usize = 16;

const ALIGN_MASK: usize = 0xf;

/// Step 1: New a Mmap
/// Step 2: 
///   Init: Find a pleace and then write first Header (mmap.size - align)
///   Set end block 
/// Malloc: 
///   First: Write the header before block where the first one fit size block then update the block header before the current block
///   block size need (size | 0x01) when the block is used
pub struct ImplictAlloc {
  pub mmap: Mmap,
  pub start: *mut u8,
}


impl ImplictAlloc {
  pub fn new() -> ImplictAlloc {
    let mut alloc = ImplictAlloc {
      mmap: Mmap::new(8 * 1024),
      start: std::ptr::null_mut(),
    };
    alloc.init();
    alloc
  }

  /// ## TODO: 
  /// - 0 + 8 = 8 8不能被 16 对齐
  /// - 8 + 8 = 16 可以对齐， 所以第一个块需要偏移 8位
  /// - 如何通用的找到第一个块的位置！！
  /// - 设 需要偏移 x 位使得 x + HEADER_SIZE = ALIGNMENT (8 or 16)
  #[inline]
  fn init(&mut self) {
    let start = self.mmap.p_start();
    unsafe {
      let first_offset = ALIGNMENT - HEAD_SIZE;
      let first_header: *mut usize = start.add(first_offset) as *mut usize;
      if !*first_header == 0 {
        panic!("Can't not init memory again");
      }
      // First Block is free so we don't need OR 0x01
      *first_header = self.mmap.size() - first_offset - 1; // - 1 是 end block
      let first_payload = start.add(first_offset + HEAD_SIZE);
      println!("first pointer: {:#?}", first_payload);
      println!("We get a block with size {}", *(first_payload.sub(8) as *mut usize));
      self.start = first_payload;
      *self.mmap.p_end() = 0 | 0x01;
    }
  }

  pub fn first_fit(&self) {
    unsafe {
      let h_p = self.start.sub(HEAD_SIZE) as *mut usize;
    }
    
    loop {
      unsafe {
        let header = *(self.start as *mut usize);
      }
    }
  }

  #[inline]
  fn is_free(size: usize) -> bool {
    if size & ALIGN_MASK != 0 {
      return false
    }
    true
  }

}