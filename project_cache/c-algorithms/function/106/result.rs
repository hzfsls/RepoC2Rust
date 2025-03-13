pub fn set_enlarge(mut set: Ptr<Set>) -> i32 {
    let mut rover: Ptr<SetEntry> = Default::default();
    let mut next: Ptr<SetEntry> = Default::default();
    let mut old_table: Ptr<Ptr<SetEntry>> = Default::default();
    let mut old_table_size: u32 = Default::default();
    let mut old_prime_index: u32 = Default::default();
    let mut index: u32 = Default::default();
    let mut i: u32 = Default::default();

    old_table = set.table;
    old_table_size = set.table_size;
    old_prime_index = set.prime_index;

    set.prime_index.prefix_plus_plus();

    if !set_allocate_table(set).as_bool() {
        set.table = old_table;
        set.table_size = old_table_size;
        set.prime_index = old_prime_index;

        return 0;
    }

    c_for!(let mut i: u32 = 0; i < old_table_size; i.prefix_plus_plus(); {
        rover = old_table[i];

        while (rover != NULL!()) {
            next = rover.next;

            index = (set.hash_func)(rover.data) % set.table_size;
            rover.next = set.table[index];
            set.table[index] = rover;

            rover = next;
        }
    });

    c_free!(old_table);

    return 1;
}
