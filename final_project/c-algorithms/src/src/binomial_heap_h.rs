use crate::translation_utils::*;
pub use crate::src::binomial_heap_c::binomial_heap_insert;
pub use crate::src::binomial_heap_c::binomial_heap_num_entries;
pub use crate::src::binomial_heap_c::_BinomialHeap;
pub use crate::src::binomial_heap_c::binomial_heap_free;
pub use crate::src::binomial_heap_c::binomial_heap_pop;
pub use crate::src::binomial_heap_c::binomial_heap_new;

pub type BinomialHeapType = i32;
macro_rules! BINOMIAL_HEAP_TYPE_MIN { () => { 0 } }
pub(crate) use BINOMIAL_HEAP_TYPE_MIN;
macro_rules! BINOMIAL_HEAP_TYPE_MAX { () => { 1 } }
pub(crate) use BINOMIAL_HEAP_TYPE_MAX;


pub type BinomialHeapValue = VoidPtr;


pub type BinomialHeapCompareFunc = FuncPtr<fn(BinomialHeapValue, BinomialHeapValue) -> i32>;


pub type BinomialHeap = _BinomialHeap;


macro_rules! ALGORITHM_BINOMIAL_HEAP_H { () => { } }
pub(crate) use ALGORITHM_BINOMIAL_HEAP_H;


macro_rules! BINOMIAL_HEAP_NULL { () => { NULL!() } }
pub(crate) use BINOMIAL_HEAP_NULL;


