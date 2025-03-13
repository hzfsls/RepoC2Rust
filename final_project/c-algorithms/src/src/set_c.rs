use crate::translation_utils::*;
pub use crate::src::set_h::*;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _SetEntry {
    pub data: SetValue,
    pub next: Ptr<SetEntry>,
}


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _Set {
    pub table: Ptr<Ptr<SetEntry>>,
    pub entries: u32,
    pub table_size: u32,
    pub prime_index: u32,
    pub hash_func: SetHashFunc,
    pub equal_func: SetEqualFunc,
    pub free_func: SetFreeFunc,
}


pub const set_primes: Array<u32, 24> = arr![
    193,      389,      769,      1543,      3079,      6151,      12289,     24593,
    49157,    98317,    196613,   393241,    786433,    1572869,   3145739,   6291469,
    12582917, 25165843, 50331653, 100663319, 201326611, 402653189, 805306457, 1610612741,
];


pub const set_num_primes: u32 = set_primes.len() as u32;


pub fn set_allocate_table(mut set: Ptr<Set>) -> i32 {
    unimplemented!();
}


pub fn set_free_entry(mut set: Ptr<Set>, mut entry: Ptr<SetEntry>) {
    if (set.free_func != NULL!()).as_bool() {
        (set.free_func)(entry.data.cast());
    }
    c_free!(entry);
}


pub fn set_new(mut hash_func: SetHashFunc, mut equal_func: SetEqualFunc) -> Ptr<Set> {
    let mut new_set: Ptr<Set> = c_malloc!(c_sizeof!(Set));

    if (new_set == NULL!()).as_bool() {
        return NULL!();
    }

    new_set.hash_func = hash_func.cast();
    new_set.equal_func = equal_func.cast();
    new_set.entries = 0;
    new_set.prime_index = 0;
    new_set.free_func = NULL!();

    if !set_allocate_table(new_set.cast()).as_bool() {
        c_free!(new_set);
        return NULL!();
    }

    return new_set.cast();
}


pub fn set_free(mut set: Ptr<Set>) {
    let mut rover: Ptr<SetEntry> = Default::default();
    let mut next: Ptr<SetEntry> = Default::default();
    let mut i: u32 = Default::default();

    c_for!(let mut i = 0; i < set.table_size; i.prefix_plus_plus(); {
        rover = set.table[i];

        while (rover != NULL!()) {
            next = rover.next;

            set_free_entry(set, rover);

            rover = next;
        }
    });

    c_free!(set.table);

    c_free!(set);
}


pub fn set_register_free_function(mut set: Ptr<Set>, mut free_func: SetFreeFunc) {
    set.free_func = free_func.cast();
}


pub fn set_enlarge(mut set: Ptr<Set>) -> i32 {
    let mut rover: Ptr<SetEntry> = Default::default();
    let mut next: Ptr<SetEntry> = Default::default();
    let mut old_table: Ptr<Ptr<SetEntry>> = Default::default();
    let mut old_table_size: u32 = Default::default();
    let mut old_prime_index: u32 = Default::default();
    let mut index: u32 = Default::default();
    let mut i: u32 = Default::default();

    old_table = set.table;
    old_table_size = set.table_size;
    old_prime_index = set.prime_index;

    set.prime_index.prefix_plus_plus();

    if !set_allocate_table(set).as_bool() {
        set.table = old_table;
        set.table_size = old_table_size;
        set.prime_index = old_prime_index;

        return 0;
    }

    c_for!(let mut i: u32 = 0; i < old_table_size; i.prefix_plus_plus(); {
        rover = old_table[i];

        while (rover != NULL!()) {
            next = rover.next;

            index = (set.hash_func)(rover.data) % set.table_size;
            rover.next = set.table[index];
            set.table[index] = rover;

            rover = next;
        }
    });

    c_free!(old_table);

    return 1;
}


pub fn set_insert(mut set: Ptr<Set>, mut data: SetValue) -> i32 {
    unimplemented!();
}


