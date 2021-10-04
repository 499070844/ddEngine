mod mmap;
mod alloc;

#[cfg(test)]
mod test {
    use crate::interpreter::heap::alloc::ImplictAlloc;

    use super::mmap;
    use std::{process};
    #[test]
    fn mmap_work() {
        let mut alloc = ImplictAlloc::new();
        println!("I'm in pid: {}", process::id());
        println!("start: {:#?} ----- end: {:#?}", alloc.mmap.p_start(), alloc.mmap.p_end());
        // unsafe {
        //     let h = alloc.start;
        //     let block_size = *(h as *const usize);
        //     let end_pointer = h.add(block_size);
        //     println!("Block size: {}, End size: {}", block_size, *(end_pointer as *const usize));
        // }
    }

    #[test]
    fn mask_work() {
        let t = 0b10001usize;
        assert_eq!(ImplictAlloc::block_size(t), 0b10000);
        assert_eq!(ImplictAlloc::block_size(0b110000), 0b110000);
        assert_eq!(ImplictAlloc::block_size(0b110010), 0b110010);
        assert_eq!(ImplictAlloc::block_size(0b110011), 0b110010);
        assert_eq!(ImplictAlloc::block_size(0b0), 0b0);
        assert_eq!(ImplictAlloc::block_size(0b1), 0b0);
    }
}