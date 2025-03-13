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
