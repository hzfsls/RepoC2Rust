use crate::translation_utils::*;
pub use crate::src::list_h::*;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _ListEntry {
    pub data: ListValue,
    pub prev: Ptr<ListEntry>,
    pub next: Ptr<ListEntry>,
}


pub fn list_free(mut list: Ptr<ListEntry>) {
    let mut entry: Ptr<ListEntry> = list.cast();

    while (entry != NULL!()).as_bool() {
        let mut next: Ptr<ListEntry> = entry.next.cast();

        c_free!(entry);

        entry = next.cast();
    }
}


pub fn list_prepend(mut list: Ptr<Ptr<ListEntry>>, mut data: ListValue) -> Ptr<ListEntry> {
    let mut newentry: Ptr<ListEntry> = Default::default();

    if (list == NULL!()) {
        return NULL!();
    }

    newentry = c_malloc!(c_sizeof!(ListEntry));

    if (newentry == NULL!()) {
        return NULL!();
    }

    newentry.data = data;

    if (*list != NULL!()) {
        (*list).prev = newentry;
    }
    newentry.prev = NULL!();
    newentry.next = *list;
    *list = newentry;

    return newentry;
}


pub fn list_append(mut list: Ptr<Ptr<ListEntry>>, mut data: ListValue) -> Ptr<ListEntry> {
    let mut rover: Ptr<ListEntry> = Default::default();
    let mut newentry: Ptr<ListEntry> = Default::default();

    if (list == NULL!()) {
        return NULL!();
    }

    newentry = c_malloc!(c_sizeof!(ListEntry));

    if (newentry == NULL!()) {
        return NULL!();
    }

    newentry.data = data;
    newentry.next = NULL!();

    if (*list == NULL!()) {
        *list = newentry;
        newentry.prev = NULL!();
    } else {
        rover = *list;
        while (rover.next != NULL!()) {
            rover = rover.next;
        }

        newentry.prev = rover;
        rover.next = newentry;
    }

    return newentry;
}


pub fn list_data(mut listentry: Ptr<ListEntry>) -> ListValue {
    if (listentry == NULL!()).as_bool() {
        return LIST_NULL!();
    }
    return listentry.data.cast();
}


pub fn list_set_data(mut listentry: Ptr<ListEntry>, mut value: ListValue) {
    if (listentry != NULL!()).as_bool() {
        listentry.data = value.cast();
    }
}


pub fn list_prev(mut listentry: Ptr<ListEntry>) -> Ptr<ListEntry> {
    if (listentry == NULL!()).as_bool() {
        return NULL!();
    }
    return listentry.prev.cast();
}


pub fn list_next(mut listentry: Ptr<ListEntry>) -> Ptr<ListEntry> {
    if (listentry == NULL!()).as_bool() {
        return NULL!();
    }
    return listentry.next.cast();
}


pub fn list_nth_entry(mut list: Ptr<ListEntry>, mut n: u32) -> Ptr<ListEntry> {
    let mut entry: Ptr<ListEntry> = list.cast();
    let mut i: u32 = 0;
    c_for!(; i < n; i.prefix_plus_plus(); {
        if (entry == NULL!()).as_bool() {
            return NULL!();
        }
        entry = entry.next.cast();
    });
    return entry.cast();
}


pub fn list_nth_data(mut list: Ptr<ListEntry>, mut n: u32) -> ListValue {
    let mut entry: Ptr<ListEntry> = Default::default();

    entry = list_nth_entry(list.cast(), n.cast());

    if (entry == NULL!()).as_bool() {
        return LIST_NULL!();
    } else {
        return entry.data.cast();
    }
}


pub fn list_length(mut list: Ptr<ListEntry>) -> u32 {
    let mut entry: Ptr<ListEntry> = Default::default();
    let mut length: u32 = 0;
    length = 0;
    entry = list.cast();
    while (entry != NULL!()).as_bool() {
        length.prefix_plus_plus();
        entry = entry.next.cast();
    }
    return length.cast();
}


pub fn list_to_array(mut list: Ptr<ListEntry>) -> Ptr<ListValue> {
    let mut rover: Ptr<ListEntry> = Default::default();
    let mut array: Ptr<ListValue> = Default::default();
    let mut length: u32 = Default::default();
    let mut i: u32 = Default::default();

    length = list_length(list.cast()).cast();

    array = c_malloc!(c_sizeof!(ListValue) * length);

    if (array == NULL!()).as_bool() {
        return NULL!();
    }

    rover = list.cast();

    c_for!(let mut i: u32 = 0; i < length; i.prefix_plus_plus(); {
        array[i] = rover.data.cast();
        rover = rover.next.cast();
    });

    return array.cast();
}


pub fn list_remove_entry(mut list: Ptr<Ptr<ListEntry>>, mut entry: Ptr<ListEntry>) -> i32 {
    if (list == NULL!() || *list == NULL!() || entry == NULL!()).as_bool() {
        return 0;
    }

    if (entry.prev == NULL!()).as_bool() {
        *list = entry.next.cast();

        if (entry.next != NULL!()).as_bool() {
            entry.next.prev = NULL!();
        }
    } else {
        entry.prev.next = entry.next.cast();

        if (entry.next != NULL!()).as_bool() {
            entry.next.prev = entry.prev.cast();
        }
    }

    c_free!(entry);

    return 1;
}


