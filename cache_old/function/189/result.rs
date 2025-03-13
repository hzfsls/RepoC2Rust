pub fn list_remove_entry(mut list: Ptr<Ptr<ListEntry>>, mut entry: Ptr<ListEntry>) -> i32 {
    if list == NULL!() || *list == NULL!() || entry == NULL!() {
        return 0;
    }

    if entry.prev == NULL!() {
        *list = entry.next.cast();

        if entry.next != NULL!() {
            entry.next.prev = NULL!();
        }
    } else {
        entry.prev.next = entry.next.cast();

        if entry.next != NULL!() {
            entry.next.prev = entry.prev.cast();
        }
    }

    c_free!(entry);

    return 1;
}
