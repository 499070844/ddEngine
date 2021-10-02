mod mmap;
mod alloc;

#[cfg(test)]
mod test {
    use crate::interpreter::heap::alloc::ImplictAlloc;

    use super::mmap;
    use std::process;
    #[test]
    fn mmap_work() {
        let mut alloc = ImplictAlloc::new();
        println!("I'm in pid: {}", process::id());
        let mut a = 0;
        loop {
            std::thread::sleep_ms(1000 * 5);
            println!("start: {:#?} ----- end: {:#?}", alloc.mmap.p_start(), alloc.mmap.p_end());
            a += 1;
        }
    }

    #[test]
    fn mask_work() {
        let t = 0b10001usize;
        assert_eq!(ImplictAlloc::get_size(t), 0b10000);
        assert_eq!(ImplictAlloc::get_size(0b110000), 0b110000);
        assert_eq!(ImplictAlloc::get_size(0b110010), 0b110010);
        assert_eq!(ImplictAlloc::get_size(0b110011), 0b110010);
        assert_eq!(ImplictAlloc::get_size(0b0), 0b0);
        assert_eq!(ImplictAlloc::get_size(0b1), 0b0);
    }
}