pub fn set_remove(mut set: Ptr<Set>, mut data: SetValue) -> i32 {
    let mut rover: Ptr<Ptr<SetEntry>> = Default::default();
    let mut entry: Ptr<SetEntry> = Default::default();
    let mut index: u32 = Default::default();

    index = (set.hash_func(data) % set.table_size).cast();

    rover = c_ref!(set.table[index]).cast();

    while (*rover != NULL!()).as_bool() {
        if (set.equal_func(data, (*rover).data) != 0).as_bool() {

            entry = *rover;

            *rover = entry.next;

            set.entries.prefix_minus_minus();

            set_free_entry(set, entry);

            return 1;
        }

        rover = c_ref!((*rover).next).cast();
    }

    return 0;
}
