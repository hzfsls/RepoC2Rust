use crate::translation_utils::*;
pub use crate::src::slist_h::*;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _SListEntry {
    pub data: SListValue,
    pub next: Ptr<SListEntry>,
}


pub fn slist_free(mut list: Ptr<SListEntry>) {
    let mut entry: Ptr<SListEntry> = list.cast();

    while (entry != NULL!()).as_bool() {
        let mut next: Ptr<SListEntry> = entry.next.cast();

        c_free!(entry);

        entry = next.cast();
    }
}


pub fn slist_prepend(mut list: Ptr<Ptr<SListEntry>>, mut data: SListValue) -> Ptr<SListEntry> {
    let mut newentry: Ptr<SListEntry> = c_malloc!(c_sizeof!(SListEntry));

    if (newentry == NULL!()) {
        return NULL!();
    }

    newentry.data = data;

    newentry.next = *list;
    *list = newentry;

    return newentry;
}


pub fn slist_append(mut list: Ptr<Ptr<SListEntry>>, mut data: SListValue) -> Ptr<SListEntry> {
    let mut rover: Ptr<SListEntry> = Default::default();
    let mut newentry: Ptr<SListEntry> = Default::default();

    newentry = c_malloc!(c_sizeof!(SListEntry));

    if (newentry == NULL!()) {
        return NULL!();
    }

    newentry.data = data;
    newentry.next = NULL!();

    if (*list == NULL!()) {
        *list = newentry;
    } else {
        rover = *list;
        while (rover.next != NULL!()) {
            rover = rover.next;
        }
        rover.next = newentry;
    }

    return newentry;
}


pub fn slist_data(mut listentry: Ptr<SListEntry>) -> SListValue {
    return listentry.data.cast();
}


pub fn slist_set_data(mut listentry: Ptr<SListEntry>, mut data: SListValue) {
    if (listentry != NULL!()).as_bool() {
        listentry.data = data.cast();
    }
}


pub fn slist_next(mut listentry: Ptr<SListEntry>) -> Ptr<SListEntry> {
    return listentry.next.cast();
}


pub fn slist_nth_entry(mut list: Ptr<SListEntry>, mut n: u32) -> Ptr<SListEntry> {
    let mut entry: Ptr<SListEntry> = list.cast();
    let mut i: u32 = 0;

    c_for!(; i < n; i.prefix_plus_plus(); {
        if (entry == NULL!()).as_bool() {
            return NULL!();
        }
        entry = entry.next.cast();
    });

    return entry.cast();
}


pub fn slist_nth_data(mut list: Ptr<SListEntry>, mut n: u32) -> SListValue {
    let mut entry: Ptr<SListEntry> = Default::default();

    entry = slist_nth_entry(list.cast(), n.cast());

    if (entry == NULL!()).as_bool() {
        return SLIST_NULL!();
    } else {
        return entry.data.cast();
    }
}


pub fn slist_length(mut list: Ptr<SListEntry>) -> u32 {
    let mut entry: Ptr<SListEntry> = Default::default();
    let mut length: u32 = 0;
    entry = list.cast();
    while (entry != NULL!()).as_bool() {
        length.prefix_plus_plus();
        entry = entry.next.cast();
    }
    return length.cast();
}


