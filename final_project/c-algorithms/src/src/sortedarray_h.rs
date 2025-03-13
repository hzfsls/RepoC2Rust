use crate::translation_utils::*;
pub use crate::src::sortedarray_c::sortedarray_remove_range;
pub use crate::src::sortedarray_c::sortedarray_clear;
pub use crate::src::sortedarray_c::sortedarray_remove;
pub use crate::src::sortedarray_c::sortedarray_index_of;
pub use crate::src::sortedarray_c::sortedarray_insert;
pub use crate::src::sortedarray_c::sortedarray_length;
pub use crate::src::sortedarray_c::_SortedArray;
pub use crate::src::sortedarray_c::sortedarray_free;
pub use crate::src::sortedarray_c::sortedarray_get;
pub use crate::src::sortedarray_c::sortedarray_new;

pub type SortedArrayValue = VoidPtr;


pub type SortedArray = _SortedArray;


pub type SortedArrayEqualFunc = FuncPtr<fn(SortedArrayValue, SortedArrayValue) -> i32>;


pub type SortedArrayCompareFunc = FuncPtr<fn(SortedArrayValue, SortedArrayValue) -> i32>;


macro_rules! ALGORITHM_SORTEDARRAY_H { () => { } }
pub(crate) use ALGORITHM_SORTEDARRAY_H;


