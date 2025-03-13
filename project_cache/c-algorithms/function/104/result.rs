pub fn set_free(mut set: Ptr<Set>) {
    let mut rover: Ptr<SetEntry> = Default::default();
    let mut next: Ptr<SetEntry> = Default::default();
    let mut i: u32 = Default::default();

    c_for!(i = 0; i < set.table_size; i.prefix_plus_plus(); {
        rover = set.table[i];

        while (rover != NULL!()) {
            next = rover.next;

            set_free_entry(set, rover);

            rover = next;
        }
    });

    c_free!(set.table);

    c_free!(set);
}
