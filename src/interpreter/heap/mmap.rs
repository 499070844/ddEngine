

#[cfg(unix)] mod _unix {
    use std::ops::Add;

    use libc;
    pub struct Mmap {
        start: *mut u8,
        size: usize,
        end: *mut u8,
    }

    impl Mmap {
        pub fn new(size: usize) -> Mmap {
            unsafe {
                let p = libc::mmap(
                    core::ptr::null_mut(),
                    size,
                    libc::PROT_WRITE | libc::PROT_READ,
                    libc::MAP_ANON | libc::MAP_PRIVATE,
                    -1,
                    0
                );
                // TODO: We can perform libc::madvise here to expect page references in some order
                if p == libc::MAP_FAILED {
                    panic!("Mmap failed!!!!");
                }

                Mmap {
                    start: p as *mut u8,
                    size,
                    end: ( p as usize + size ) as *mut u8,
                }
            }
        }

        #[inline]
        pub fn p_start(&self) -> *mut u8 {
            self.start
        }

        #[inline]
        pub fn size(&self) -> usize {
            self.size
        }

        #[inline]
        pub fn p_end(&self) -> *mut u8 {
            self.end
        }

        #[inline]
        pub fn align(&self) -> *mut u8 {
            unsafe { self.start.add(1) }
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