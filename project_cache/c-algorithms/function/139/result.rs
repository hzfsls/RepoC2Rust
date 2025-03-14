pub fn set_to_array(mut set: Ptr<Set>) -> Ptr<SetValue> {
    let mut array: Ptr<SetValue> = Default::default();
    let mut array_counter: i32 = Default::default();
    let mut i: u32 = Default::default();
    let mut rover: Ptr<SetEntry> = Default::default();

    array = c_malloc!(c_sizeof!(SetValue) * set.entries);

    if (array == NULL!()).as_bool() {
        return NULL!();
    }

    array_counter = 0;

    c_for!(i = 0; i < set.table_size; i.prefix_plus_plus(); {
        rover = set.table[i].cast();

        while (rover != NULL!()).as_bool() {
            array[array_counter] = rover.data.cast();
            array_counter.suffix_plus_plus();

            rover = rover.next.cast();
        }
    });

    return array.cast();
}
