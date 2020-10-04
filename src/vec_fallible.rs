use super::*;
use std::ptr;
impl<T> FallibleSequence for Vec<T> {
    type TargetRaw = T;
    type TargetRef = T;
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
    fn try_insert(&mut self, idx: usize, element: T) -> EmptyResult {
        if self.has_index(idx) {
            let len = self.len();
            if len == self.capacity() {
                self.reserve(1);
            }
            unsafe {
                // infallible
                // The spot to put the new value
                {
                    let p = self.as_mut_ptr().add(idx);
                    // Shift everything over to make space. (Duplicating the
                    // `index`th element into two consecutive places.)
                    ptr::copy(p, p.offset(1), len - idx);
                    // Write it in, overwriting the first copy of the `index`th
                    // element.
                    ptr::write(p, element);
                }
                self.set_len(len + 1);
            }
            Ok(())
        } else {
            Err(())
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

#[test]
fn test_vec_try_insert() {
    let mut v = vec![1, 2, 4];
    assert!(v.try_insert(2, 3).is_ok());
    assert!(v.try_insert(4, 5).is_err());
    assert_eq!(v, vec![1, 2, 3, 4]);
}
