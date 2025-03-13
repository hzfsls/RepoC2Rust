use crate::translation_utils::*;
pub use crate::src::slist_c::slist_iter_has_more;
pub use crate::src::slist_c::slist_append;
pub use crate::src::slist_c::slist_remove_data;
pub use crate::src::slist_c::slist_data;
pub use crate::src::slist_c::slist_iter_next;
pub use crate::src::slist_c::slist_find_data;
pub use crate::src::slist_c::slist_to_array;
pub use crate::src::slist_c::slist_set_data;
pub use crate::src::slist_c::slist_next;
pub use crate::src::slist_c::slist_iterate;
pub use crate::src::slist_c::_SListEntry;
pub use crate::src::slist_c::slist_sort;
pub use crate::src::slist_c::slist_nth_data;
pub use crate::src::slist_c::slist_free;
pub use crate::src::slist_c::slist_iter_remove;
pub use crate::src::slist_c::slist_remove_entry;
pub use crate::src::slist_c::slist_nth_entry;
pub use crate::src::slist_c::slist_length;
pub use crate::src::slist_c::slist_prepend;

pub type SListEntry = _SListEntry;


pub type SListIterator = _SListIterator;


pub type SListValue = VoidPtr;


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _SListIterator {
    pub prev_next: Ptr<Ptr<SListEntry>>,
    pub current: Ptr<SListEntry>,
}


pub type SListCompareFunc = FuncPtr<fn(SListValue, SListValue) -> i32>;


pub type SListEqualFunc = FuncPtr<fn(SListValue, SListValue) -> i32>;


macro_rules! ALGORITHM_SLIST_H { () => { } }
pub(crate) use ALGORITHM_SLIST_H;


macro_rules! SLIST_NULL { () => { NULL!() } }
pub(crate) use SLIST_NULL;


