use crate::translation_utils::*;
pub use crate::src::compare_int_h::*;

pub fn int_equal(mut vlocation1: Ptr<Void>, mut vlocation2: Ptr<Void>) -> i32 {
    let mut location1: Ptr<i32> = vlocation1.cast::<Ptr<i32>>();
    let mut location2: Ptr<i32> = vlocation2.cast::<Ptr<i32>>();

    return (*location1 == *location2).cast();
}


pub fn int_compare(mut vlocation1: Ptr<Void>, mut vlocation2: Ptr<Void>) -> i32 {
    let mut location1: Ptr<i32> = vlocation1.cast::<Ptr<i32>>();
    let mut location2: Ptr<i32> = vlocation2.cast::<Ptr<i32>>();

    if (*location1 < *location2).as_bool() {
        return -1;
    } else if (*location1 > *location2).as_bool() {
        return 1;
    } else {
        return 0;
    }
}


