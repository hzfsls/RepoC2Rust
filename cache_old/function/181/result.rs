pub fn list_data(mut listentry: Ptr<ListEntry>) -> ListValue {
    if listentry == NULL!() {
        return LIST_NULL!();
    }
    return listentry.data.cast();
}
