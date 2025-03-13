use crate::translation_utils::*;
pub use crate::src::hash_int_h::*;

pub fn int_hash(mut vlocation: Ptr<Void>) -> u32 {
    let mut location: Ptr<i32> = vlocation.cast::<Ptr<i32>>();
    return (*location).cast::<u32>();
}


