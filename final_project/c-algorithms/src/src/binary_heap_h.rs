use crate::translation_utils::*;
pub use crate::src::binary_heap_c::_BinaryHeap;
pub use crate::src::binary_heap_c::binary_heap_new;
pub use crate::src::binary_heap_c::binary_heap_insert;
pub use crate::src::binary_heap_c::binary_heap_num_entries;
pub use crate::src::binary_heap_c::binary_heap_pop;
pub use crate::src::binary_heap_c::binary_heap_free;

pub type BinaryHeapType = i32;
macro_rules! BINARY_HEAP_TYPE_MIN { () => { 0 } }
pub(crate) use BINARY_HEAP_TYPE_MIN;
macro_rules! BINARY_HEAP_TYPE_MAX { () => { 1 } }
pub(crate) use BINARY_HEAP_TYPE_MAX;


pub type BinaryHeapValue = VoidPtr;


pub type BinaryHeapCompareFunc = FuncPtr<fn(BinaryHeapValue, BinaryHeapValue) -> i32>;


pub type BinaryHeap = _BinaryHeap;


macro_rules! ALGORITHM_BINARY_HEAP_H { () => { } }
pub(crate) use ALGORITHM_BINARY_HEAP_H;


macro_rules! BINARY_HEAP_NULL { () => { NULL!() } }
pub(crate) use BINARY_HEAP_NULL;