pub fn list_remove_data(mut list: Ptr<Ptr<ListEntry>>, mut callback: ListEqualFunc, mut data: ListValue) -> u32 {
    let mut entries_removed: u32 = 0;
    let mut rover: Ptr<ListEntry> = Default::default();
    let mut next: Ptr<ListEntry> = Default::default();

    if (list == NULL!() || callback == NULL!()).as_bool() {
        return 0;
    }

    entries_removed = 0;

    rover = *list;

    while (rover != NULL!()).as_bool() {

        next = rover.next;

        if callback(rover.data.cast(), data.cast()).as_bool() {

            if (rover.prev == NULL!()).as_bool() {

                *list = rover.next;
            } else {

                rover.prev.next = rover.next;
            }

            if (rover.next != NULL!()).as_bool() {
                rover.next.prev = rover.prev;
            }

            c_free!(rover);

            entries_removed.prefix_plus_plus();
        }

        rover = next;
    }

    return entries_removed.cast();
}


pub fn list_sort_internal(mut list: Ptr<Ptr<ListEntry>>, mut compare_func: ListCompareFunc) -> Ptr<ListEntry> {
    let mut pivot: Ptr<ListEntry> = Default::default();
    let mut rover: Ptr<ListEntry> = Default::default();
    let mut less_list: Ptr<ListEntry> = Default::default();
    let mut more_list: Ptr<ListEntry> = Default::default();
    let mut less_list_end: Ptr<ListEntry> = Default::default();
    let mut more_list_end: Ptr<ListEntry> = Default::default();

    if (list == NULL!() || compare_func == NULL!()).as_bool() {
        return NULL!();
    }

    if (*list == NULL!() || (*list).next == NULL!()).as_bool() {
        return *list;
    }

    pivot = *list;

    less_list = NULL!();
    more_list = NULL!();
    rover = (*list).next.cast();

    while (rover != NULL!()).as_bool() {
        let mut next: Ptr<ListEntry> = rover.next.cast();

        if (compare_func(rover.data.cast(), pivot.data.cast()) < 0).as_bool() {
            rover.prev = NULL!();
            rover.next = less_list.cast();
            if (less_list != NULL!()).as_bool() {
                less_list.prev = rover.cast();
            }
            less_list = rover.cast();
        } else {
            rover.prev = NULL!();
            rover.next = more_list.cast();
            if (more_list != NULL!()).as_bool() {
                more_list.prev = rover.cast();
            }
            more_list = rover.cast();
        }

        rover = next.cast();
    }

    less_list_end = list_sort_internal(c_ref!(less_list).cast(), compare_func.cast());
    more_list_end = list_sort_internal(c_ref!(more_list).cast(), compare_func.cast());

    *list = less_list.cast();

    if (less_list == NULL!()).as_bool() {
        pivot.prev = NULL!();
        *list = pivot.cast();
    } else {
        pivot.prev = less_list_end.cast();
        less_list_end.next = pivot.cast();
    }

    pivot.next = more_list.cast();
    if (more_list != NULL!()).as_bool() {
        more_list.prev = pivot.cast();
    }

    if (more_list == NULL!()).as_bool() {
        return pivot.cast();
    } else {
        return more_list_end.cast();
    }
}


pub fn list_sort(mut list: Ptr<Ptr<ListEntry>>, mut compare_func: ListCompareFunc) {
    list_sort_internal(list.cast(), compare_func.cast());
}


pub fn list_find_data(mut list: Ptr<ListEntry>, mut callback: ListEqualFunc, mut data: ListValue) -> Ptr<ListEntry> {
    let mut rover: Ptr<ListEntry> = list.cast();
    c_for!(; rover != NULL!(); rover = rover.next.cast(); {
        if (callback(rover.data.cast(), data.cast()) != 0).as_bool() {
            return rover.cast();
        }
    });
    return NULL!();
}


pub fn list_iterate(mut list: Ptr<Ptr<ListEntry>>, mut iter: Ptr<ListIterator>) {
    iter.prev_next = list.cast();
    iter.current = NULL!();
}


pub fn list_iter_has_more(mut iter: Ptr<ListIterator>) -> i32 {
    if (iter.current == NULL!() || iter.current != *iter.prev_next).as_bool() {
        return (*iter.prev_next != NULL!()).cast();
    } else {
        return (iter.current.next != NULL!()).cast();
    }
}


pub fn list_iter_next(mut iter: Ptr<ListIterator>) -> ListValue {
    if (iter.current == NULL!() || iter.current != *iter.prev_next) {
        iter.current = *iter.prev_next;
    } else {
        iter.prev_next = c_ref!(iter.current.next);
        iter.current = iter.current.next;
    }

    if (iter.current == NULL!()) {
        return LIST_NULL!();
    } else {
        return iter.current.data;
    }
}


pub fn list_iter_remove(mut iter: Ptr<ListIterator>) {
    if (iter.current == NULL!() || iter.current != *iter.prev_next).as_bool() {
    } else {
        *iter.prev_next = iter.current.next.cast();
        if (iter.current.next != NULL!()).as_bool() {
            iter.current.next.prev = iter.current.prev.cast();
        }
        c_free!(iter.current);
        iter.current = NULL!();
    }
}


