use crate::translation_utils::*;
pub use crate::src::hash_pointer_h::*;

pub fn pointer_hash(mut location: Ptr<Void>) -> u32 {
    return location.cast::<usize>().cast::<u32>();
}


