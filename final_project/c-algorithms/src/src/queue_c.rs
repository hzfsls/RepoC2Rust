use crate::translation_utils::*;
pub use crate::src::queue_h::*;

pub type QueueEntry = _QueueEntry;


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _QueueEntry {
    pub data: QueueValue,
    pub prev: Ptr<QueueEntry>,
    pub next: Ptr<QueueEntry>,
}


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _Queue {
    pub head: Ptr<QueueEntry>,
    pub tail: Ptr<QueueEntry>,
}


pub fn queue_new() -> Ptr<Queue> {
    let mut queue: Ptr<Queue> = c_malloc!(c_sizeof!(Queue));

    if (queue == NULL!()).as_bool() {
        return NULL!();
    }

    queue.head = NULL!();
    queue.tail = NULL!();

    return queue.cast();
}


pub fn queue_free(mut queue: Ptr<Queue>) {
    while (!queue_is_empty(queue.cast())).as_bool() {
        queue_pop_head(queue.cast());
    }
    c_free!(queue);
}


pub fn queue_push_head(mut queue: Ptr<Queue>, mut data: QueueValue) -> i32 {
    let mut new_entry: Ptr<QueueEntry> = c_malloc!(c_sizeof!(QueueEntry));

    if (new_entry == NULL!()).as_bool() {
        return 0;
    }

    new_entry.data = data.cast();
    new_entry.prev = NULL!();
    new_entry.next = queue.head.cast();

    if (queue.head == NULL!()).as_bool() {
        queue.head = new_entry.cast();
        queue.tail = new_entry.cast();
    } else {
        queue.head.prev = new_entry.cast();
        queue.head = new_entry.cast();
    }

    return 1;
}


pub fn queue_pop_head(mut queue: Ptr<Queue>) -> QueueValue {
    let mut entry: Ptr<QueueEntry> = Default::default();
    let mut result: QueueValue = Default::default();

    if queue_is_empty(queue.cast()).as_bool() {
        return QUEUE_NULL!();
    }

    entry = queue.head.cast();
    queue.head = entry.next.cast();
    result = entry.data.cast();

    if (queue.head == NULL!()).as_bool() {
        queue.tail = NULL!();
    } else {
        queue.head.prev = NULL!();
    }

    c_free!(entry);

    return result.cast();
}


pub fn queue_peek_head(mut queue: Ptr<Queue>) -> QueueValue {
    if queue_is_empty(queue.cast()).as_bool() {
        return QUEUE_NULL!();
    } else {
        return queue.head.data.cast();
    }
}


pub fn queue_push_tail(mut queue: Ptr<Queue>, mut data: QueueValue) -> i32 {
    let mut new_entry: Ptr<QueueEntry> = c_malloc!(c_sizeof!(QueueEntry));

    if (new_entry == NULL!()).as_bool() {
        return 0;
    }

    new_entry.data = data.cast();
    new_entry.prev = queue.tail.cast();
    new_entry.next = NULL!();

    if (queue.tail == NULL!()).as_bool() {
        queue.head = new_entry.cast();
        queue.tail = new_entry.cast();
    } else {
        queue.tail.next = new_entry.cast();
        queue.tail = new_entry.cast();
    }

    return 1;
}


pub fn queue_pop_tail(mut queue: Ptr<Queue>) -> QueueValue {
    let mut entry: Ptr<QueueEntry> = Default::default();
    let mut result: QueueValue = Default::default();

    if queue_is_empty(queue.cast()).as_bool() {
        return QUEUE_NULL!();
    }

    entry = queue.tail.cast();
    queue.tail = entry.prev.cast();
    result = entry.data.cast();

    if (queue.tail == NULL!()).as_bool() {
        queue.head = NULL!();
    } else {
        queue.tail.next = NULL!();
    }

    c_free!(entry);

    return result.cast();
}


pub fn queue_peek_tail(mut queue: Ptr<Queue>) -> QueueValue {
    if queue_is_empty(queue.cast()).as_bool() {
        return QUEUE_NULL!();
    } else {
        return queue.tail.data.cast();
    }
}


pub fn queue_is_empty(mut queue: Ptr<Queue>) -> i32 {
    return (queue.head == NULL!()).cast();
}


