pub fn sortedarray_last_index(mut sortedarray: Ptr<SortedArray>, mut data: SortedArrayValue, mut left: u32, mut right: u32) -> u32 {
    let mut index: u32 = right;

    while (left < right).as_bool() {
        index = (left + right) / 2;

        let mut order: i32 = (sortedarray.cmp_func)(data.cast(), sortedarray.data[index].cast()).cast();
        if (order <= 0).as_bool() {
            left = index + 1;
        } else {
            right = index;
        }
    }

    return index.cast();
}
