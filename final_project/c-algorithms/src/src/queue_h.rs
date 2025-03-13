use crate::translation_utils::*;
pub use crate::src::queue_c::_Queue;
pub use crate::src::queue_c::queue_peek_tail;
pub use crate::src::queue_c::queue_pop_tail;
pub use crate::src::queue_c::queue_push_head;
pub use crate::src::queue_c::queue_push_tail;
pub use crate::src::queue_c::queue_new;
pub use crate::src::queue_c::queue_pop_head;
pub use crate::src::queue_c::queue_is_empty;
pub use crate::src::queue_c::queue_free;
pub use crate::src::queue_c::queue_peek_head;

pub type Queue = _Queue;


pub type QueueValue = VoidPtr;


macro_rules! ALGORITHM_QUEUE_H { () => { } }
pub(crate) use ALGORITHM_QUEUE_H;


macro_rules! QUEUE_NULL { () => { NULL!() } }
pub(crate) use QUEUE_NULL;


