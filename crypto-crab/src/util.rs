#[inline(always)]
pub fn array<T, const N: usize>(mut closure: impl FnMut(usize) -> T) -> [T; N] {
    let mut array = std::mem::MaybeUninit::uninit();

    struct PartialRawSlice<T> {
        ptr: *mut T,
        len: usize,
    }

    impl<T> Drop for PartialRawSlice<T> {
        fn drop(&mut self) {
            // SAFE: all values (0..self.len) are initialized
            unsafe { std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr, self.len)) };
        }
    }

    let mut raw_slice = PartialRawSlice {
        ptr: array.as_mut_ptr() as *mut T,
        len: 0,
    };

    for i in 0..N {
        // SAFE: the ptr point to a valid memory location to store a type T
        unsafe { std::ptr::write(raw_slice.ptr.add(i), closure(i)) };
        raw_slice.len += 1;
    }

    std::mem::forget(raw_slice);

    // SAFE: at this point, all values are initialized
    unsafe { array.assume_init() }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    struct Panic(Arc<AtomicUsize>);
    impl Panic {
        fn new(i: usize, drops: Arc<AtomicUsize>) -> Self {
            if i == 3 {
                panic!()
            }
            Self(drops)
        }
    }
    impl Drop for Panic {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn test_array() {
        let array: [usize; 4] = array(|i| i);
        assert_eq!(array, [0, 1, 2, 3])
    }

    #[test]
    fn test_array_properly_drops() {
        let drops = Arc::new(AtomicUsize::new(0));
        std::panic::set_hook(Box::new(|_| {}));
        let _ = std::panic::catch_unwind(|| {
            let _: [Panic; 4] = array(|i| Panic::new(i, drops.clone()));
        });
        assert_eq!(3, drops.load(Ordering::SeqCst));
    }
}
