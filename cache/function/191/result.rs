pub fn list_sort_internal(mut list: Ptr<Ptr<ListEntry>>, mut compare_func: ListCompareFunc) -> Ptr<ListEntry> {
    let mut pivot: Ptr<ListEntry> = Default::default();
    let mut rover: Ptr<ListEntry> = Default::default();
    let mut less_list: Ptr<ListEntry> = Default::default();
    let mut more_list: Ptr<ListEntry> = Default::default();
    let mut less_list_end: Ptr<ListEntry> = Default::default();
    let mut more_list_end: Ptr<ListEntry> = Default::default();

    if list == NULL!() || compare_func == NULL!() {
        return NULL!();
    }

    if *list == NULL!() || (*list).next == NULL!() {
        return *list;
    }

    pivot = *list;

    less_list = NULL!();
    more_list = NULL!();
    rover = (*list).next;

    while rover != NULL!() {
        let mut next: Ptr<ListEntry> = rover.next;

        if compare_func(rover.data.cast(), pivot.data.cast()) < 0 {
            rover.prev = NULL!();
            rover.next = less_list;
            if less_list != NULL!() {
                less_list.prev = rover;
            }
            less_list = rover;
        } else {
            rover.prev = NULL!();
            rover.next = more_list;
            if more_list != NULL!() {
                more_list.prev = rover;
            }
            more_list = rover;
        }

        rover = next;
    }

    less_list_end = list_sort_internal(c_ref!(less_list).cast(), compare_func.cast());
    more_list_end = list_sort_internal(c_ref!(more_list).cast(), compare_func.cast());

    *list = less_list;

    if less_list == NULL!() {
        pivot.prev = NULL!();
        *list = pivot;
    } else {
        pivot.prev = less_list_end;
        less_list_end.next = pivot;
    }

    pivot.next = more_list;
    if more_list != NULL!() {
        more_list.prev = pivot;
    }

    if more_list == NULL!() {
        return pivot;
    } else {
        return more_list_end;
    }
}
