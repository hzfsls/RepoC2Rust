pub fn set_num_entries(mut set: Ptr<Set>) -> u32 {
    return set.entries.cast();
}
