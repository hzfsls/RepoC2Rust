pub fn slist_nth_data(mut list: Ptr<SListEntry>, mut n: u32) -> SListValue {
    let mut entry: Ptr<SListEntry> = Default::default();

    entry = slist_nth_entry(list.cast(), n.cast());

    if entry == NULL!() {
        return SLIST_NULL!();
    } else {
        return entry.data.cast();
    }
}
