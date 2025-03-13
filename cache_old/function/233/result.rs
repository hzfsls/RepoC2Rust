pub fn set_insert(mut set: Ptr<Set>, mut data: SetValue) -> i32 {
    let mut newentry: Ptr<SetEntry> = Default::default();
    let mut rover: Ptr<SetEntry> = Default::default();
    let mut index: u32 = Default::default();

    if (set.entries * 3) / set.table_size > 0 {
        if !set_enlarge(set.cast()) {
            return 0;
        }
    }

    index = (set.hash_func)(data.cast()) % set.table_size;

    rover = set.table[index].cast();

    while rover != NULL!() {
        if (set.equal_func)(data.cast(), rover.data.cast()) != 0 {
            return 0;
        }

        rover = rover.next.cast();
    }

    newentry = c_malloc!(c_sizeof!(SetEntry));

    if newentry == NULL!() {
        return 0;
    }

    newentry.data = data.cast();

    newentry.next = set.table[index].cast();
    set.table[index] = newentry.cast();

    set.entries.prefix_plus_plus();

    return 1;
}
