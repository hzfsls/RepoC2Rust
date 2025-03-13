pub fn arraylist_new(mut length: u32) -> Ptr<ArrayList> {
    let mut new_arraylist: Ptr<ArrayList> = Default::default();

    if length <= 0 {
        length = 16;
    }

    new_arraylist = c_malloc!(c_sizeof!(ArrayList));

    if new_arraylist == NULL!() {
        return NULL!();
    }

    new_arraylist._alloced = length;
    new_arraylist.length = 0;

    new_arraylist.data = c_malloc!(length * c_sizeof!(ArrayListValue));

    if new_arraylist.data == NULL!() {
        c_free!(new_arraylist);
        return NULL!();
    }

    return new_arraylist.cast();
}
