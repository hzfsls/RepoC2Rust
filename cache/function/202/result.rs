pub fn slist_set_data(mut listentry: Ptr<SListEntry>, mut data: SListValue) {
    if listentry != NULL!() {
        listentry.data = data.cast();
    }
}
