

#[cfg(unix)] mod _unix {
    use libc;
    pub struct Mmap {
        pub start: *mut u8,
        pub size: usize,
    }

    impl Mmap {
        pub fn new(size: usize) -> Mmap {
            unsafe {
                let p = libc::mmap(core::ptr::null_mut(),
                    size,
                    libc::PROT_WRITE | libc::PROT_READ,
                    libc::MAP_ANON | libc::MAP_PRIVATE,
                    -1,
                    0
                );
                return Mmap {
                    start: p as *mut u8,
                    size,
                }
            }
        }
    }

    impl Drop for Mmap {
        fn drop(&mut self) {
            unsafe { libc::munmap(self.start as *mut _, self.size as _); }
        }
    }
}

#[cfg(unix)]
pub use _unix::*;