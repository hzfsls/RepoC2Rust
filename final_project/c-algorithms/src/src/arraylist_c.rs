use crate::translation_utils::*;
pub use crate::src::arraylist_h::*;

pub fn arraylist_new(mut length: u32) -> Ptr<ArrayList> {
    let mut new_arraylist: Ptr<ArrayList> = Default::default();

    if (length <= 0).as_bool() {
        length = 16;
    }

    new_arraylist = c_malloc!(c_sizeof!(ArrayList));

    if (new_arraylist == NULL!()).as_bool() {
        return NULL!();
    }

    new_arraylist._alloced = length;
    new_arraylist.length = 0;

    new_arraylist.data = c_malloc!(length * c_sizeof!(ArrayListValue));

    if (new_arraylist.data == NULL!()).as_bool() {
        c_free!(new_arraylist);
        return NULL!();
    }

    return new_arraylist.cast();
}


pub fn arraylist_free(mut arraylist: Ptr<ArrayList>) {
    if (arraylist != NULL!()).as_bool() {
        c_free!(arraylist.data);
        c_free!(arraylist);
    }
}


pub fn arraylist_enlarge(mut arraylist: Ptr<ArrayList>) -> i32 {
    let mut data: Ptr<ArrayListValue> = Default::default();
    let mut newsize: u32 = Default::default();

    newsize = arraylist._alloced * 2;

    data = c_realloc!(arraylist.data, c_sizeof!(ArrayListValue) * newsize);

    if (data == NULL!()).as_bool() {
        return 0;
    } else {
        arraylist.data = data.cast();
        arraylist._alloced = newsize.cast();

        return 1;
    }
}


pub fn arraylist_insert(mut arraylist: Ptr<ArrayList>, mut index: u32, mut data: ArrayListValue) -> i32 {
    if (index > arraylist.length) {
        return 0;
    }

    if (arraylist.length + 1 > arraylist._alloced) {
        if !arraylist_enlarge(arraylist).as_bool() {
            return 0;
        }
    }

    c_memmove!(
        c_ref!(arraylist.data[index + 1]),
        c_ref!(arraylist.data[index]),
        (arraylist.length - index) * c_sizeof!(ArrayListValue)
    );

    arraylist.data[index] = data;
    arraylist.length.prefix_plus_plus();

    return 1;
}


pub fn arraylist_append(mut arraylist: Ptr<ArrayList>, mut data: ArrayListValue) -> i32 {
    return arraylist_insert(arraylist.cast(), arraylist.length.cast(), data.cast()).cast();
}


pub fn arraylist_prepend(mut arraylist: Ptr<ArrayList>, mut data: ArrayListValue) -> i32 {
    return arraylist_insert(arraylist.cast(), 0, data.cast()).cast();
}


pub fn arraylist_remove_range(mut arraylist: Ptr<ArrayList>, mut index: u32, mut length: u32) {
    if (index > arraylist.length || index + length > arraylist.length) {
        return;
    }
    c_memmove!(
        c_ref!(arraylist.data[index]),
        c_ref!(arraylist.data[index + length]),
        (arraylist.length - (index + length)) * c_sizeof!(ArrayListValue)
    );
    arraylist.length -= length;
}


pub fn arraylist_remove(mut arraylist: Ptr<ArrayList>, mut index: u32) {
    arraylist_remove_range(arraylist.cast(), index.cast(), 1);
}


pub fn arraylist_index_of(mut arraylist: Ptr<ArrayList>, mut callback: ArrayListEqualFunc, mut data: ArrayListValue) -> i32 {
    let mut i: u32 = 0;
    c_for!(i = 0; i < arraylist.length; i.prefix_plus_plus(); {
        if (callback(arraylist.data[i], data) != 0).as_bool() {
            return i.cast::<i32>();
        }
    });
    return -1;
}


pub fn arraylist_clear(mut arraylist: Ptr<ArrayList>) {
    arraylist.length = 0;
}


pub fn arraylist_sort_internal(mut list_data: Ptr<ArrayListValue>, mut list_length: u32, mut compare_func: ArrayListCompareFunc) {
    let mut pivot: ArrayListValue = Default::default();
    let mut tmp: ArrayListValue = Default::default();
    let mut i: u32 = Default::default();
    let mut list1_length: u32 = Default::default();
    let mut list2_length: u32 = Default::default();

    if (list_length <= 1).as_bool() {
        return;
    }

    pivot = list_data[list_length - 1].cast();

    list1_length = 0;

    c_for!(let mut i: u32 = 0; i < list_length - 1; i.prefix_plus_plus(); {
        if (compare_func(list_data[i].cast(), pivot.cast()) < 0).as_bool() {
            tmp = list_data[i].cast();
            list_data[i] = list_data[list1_length].cast();
            list_data[list1_length] = tmp.cast();

            list1_length += 1;
        } else {
        }
    });

    list2_length = list_length - list1_length - 1;

    list_data[list_length - 1] = list_data[list1_length].cast();
    list_data[list1_length] = pivot.cast();

    arraylist_sort_internal(list_data.cast(), list1_length.cast(), compare_func.cast());

    arraylist_sort_internal((list_data.cast::<Ptr<ArrayListValue>>() + list1_length + 1).cast(), list2_length.cast(), compare_func.cast());
}


pub fn arraylist_sort(mut arraylist: Ptr<ArrayList>, mut compare_func: ArrayListCompareFunc) {
    arraylist_sort_internal(arraylist.data.cast(), arraylist.length.cast(), compare_func.cast());
}