pub fn slist_to_array(mut list: Ptr<SListEntry>) -> Ptr<SListValue> {
    let mut rover: Ptr<SListEntry> = Default::default();
    let mut array: Ptr<SListValue> = Default::default();
    let mut length: u32 = Default::default();
    let mut i: u32 = Default::default();

    length = slist_length(list.cast()).cast();

    array = c_malloc!(c_sizeof!(SListValue) * length);

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


pub fn slist_remove_entry(mut list: Ptr<Ptr<SListEntry>>, mut entry: Ptr<SListEntry>) -> i32 {
    let mut rover: Ptr<SListEntry> = Default::default();

    if (*list == NULL!() || entry == NULL!()) {
        return 0;
    }

    if (*list == entry) {
        *list = entry.next;
    } else {
        rover = *list;

        while (rover != NULL!() && rover.next != entry) {
            rover = rover.next;
        }

        if (rover == NULL!()) {
            return 0;
        } else {
            rover.next = entry.next;
        }
    }

    c_free!(entry);

    return 1;
}


pub fn slist_remove_data(mut list: Ptr<Ptr<SListEntry>>, mut callback: SListEqualFunc, mut data: SListValue) -> u32 {
    let mut rover: Ptr<Ptr<SListEntry>> = Default::default();
    let mut next: Ptr<SListEntry> = Default::default();
    let mut entries_removed: u32 = 0;

    entries_removed = 0;

    rover = list.cast();

    while (*rover != NULL!()).as_bool() {

        if (callback((*rover).data.cast(), data.cast()) != 0).as_bool() {

            next = (*rover).next.cast();
            c_free!(*rover);
            *rover = next.cast();

            entries_removed.prefix_plus_plus();
        } else {

            rover = c_ref!((*rover).next).cast();
        }
    }

    return entries_removed.cast();
}


pub fn slist_sort_internal(mut list: Ptr<Ptr<SListEntry>>, mut compare_func: SListCompareFunc) -> Ptr<SListEntry> {
    let mut pivot: Ptr<SListEntry> = Default::default();
    let mut rover: Ptr<SListEntry> = Default::default();
    let mut less_list: Ptr<SListEntry> = Default::default();
    let mut more_list: Ptr<SListEntry> = Default::default();
    let mut less_list_end: Ptr<SListEntry> = Default::default();
    let mut more_list_end: Ptr<SListEntry> = Default::default();

    if (*list == NULL!()).as_bool() || ((*list).next == NULL!()).as_bool() {
        return *list;
    }

    pivot = *list;

    less_list = NULL!();
    more_list = NULL!();
    rover = (*list).next;

    while (rover != NULL!()).as_bool() {
        let mut next: Ptr<SListEntry> = rover.next;

        if (compare_func(rover.data.cast(), pivot.data.cast()) < 0).as_bool() {
            rover.next = less_list;
            less_list = rover;
        } else {
            rover.next = more_list;
            more_list = rover;
        }

        rover = next;
    }

    less_list_end = slist_sort_internal(c_ref!(less_list).cast(), compare_func);
    more_list_end = slist_sort_internal(c_ref!(more_list).cast(), compare_func);

    *list = less_list;

    if (less_list == NULL!()).as_bool() {
        *list = pivot;
    } else {
        less_list_end.next = pivot;
    }

    pivot.next = more_list;

    if (more_list == NULL!()).as_bool() {
        return pivot;
    } else {
        return more_list_end;
    }
}


pub fn slist_sort(mut list: Ptr<Ptr<SListEntry>>, mut compare_func: SListCompareFunc) {
    slist_sort_internal(list.cast(), compare_func.cast());
}


pub fn slist_find_data(mut list: Ptr<SListEntry>, mut callback: SListEqualFunc, mut data: SListValue) -> Ptr<SListEntry> {
    let mut rover: Ptr<SListEntry> = list.cast();
    while (rover != NULL!()).as_bool() {
        if (callback(rover.data.cast(), data.cast()) != 0).as_bool() {
            return rover.cast();
        }
        rover = rover.next.cast();
    }
    return NULL!();
}


pub fn slist_iterate(mut list: Ptr<Ptr<SListEntry>>, mut iter: Ptr<SListIterator>) {
    iter.prev_next = list.cast();
    iter.current = NULL!();
}


pub fn slist_iter_has_more(mut iter: Ptr<SListIterator>) -> i32 {
    if (iter.current == NULL!() || iter.current != *iter.prev_next).as_bool() {
        return (*iter.prev_next != NULL!()).cast();
    } else {
        return (iter.current.next != NULL!()).cast();
    }
}


pub fn slist_iter_next(mut iter: Ptr<SListIterator>) -> SListValue {
    if (iter.current == NULL!() || iter.current != *iter.prev_next) {
        iter.current = *iter.prev_next;
    } else {
        iter.prev_next = c_ref!(iter.current.next);
        iter.current = iter.current.next;
    }

    if (iter.current == NULL!()) {
        return SLIST_NULL!();
    } else {
        return iter.current.data;
    }
}


pub fn slist_iter_remove(mut iter: Ptr<SListIterator>) {
    if (iter.current == NULL!() || iter.current != *iter.prev_next).as_bool() {
    } else {
        *iter.prev_next = iter.current.next.cast();
        c_free!(iter.current);
        iter.current = NULL!();
    }
}


