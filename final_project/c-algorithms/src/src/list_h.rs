use crate::translation_utils::*;
pub use crate::src::list_c::list_sort;
pub use crate::src::list_c::list_nth_data;
pub use crate::src::list_c::list_prepend;
pub use crate::src::list_c::list_iterate;
pub use crate::src::list_c::list_prev;
pub use crate::src::list_c::list_next;
pub use crate::src::list_c::list_iter_remove;
pub use crate::src::list_c::list_set_data;
pub use crate::src::list_c::list_remove_data;
pub use crate::src::list_c::_ListEntry;
pub use crate::src::list_c::list_length;
pub use crate::src::list_c::list_iter_next;
pub use crate::src::list_c::list_data;
pub use crate::src::list_c::list_find_data;
pub use crate::src::list_c::list_to_array;
pub use crate::src::list_c::list_free;
pub use crate::src::list_c::list_iter_has_more;
pub use crate::src::list_c::list_remove_entry;
pub use crate::src::list_c::list_nth_entry;
pub use crate::src::list_c::list_append;

pub type ListEntry = _ListEntry;


pub type ListIterator = _ListIterator;


pub type ListValue = VoidPtr;


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _ListIterator {
    pub prev_next: Ptr<Ptr<ListEntry>>,
    pub current: Ptr<ListEntry>,
}


pub type ListCompareFunc = FuncPtr<fn(ListValue, ListValue) -> i32>;


pub type ListEqualFunc = FuncPtr<fn(ListValue, ListValue) -> i32>;


macro_rules! ALGORITHM_LIST_H { () => { } }
pub(crate) use ALGORITHM_LIST_H;


macro_rules! LIST_NULL { () => { NULL!() } }
pub(crate) use LIST_NULL;


