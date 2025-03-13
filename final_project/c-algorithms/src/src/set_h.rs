use crate::translation_utils::*;
pub use crate::src::set_c::set_remove;
pub use crate::src::set_c::set_intersection;
pub use crate::src::set_c::set_free;
pub use crate::src::set_c::set_insert;
pub use crate::src::set_c::set_iter_next;
pub use crate::src::set_c::set_to_array;
pub use crate::src::set_c::set_new;
pub use crate::src::set_c::set_iter_has_more;
pub use crate::src::set_c::_SetEntry;
pub use crate::src::set_c::set_union;
pub use crate::src::set_c::set_iterate;
pub use crate::src::set_c::set_register_free_function;
pub use crate::src::set_c::set_num_entries;
pub use crate::src::set_c::_Set;
pub use crate::src::set_c::set_query;

pub type Set = _Set;


pub type SetIterator = _SetIterator;


pub type SetEntry = _SetEntry;


pub type SetValue = VoidPtr;


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _SetIterator {
    pub set: Ptr<Set>,
    pub next_entry: Ptr<SetEntry>,
    pub next_chain: u32,
}


pub type SetHashFunc = FuncPtr<fn(SetValue) -> u32>;


pub type SetEqualFunc = FuncPtr<fn(SetValue, SetValue) -> i32>;


pub type SetFreeFunc = FuncPtr<fn(SetValue)>;


macro_rules! ALGORITHM_SET_H { () => { } }
pub(crate) use ALGORITHM_SET_H;


macro_rules! SET_NULL { () => { NULL!() } }
pub(crate) use SET_NULL;