pub fn set_remove(mut set: Ptr<Set>, mut data: SetValue) -> i32 {
    unimplemented!();
}


pub fn set_query(mut set: Ptr<Set>, mut data: SetValue) -> i32 {
    unimplemented!();
}


pub fn set_num_entries(mut set: Ptr<Set>) -> u32 {
    return set.entries.cast();
}


pub fn set_to_array(mut set: Ptr<Set>) -> Ptr<SetValue> {
    let mut array: Ptr<SetValue> = Default::default();
    let mut array_counter: i32 = Default::default();
    let mut i: u32 = Default::default();
    let mut rover: Ptr<SetEntry> = Default::default();

    array = c_malloc!(c_sizeof!(SetValue) * set.entries);

    if (array == NULL!()).as_bool() {
        return NULL!();
    }

    array_counter = 0;

    c_for!(let mut i: u32 = 0; i < set.table_size; i.prefix_plus_plus(); {
        rover = set.table[i].cast();

        while (rover != NULL!()).as_bool() {
            array[array_counter] = rover.data.cast();
            array_counter.suffix_plus_plus();

            rover = rover.next.cast();
        }
    });

    return array.cast();
}


pub fn set_union(mut set1: Ptr<Set>, mut set2: Ptr<Set>) -> Ptr<Set> {
    unimplemented!();
}


pub fn set_intersection(mut set1: Ptr<Set>, mut set2: Ptr<Set>) -> Ptr<Set> {
    let mut new_set: Ptr<Set> = Default::default();
    let mut iterator: SetIterator = Default::default();
    let mut value: SetValue = Default::default();

    new_set = set_new(set1.hash_func.cast(), set2.equal_func.cast());

    if (new_set == NULL!()).as_bool() {
        return NULL!();
    }

    set_iterate(set1.cast(), c_ref!(iterator).cast());

    while set_iter_has_more(c_ref!(iterator).cast()).as_bool() {
        value = set_iter_next(c_ref!(iterator).cast()).cast();

        if (set_query(set2.cast(), value.cast()) != 0).as_bool() {
            if !set_insert(new_set.cast(), value.cast()).as_bool() {
                set_free(new_set.cast());

                return NULL!();
            }
        }
    }

    return new_set.cast();
}


pub fn set_iterate(mut set: Ptr<Set>, mut iter: Ptr<SetIterator>) {
    let mut chain: u32 = Default::default();

    iter.set = set.cast();
    iter.next_entry = NULL!();

    c_for!(chain = 0; chain < set.table_size; chain.prefix_plus_plus(); {
        if (set.table[chain] != NULL!()).as_bool() {
            iter.next_entry = set.table[chain].cast();
            break;
        }
    });

    iter.next_chain = chain.cast();
}


pub fn set_iter_next(mut iterator: Ptr<SetIterator>) -> SetValue {
    let mut set: Ptr<Set> = Default::default();
    let mut result: SetValue = Default::default();
    let mut current_entry: Ptr<SetEntry> = Default::default();
    let mut chain: u32 = Default::default();

    set = iterator.set.cast();

    if (iterator.next_entry == NULL!()).as_bool() {
        return SET_NULL!();
    }

    current_entry = iterator.next_entry.cast();
    result = current_entry.data.cast();

    if (current_entry.next != NULL!()).as_bool() {
        iterator.next_entry = current_entry.next.cast();
    } else {
        iterator.next_entry = NULL!();
        chain = (iterator.next_chain + 1).cast();
        while (chain < set.table_size).as_bool() {
            if (set.table[chain] != NULL!()).as_bool() {
                iterator.next_entry = set.table[chain].cast();
                break;
            }
            chain.prefix_plus_plus();
        }
        iterator.next_chain = chain.cast();
    }
    return result.cast();
}


pub fn set_iter_has_more(mut iterator: Ptr<SetIterator>) -> i32 {
    return (iterator.next_entry != NULL!()).as_bool().cast();
}


