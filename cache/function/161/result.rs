pub fn arraylist_insert(mut arraylist: Ptr<ArrayList>, mut index: u32, mut data: ArrayListValue) -> i32 {
    if index > arraylist.length {
        return 0;
    }

    if arraylist.length + 1 > arraylist._alloced {
        if !arraylist_enlarge(arraylist.cast()) {
            return 0;
        }
    }

    c_memmove_s!(
        c_ref!(arraylist.data[index + 1]).cast(),
        (arraylist.length - index) * c_sizeof!(ArrayListValue),
        c_ref!(arraylist.data[index]).cast(),
        (arraylist.length - index) * c_sizeof!(ArrayListValue)
    );

    arraylist.data[index] = data.cast();
    arraylist.length.prefix_plus_plus();

    return 1;
}
