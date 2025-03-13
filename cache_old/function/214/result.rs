pub fn slist_iter_has_more(mut iter: Ptr<SListIterator>) -> i32 {
    if iter.current == NULL!() || iter.current != *iter.prev_next {
        return (*iter.prev_next != NULL!()).cast::<i32>();
    } else {
        return (iter.current.next != NULL!()).cast::<i32>();
    }
}
