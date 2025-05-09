pub fn slist_nth_data(mut list: Ptr<SListEntry>, mut n: u32) -> SListValue {
    let mut entry: Ptr<SListEntry>;

    entry = slist_nth_entry(list.cast(), n.cast()).cast();

    if (entry == NULL!()).as_bool() {
        return SLIST_NULL!();
    } else {
        return entry.data.cast();
    }
}
