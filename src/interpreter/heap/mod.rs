mod mmap;
mod alloc;

#[cfg(test)]
mod test {
    use crate::interpreter::heap::alloc::ImplictAlloc;

    use super::mmap;
    use std::process;
    #[test]
    fn mmap_work() {
        let alloc = ImplictAlloc::new();
        println!("I'm in pid: {}", process::id());
        let mut a = 0;
        loop {
            std::thread::sleep_ms(1000 * 5);
            println!("start: {:#?} ----- end: {:#?}", alloc.mmap.p_start(), alloc.mmap.p_end());
            a += 1;
        }
    }
}