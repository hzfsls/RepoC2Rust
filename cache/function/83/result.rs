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
