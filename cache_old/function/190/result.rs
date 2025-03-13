pub fn list_remove_data(mut list: Ptr<Ptr<ListEntry>>, mut callback: ListEqualFunc, mut data: ListValue) -> u32 {
    let mut entries_removed: u32 = 0;
    let mut rover: Ptr<ListEntry> = Default::default();
    let mut next: Ptr<ListEntry> = Default::default();

    if list == NULL!() || callback == NULL!() {
        return 0;
    }

    entries_removed = 0;

    rover = *list;

    while rover != NULL!() {
        next = rover.next;

        if callback(rover.data.cast(), data.cast()) {
            if rover.prev == NULL!() {
                *list = rover.next;
            } else {
                rover.prev.next = rover.next;
            }

            if rover.next != NULL!() {
                rover.next.prev = rover.prev;
            }

            c_free!(rover);

            entries_removed.suffix_plus_plus();
        }

        rover = next;
    }

    return entries_removed;
}
