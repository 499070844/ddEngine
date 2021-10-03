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
            println!("first header pointer: {:#?}", first_header);
            println!(
                "We get a block with size {}",
                *(first_header.sub(8) as *mut usize)
            );
            self.start = first_header as *mut u8;
            *self.mmap.p_end() = 0 | 0x01;
        }
    }

    /// Step 1: Find a fit block
    /// Step 2: Split the block
    /// Step 3: Check alignment (pad)
    pub fn first_fit(&self, size: usize) -> *mut u8 {
        unsafe {
          // Step 1
            let mut h_p = self.start;
            while !Self::is_end(*(h_p as *mut usize))
                && (!Self::is_free(*(h_p as *mut usize))
                    || Self::get_size(*(h_p as *mut usize)) <= size)
            {
                h_p = h_p.add(*(h_p as *mut usize));
            }
            if Self::is_end(*(h_p as *mut usize)) { panic!("Can't find a fit block after traveling all blocks!!!!"); }
            // Step 2
            // Update fit block size
            let _h_p = h_p as *mut usize;
            // save original block size
            let elder_size = *_h_p;
            let new_size = size + HEAD_SIZE;
            // | H | B B B B B
            //     16 (16-5) + 5 + HEAD_SIZE
            // let new_size = padding(size) + HEAD_SIZE;
            *_h_p = new_size | 0x01;
            let _pair_p = h_p.add(new_size) as *mut usize;
            *_pair_p = elder_size - new_size;
        }
        todo!()
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
    pub fn get_size(size: usize) -> usize {
        !(0x01usize & size) & size
    }

    #[inline]
    fn block_pointer(h_p: *mut u8) -> *mut u8 {
      unsafe { h_p.add(HEAD_SIZE) }
    }
}
#[cfg(test)]
mod alloc_test {
    use super::*;
    #[test]
    fn padding_test() {
        assert_eq!(16, ImplictAlloc::padding(15));
        assert_eq!(16, ImplictAlloc::padding(14));
        assert_eq!(1600, ImplictAlloc::padding(1600));
        assert_eq!(1600, ImplictAlloc::padding(1598));
        assert_eq!(1600, ImplictAlloc::padding(1598));
    }
}