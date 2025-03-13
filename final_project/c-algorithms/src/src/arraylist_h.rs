use crate::translation_utils::*;
pub use crate::src::arraylist_c::arraylist_insert;
pub use crate::src::arraylist_c::arraylist_index_of;
pub use crate::src::arraylist_c::arraylist_remove;
pub use crate::src::arraylist_c::arraylist_append;
pub use crate::src::arraylist_c::arraylist_prepend;
pub use crate::src::arraylist_c::arraylist_clear;
pub use crate::src::arraylist_c::arraylist_free;
pub use crate::src::arraylist_c::arraylist_remove_range;
pub use crate::src::arraylist_c::arraylist_new;
pub use crate::src::arraylist_c::arraylist_sort;

pub type ArrayListValue = VoidPtr;


pub type ArrayList = _ArrayList;


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _ArrayList {
    pub data: Ptr<ArrayListValue>,
    pub length: u32,
    pub _alloced: u32,
}


pub type ArrayListEqualFunc = FuncPtr<fn(ArrayListValue, ArrayListValue) -> i32>;


pub type ArrayListCompareFunc = FuncPtr<fn(ArrayListValue, ArrayListValue) -> i32>;


macro_rules! ALGORITHM_ARRAYLIST_H { () => { } }
pub(crate) use ALGORITHM_ARRAYLIST_H;


