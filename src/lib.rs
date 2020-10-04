use std::ptr;
/// A `SafeSequence` provides `try_*` implementations for sequential data structures
/// like `Vec`, `VecDeque` and `String`
pub trait SafeSequence<T> {
    /// Check if `idx` exists in the collection
    fn has_index(&self, idx: usize) -> bool;
    /// Try to get an element
    ///
    /// If `idx` exists, then the element is returned, otherwise `None` is returned
    fn try_get(&self, idx: usize) -> Option<&T>;
    /// Try to remove an element
    ///
    /// If `idx` exists, then the corresponding element is returned, and all the items
    /// in the collection are moved to fill that spot
    fn try_remove(&mut self, idx: usize) -> Option<T>;
}

impl<T> SafeSequence<T> for Vec<T> {
    fn has_index(&self, idx: usize) -> bool {
        idx < self.len()
    }
    fn try_get(&self, idx: usize) -> Option<&T> {
        if self.has_index(idx) {
            Some(unsafe { self.get_unchecked(idx) })
        } else {
            None
        }
    }
    fn try_remove(&mut self, idx: usize) -> Option<T> {
        if self.has_index(idx) {
            Some(unsafe {
                // infallible
                let ret;
                {
                    // the place we are taking from.
                    let ptr = self.as_mut_ptr().add(idx);
                    // copy it out, unsafely having a copy of the value on
                    // the stack and in the vector at the same time.
                    ret = ptr::read(ptr);
                    // Shift everything down to fill in that spot.
                    ptr::copy(ptr.offset(1), ptr, self.len() - idx - 1);
                }
                self.set_len(self.len() - 1);
                ret
            })
        } else {
            None
        }
    }
}

#[test]
fn test_vec_has_index() {
    let v = vec![1, 2, 3];
    assert!(v.has_index(0));
    assert!(!v.has_index(3));
}

#[test]
fn test_vec_try_get() {
    let v = vec![1, 2, 3, 4];
    assert_eq!(v.try_get(0), Some(&1));
    assert_eq!(v.try_get(3), Some(&4));
    assert!(v.try_get(4).is_none());
}

#[test]
fn test_vec_try_remove() {
    let mut v = vec![1, 2, 3, 4];
    assert_eq!(v.try_remove(0), Some(1));
    assert_eq!(v.try_remove(2), Some(4));
    // now: [2, 3]
    assert!(v.try_remove(2).is_none());
}
