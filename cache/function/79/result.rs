pub fn slist_nth_entry(mut list: Ptr<SListEntry>, mut n: u32) -> Ptr<SListEntry> {
    let mut entry: Ptr<SListEntry> = list.cast();
    let mut i: u32 = 0;

    c_for!(; i < n; i.prefix_plus_plus(); {
        if (entry == NULL!()).as_bool() {
            return NULL!();
        }
        entry = entry.next.cast();
    });

    return entry.cast();
}
