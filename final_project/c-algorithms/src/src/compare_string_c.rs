use crate::translation_utils::*;
pub use crate::src::compare_string_h::*;

pub fn string_equal(mut string1: Ptr<Void>, mut string2: Ptr<Void>) -> i32 {
    return (c_strcmp!(string1.cast::<Ptr<u8>>(), string2.cast::<Ptr<u8>>()) == 0).cast();
}


pub fn string_compare(mut string1: Ptr<Void>, mut string2: Ptr<Void>) -> i32 {
    let mut result: i32 = Default::default();

    result = c_strcmp!(string1.cast::<Ptr<u8>>(), string2.cast::<Ptr<u8>>());

    if (result < 0).as_bool() {
        return -1;
    } else if (result > 0).as_bool() {
        return 1;
    } else {
        return 0;
    }
}


pub fn string_nocase_equal(mut string1: Ptr<Void>, mut string2: Ptr<Void>) -> i32 {
    return (string_nocase_compare(string1.cast::<Ptr<u8>>(), string2.cast::<Ptr<u8>>()) == 0).cast();
}


pub fn string_nocase_compare(mut string1: Ptr<Void>, mut string2: Ptr<Void>) -> i32 {
    let mut p1: Ptr<u8> = string1.cast::<Ptr<u8>>();
    let mut p2: Ptr<u8> = string2.cast::<Ptr<u8>>();
    let mut c1: i32 = Default::default();
    let mut c2: i32 = Default::default();

    loop {
        c1 = c_tolower!(*p1).cast();
        c2 = c_tolower!(*p2).cast();

        if c1 != c2 {
            if c1 < c2 {
                return -1;
            } else {
                return 1;
            }
        }

        if c1 == '\0' as i32 {
            break;
        }

        p1 = p1 + 1;
        p2 = p2 + 1;
    }

    return 0;
}


