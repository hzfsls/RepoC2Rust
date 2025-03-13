use crate::translation_utils::*;
pub use crate::src::sortedarray_h::*;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _SortedArray {
    pub data: Ptr<SortedArrayValue>,
    pub length: u32,
    pub _alloced: u32,
    pub equ_func: SortedArrayEqualFunc,
    pub cmp_func: SortedArrayCompareFunc,
}


pub fn sortedarray_first_index(mut sortedarray: Ptr<SortedArray>, mut data: SortedArrayValue, mut left: u32, mut right: u32) -> u32 {
    let mut index: u32 = left.cast();

    while (left < right).as_bool() {
        index = ((left + right) / 2).cast();

        let mut order: i32 = (sortedarray.cmp_func)(data.cast(), sortedarray.data[index].cast()).cast();
        if order > 0 {
            left = (index + 1).cast();
        } else {
            right = index.cast();
        }
    }

    return index.cast();
}


pub fn sortedarray_last_index(mut sortedarray: Ptr<SortedArray>, mut data: SortedArrayValue, mut left: u32, mut right: u32) -> u32 {
    let mut index: u32 = right.cast();

    while (left < right).as_bool() {
        index = ((left + right) / 2).cast();

        let mut order: i32 = (sortedarray.cmp_func)(data.cast(), sortedarray.data[index].cast()).cast();
        if (order <= 0).as_bool() {
            left = (index + 1).cast();
        } else {
            right = index.cast();
        }
    }

    return index.cast();
}


pub fn sortedarray_get(mut array: Ptr<SortedArray>, mut i: u32) -> Ptr<SortedArrayValue> {
    if (array == NULL!()).as_bool() {
        return NULL!();
    }
    return array.data[i].cast();
}


pub fn sortedarray_length(mut array: Ptr<SortedArray>) -> u32 {
    return array.length.cast();
}


pub fn sortedarray_new(mut length: u32, mut equ_func: SortedArrayEqualFunc, mut cmp_func: SortedArrayCompareFunc) -> Ptr<SortedArray> {
    if (equ_func == NULL!() || cmp_func == NULL!()).as_bool() {
        return NULL!();
    }

    if (length == 0).as_bool() {
        length = 16;
    }

    let mut array: Ptr<SortedArrayValue> = c_malloc!(c_sizeof!(SortedArrayValue) * length);

    if (array == NULL!()).as_bool() {
        return NULL!();
    }

    let mut sortedarray: Ptr<SortedArray> = c_malloc!(c_sizeof!(SortedArray));

    if (sortedarray == NULL!()).as_bool() {
        c_free!(array);
        return NULL!();
    }

    sortedarray.data = array.cast();
    sortedarray.length = 0;
    sortedarray._alloced = length;
    sortedarray.equ_func = equ_func.cast();
    sortedarray.cmp_func = cmp_func.cast();
    return sortedarray.cast();
}


pub fn sortedarray_free(mut sortedarray: Ptr<SortedArray>) {
    if (sortedarray != NULL!()).as_bool() {
        c_free!(sortedarray.data);
        c_free!(sortedarray);
    }
}


pub fn sortedarray_remove(mut sortedarray: Ptr<SortedArray>, mut index: u32) {
    sortedarray_remove_range(sortedarray.cast(), index.cast(), 1);
}


pub fn sortedarray_remove_range(mut sortedarray: Ptr<SortedArray>, mut index: u32, mut length: u32) {
    if (index > sortedarray.length || index + length > sortedarray.length) {
        return;
    }

    c_memmove!(
        c_ref!(sortedarray.data[index]),
        c_ref!(sortedarray.data[index + length]),
        (sortedarray.length - (index + length)) * c_sizeof!(SortedArrayValue)
    );

    sortedarray.length -= length;
}


pub fn sortedarray_insert(mut sortedarray: Ptr<SortedArray>, mut data: SortedArrayValue) -> i32 {
    let mut left: u32 = 0;
    let mut right: u32 = sortedarray.length;
    let mut index: u32 = 0;

    right = if right > 1 { right } else { 0 };

    while (left != right) {
        index = (left + right) / 2;

        let mut order: i32 = (sortedarray.cmp_func)(data, sortedarray.data[index]);
        if order < 0 {
            right = index;
        } else if order > 0 {
            left = index + 1;
        } else {
            break;
        }
    }

    if (sortedarray.length > 0) && (sortedarray.cmp_func)(data, sortedarray.data[index]) > 0 {
        index += 1;
    }

    if (sortedarray.length + 1 > sortedarray._alloced) {
        let mut newsize: u32;
        let mut data: Ptr<SortedArrayValue>;

        newsize = sortedarray._alloced * 2;
        data = c_realloc!(sortedarray.data, c_sizeof!(SortedArrayValue) * newsize);

        if (data == NULL!()) {
            return 0;
        } else {
            sortedarray.data = data;
            sortedarray._alloced = newsize;
        }
    }

    c_memmove!(
        c_ref!(sortedarray.data[index + 1]),
        c_ref!(sortedarray.data[index]),
        (sortedarray.length - index) * c_sizeof!(SortedArrayValue)
    );

    sortedarray.data[index] = data;
    sortedarray.length += 1;

    return 1;
}


pub fn sortedarray_index_of(mut sortedarray: Ptr<SortedArray>, mut data: SortedArrayValue) -> i32 {
    if (sortedarray == NULL!()).as_bool() {
        return -1;
    }

    let mut left: u32 = 0;
    let mut right: u32 = sortedarray.length.cast();
    let mut index: u32 = 0;

    right = if right > 1 { right } else { 0 };

    while (left != right).as_bool() {
        index = (left + right) / 2;

        let mut order: i32 = (sortedarray.cmp_func)(data.cast(), sortedarray.data[index].cast()).cast();
        if order < 0 {
            right = index;
        } else if order > 0 {
            left = index + 1;
        } else {
            left = sortedarray_first_index(sortedarray.cast(), data.cast(), left.cast(), index.cast()).cast();
            right = sortedarray_last_index(sortedarray.cast(), data.cast(), index.cast(), right.cast()).cast();

            c_for!(let mut index: u32 = left; index <= right; index.suffix_plus_plus(); {
                if (sortedarray.equ_func)(data.cast(), sortedarray.data[index].cast()).as_bool() {
                    return index.cast::<i32>();
                }
            });

            return -1;
        }
    }

    return -1;
}


pub fn sortedarray_clear(mut sortedarray: Ptr<SortedArray>) {
    sortedarray.length = 0;
}


