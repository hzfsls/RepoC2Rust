pub fn list_prepend(mut list: Ptr<Ptr<ListEntry>>, mut data: ListValue) -> Ptr<ListEntry> {
    let mut newentry: Ptr<ListEntry> = Default::default();

    if list == NULL!() {
        return NULL!();
    }

    newentry = c_malloc!(c_sizeof!(ListEntry));

    if newentry == NULL!() {
        return NULL!();
    }

    newentry.data = data.cast();

    if *list != NULL!() {
        (*list).prev = newentry.cast();
    }
    newentry.prev = NULL!();
    newentry.next = *list.cast();
    *list = newentry.cast();

    return newentry.cast();
}
