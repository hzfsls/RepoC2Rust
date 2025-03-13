pub fn slist_append(mut list: Ptr<Ptr<SListEntry>>, mut data: SListValue) -> Ptr<SListEntry> {
    let mut rover: Ptr<SListEntry> = Default::default();
    let mut newentry: Ptr<SListEntry> = c_malloc!(c_sizeof!(SListEntry));

    if newentry == NULL!() {
        return NULL!();
    }

    newentry.data = data.cast();
    newentry.next = NULL!();

    if *list == NULL!() {
        *list = newentry.cast();
    } else {
        rover = *list.cast();
        while rover.next != NULL!() {
            rover = rover.next.cast();
        }
        rover.next = newentry.cast();
    }

    return newentry.cast();
}
