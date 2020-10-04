mod vec_fallible;
type EmptyResult = Result<(), ()>;
/// A `FallibleSequence` provides `try_*` implementations for sequential data structures
/// like `Vec`, `VecDeque` and `String`
pub trait FallibleSequence {
    type TargetRaw;
    type TargetRef;
    /// Check if `idx` exists in the collection
    fn has_index(&self, idx: usize) -> bool;
    /// Try to get an element
    ///
    /// If `idx` exists, then the element is returned, otherwise `None` is returned
    fn try_get(&self, idx: usize) -> Option<&Self::TargetRef>;
    /// Try to remove an element
    ///
    /// If `idx` exists, then the corresponding element is returned, and all the items
    /// in the collection are moved to fill that spot
    fn try_remove(&mut self, idx: usize) -> Option<Self::TargetRaw>;
    /// Try to insert an element at an existing index
    ///
    /// If `idx` exists, then the `element` is inserted at that index, and all the other
    /// elements (inclusive of the one previously at `idx`) are moved to the right
    fn try_insert(&mut self, idx: usize, element: Self::TargetRaw) -> EmptyResult;
}
