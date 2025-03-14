pub fn slist_find_data(mut list: Ptr<SListEntry>, mut callback: SListEqualFunc, mut data: SListValue) -> Ptr<SListEntry> {
    let mut rover: Ptr<SListEntry> = list.cast();
    c_for!(; rover != NULL!(); rover = rover.next.cast(); {
        if (callback(rover.data.cast(), data.cast()) != 0).as_bool() {
            return rover.cast();
        }
    });
    return NULL!();
}
