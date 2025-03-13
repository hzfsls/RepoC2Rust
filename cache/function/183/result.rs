pub fn list_prev(mut listentry: Ptr<ListEntry>) -> Ptr<ListEntry> {
    if listentry == NULL!() {
        return NULL!();
    }
    return listentry.prev.cast();
}
