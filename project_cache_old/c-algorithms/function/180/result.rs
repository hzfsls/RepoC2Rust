pub fn list_find_data(mut list: Ptr<ListEntry>, mut callback: ListEqualFunc, mut data: ListValue) -> Ptr<ListEntry> {
    let mut rover: Ptr<ListEntry> = list.cast();
    c_for!(; rover != NULL!(); rover = rover.next.cast(); {
        if (callback(rover.data.cast(), data.cast()) != 0).as_bool() {
            return rover.cast();
        }
    });
    return NULL!();
}
