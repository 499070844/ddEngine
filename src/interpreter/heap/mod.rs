mod mmap;
mod alloc;

#[cfg(test)]
mod test {
    use super::mmap;
    use std::process;
    #[test]
    fn mmap_work() {
        let map = mmap::Mmap::new(2 * 1024 * 1024 * 1024 );
        println!("I'm in pid: {}", process::id());
        let mut a = 0;
        loop {
            std::thread::sleep_ms(1000 * 5);
            println!("{:#?}", &map.start);
            a += 1;
        }
    }
}