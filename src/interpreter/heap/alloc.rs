use core::panic;

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
    /// ### header pointer
    /// We need we need add *HEAD_SIZE* when returning a block pointer
    pub start: *mut u8,
}

impl ImplictAlloc {
    pub fn new() -> ImplictAlloc {
        let mut alloc = ImplictAlloc {
            // mmap: Mmap::new(8 * 1024),
            mmap: Mmap::new(8 * 1023),
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
            // Here must be 16 的倍数！！ end 那里可以用来对齐
            if self.mmap.size() < 16 {
                panic!("Can't init memory less than 16");
            }
            let (init_size, end_size) = Self::bottom_align(self.mmap.size() - first_offset);
            *first_header = init_size;
            // Init end block
            // TODO: We need calculate end pointer
            let end_pointer = self.mmap.p_end().sub(end_size) as *mut usize;
            *end_pointer = 0 | 0x01;
            println!("first header pointer: {:#?}", first_header);
            println!(
                "We got the first block with size {}, and end block with size {}",
                *(first_header as *mut usize),
                *(end_pointer as *mut usize)
            );
            self.start = first_header as *mut u8;
        }
    }

    fn spilt(&self, h_p: *mut u8, a_size: usize, b_size: usize) {

    }

    /// Step 1: Find a fit block
    /// Step 2: Split the block
    /// Step 3: Check alignment (pad)
    /// 地址是16n header地址是 (8 + 16n) * 13 + 8 => (13 * 8 + 16n * 13) + 8 无法被16整除，所以size一定是16的倍数
    /// Size 一定是 16的倍数
    pub fn first_fit(&self, size: usize) -> *mut u8 {
        if size == 0 { panic!("Size can't be 0"); }
        unsafe {
          // Step 1
            let mut h_p = self.start;
            let new_size = Self::padding(size + HEAD_SIZE);
            while !Self::is_end(*(h_p as *mut usize))
                && (!Self::is_free(*(h_p as *mut usize))
                    // 为什么要 >= ?
                    // 因为每次分配都要切分 原来空闲的 Block。要是 原 Block == 新 Block 则无法切分
                    || Self::block_size(*(h_p as *mut usize)) <= new_size)
            {
                h_p = h_p.add(Self::block_size(*(h_p as *mut usize)));
            }
            if Self::is_end(*(h_p as *mut usize)) { panic!("Can't find a fit block after traveling all blocks!!!!"); }
            // Step 2
            // Update fit block size
            let _h_p = h_p as *mut usize;
            // save original block size
            let elder_size = *_h_p;
            *_h_p = new_size | 0x01;
            // Spilt free block
            let _free_block_h = h_p.add(new_size) as *mut usize;
            println!("Spilt the block, the next block at {:#?}", _free_block_h);
            *_free_block_h = elder_size - new_size;
            h_p.add(HEAD_SIZE)
        }
    }

    /// Return size after padding
    /// # Painc
    /// - size + 16 overflow
    #[inline]
    fn padding(size: usize) -> usize {
        let l4sb = size & ALIGN_MASK;
        if l4sb == 0 {
            return size
        }
        let pad = ALIGNMENT - l4sb;
        let res = size + pad;
        if res < size {
            panic!("Can't padding the block cause by overflow");
        }
        res
    }
    
    /// Calculate inital block size, if size can't align 16 
    /// # Return
    /// - (A, B)
    /// 
    /// A: size after bottom align
    /// 
    /// B: number bigger than 16
    /// - (0, x)
    /// the input which less than 16
    // TODO: 这里 尾端对齐 可以更加动态一些
    #[inline]
    fn bottom_align(size: usize) -> (usize, usize) {
        let l4sb = size & ALIGN_MASK;
        if l4sb < 8 {
            let res = size - l4sb - ALIGNMENT;
            if res > size || res == 0 { panic!("Overflow in bottom align"); }
            return (res, l4sb + ALIGNMENT);
        }
        (size - l4sb, l4sb)
    }

    #[inline]
    fn is_free(size: usize) -> bool {
        if size & ALIGN_MASK != 0 {
            return false;
        }
        true
    }

    /// size = 0x00 | 0x01
    #[inline]
    fn is_end(size: usize) -> bool {
        if size == 1 {
            return true;
        }
        false
    }

    #[inline]
    pub fn block_size(size: usize) -> usize {
        !(0x01 & size) & size
    }

    #[inline]
    fn block_pointer(h_p: *mut u8) -> *mut u8 {
      unsafe { h_p.add(HEAD_SIZE) }
    }

    fn is_align(p: *mut u8) -> bool {
        p as usize & ALIGN_MASK == 0
    }
}
#[cfg(test)]
mod alloc_test {
    use super::*;
    use rand::prelude::*;
    #[test]
    fn padding_test() {
        assert_eq!(16, ImplictAlloc::padding(15));
        assert_eq!(16, ImplictAlloc::padding(14));
        assert_eq!(1600, ImplictAlloc::padding(1600));
        assert_eq!(1600, ImplictAlloc::padding(1598));
        assert_eq!(1600, ImplictAlloc::padding(1598));
        assert_eq!(288, ImplictAlloc::padding(275));
    }

    #[test]
    #[should_panic]
    fn first_fit_work() {
        let mut rng = rand::thread_rng();
        let alloc = ImplictAlloc::new();
        for _ in 0..1000 {
            let r_size: usize = rng.gen_range(1..1000);
            println!("rng gen size {}", r_size);
            let p = alloc.first_fit(r_size);
        unsafe {
            println!("size after padding: {} p: {:#?}", ImplictAlloc::block_size(*(p.sub(HEAD_SIZE) as *mut usize)), p); }
            assert!(ImplictAlloc::is_align(p));
        }
    }

    #[test]
    fn bottom_align_work() {
        assert_eq!(16, ImplictAlloc::bottom_align(29).0);
        assert_eq!(13, ImplictAlloc::bottom_align(29).1);
        assert_eq!(16, ImplictAlloc::bottom_align(33).0);
        assert_eq!(17, ImplictAlloc::bottom_align(33).1);
    }

    #[test]
    #[should_panic]
    fn bottom_align_panic() {
        ImplictAlloc::bottom_align(16);
    }
}