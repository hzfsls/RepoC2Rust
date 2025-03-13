use crate::translation_utils::*;
pub use crate::src::hash_table_h::*;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _HashTableEntry {
    pub pair: HashTablePair,
    pub next: Ptr<HashTableEntry>,
}


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _HashTable {
    pub table: Ptr<Ptr<HashTableEntry>>,
    pub table_size: u32,
    pub hash_func: HashTableHashFunc,
    pub equal_func: HashTableEqualFunc,
    pub key_free_func: HashTableKeyFreeFunc,
    pub value_free_func: HashTableValueFreeFunc,
    pub entries: u32,
    pub prime_index: u32,
}


pub const hash_table_primes: Array<u32, 24> = arr![
    193,      389,      769,      1543,      3079,      6151,      12289,     24593,
    49157,    98317,    196613,   393241,    786433,    1572869,   3145739,   6291469,
    12582917, 25165843, 50331653, 100663319, 201326611, 402653189, 805306457, 1610612741,
];


pub const hash_table_num_primes: u32 = hash_table_primes.len() as u32;


pub fn hash_table_allocate_table(mut hash_table: Ptr<HashTable>) -> i32 {
    unimplemented!();
}


pub fn hash_table_free_entry(mut hash_table: Ptr<HashTable>, mut entry: Ptr<HashTableEntry>) {
    let mut pair: Ptr<HashTablePair> = c_ref!(entry.pair).cast();

    if (hash_table.key_free_func != NULL!()).as_bool() {
        (hash_table.key_free_func)(pair.key.cast());
    }

    if (hash_table.value_free_func != NULL!()).as_bool() {
        (hash_table.value_free_func)(pair.value.cast());
    }

    c_free!(entry);
}


pub fn hash_table_new(mut hash_func: HashTableHashFunc, mut equal_func: HashTableEqualFunc) -> Ptr<HashTable> {
    let mut hash_table: Ptr<HashTable> = c_malloc!(c_sizeof!(HashTable));

    if (hash_table == NULL!()).as_bool() {
        return NULL!();
    }

    hash_table.hash_func = hash_func.cast();
    hash_table.equal_func = equal_func.cast();
    hash_table.key_free_func = NULL!();
    hash_table.value_free_func = NULL!();
    hash_table.entries = 0;
    hash_table.prime_index = 0;

    if !hash_table_allocate_table(hash_table.cast()).as_bool() {
        c_free!(hash_table);

        return NULL!();
    }

    return hash_table.cast();
}


pub fn hash_table_free(mut hash_table: Ptr<HashTable>) {
    let mut rover: Ptr<HashTableEntry> = Default::default();
    let mut next: Ptr<HashTableEntry> = Default::default();
    let mut i: u32 = Default::default();

    c_for!(let mut i = 0; i < hash_table.table_size; i.prefix_plus_plus(); {
        rover = hash_table.table[i];
        while (rover != NULL!()) {
            next = rover.next;
            hash_table_free_entry(hash_table, rover);
            rover = next;
        }
    });

    c_free!(hash_table.table);

    c_free!(hash_table);
}


pub fn hash_table_register_free_functions(mut hash_table: Ptr<HashTable>, mut key_free_func: HashTableKeyFreeFunc, mut value_free_func: HashTableValueFreeFunc) {
    hash_table.key_free_func = key_free_func.cast();
    hash_table.value_free_func = value_free_func.cast();
}


pub fn hash_table_enlarge(mut hash_table: Ptr<HashTable>) -> i32 {
    let mut old_table: Ptr<Ptr<HashTableEntry>> = Default::default();
    let mut old_table_size: u32 = Default::default();
    let mut old_prime_index: u32 = Default::default();
    let mut rover: Ptr<HashTableEntry> = Default::default();
    let mut pair: Ptr<HashTablePair> = Default::default();
    let mut next: Ptr<HashTableEntry> = Default::default();
    let mut index: u32 = Default::default();
    let mut i: u32 = Default::default();

    old_table = hash_table.table.cast();
    old_table_size = hash_table.table_size.cast();
    old_prime_index = hash_table.prime_index.cast();

    hash_table.prime_index.prefix_plus_plus();

    if !hash_table_allocate_table(hash_table.cast()).as_bool() {
        hash_table.table = old_table.cast();
        hash_table.table_size = old_table_size.cast();
        hash_table.prime_index = old_prime_index.cast();

        return 0;
    }

    c_for!(let mut i: u32 = 0; i < old_table_size; i.prefix_plus_plus(); {
        rover = old_table[i].cast();

        while (rover != NULL!()).as_bool() {
            next = rover.next.cast();

            pair = c_ref!(rover.pair).cast();

            index = (hash_table.hash_func)(pair.key.cast()) % hash_table.table_size;

            rover.next = hash_table.table[index].cast();
            hash_table.table[index] = rover.cast();

            rover = next.cast();
        }
    });

    c_free!(old_table);

    return 1;
}


pub fn hash_table_insert(mut hash_table: Ptr<HashTable>, mut key: HashTableKey, mut value: HashTableValue) -> i32 {
    unimplemented!();
}


pub fn hash_table_lookup(mut hash_table: Ptr<HashTable>, mut key: HashTableKey) -> HashTableValue {
    unimplemented!();
}


pub fn hash_table_remove(mut hash_table: Ptr<HashTable>, mut key: HashTableKey) -> i32 {
    unimplemented!();
}


pub fn hash_table_num_entries(mut hash_table: Ptr<HashTable>) -> u32 {
    return hash_table.entries.cast();
}


pub fn hash_table_iterate(mut hash_table: Ptr<HashTable>, mut iterator: Ptr<HashTableIterator>) {
    let mut chain: u32 = Default::default();

    iterator.hash_table = hash_table.cast();

    iterator.next_entry = NULL!();

    c_for!(chain = 0; chain < hash_table.table_size; chain.prefix_plus_plus(); {
        if (hash_table.table[chain] != NULL!()).as_bool() {
            iterator.next_entry = hash_table.table[chain].cast();
            iterator.next_chain = chain.cast();
            break;
        }
    });
}


pub fn hash_table_iter_has_more(mut iterator: Ptr<HashTableIterator>) -> i32 {
    return (iterator.next_entry != NULL!()).as_bool().cast::<i32>();
}


pub fn hash_table_iter_next(mut iterator: Ptr<HashTableIterator>) -> HashTablePair {
    let mut current_entry: Ptr<HashTableEntry> = Default::default();
    let mut hash_table: Ptr<HashTable> = Default::default();
    let mut pair: HashTablePair = HashTablePair { key: NULL!(), value: NULL!() };
    let mut chain: u32 = Default::default();

    hash_table = iterator.hash_table;

    if (iterator.next_entry == NULL!()) {
        return pair;
    }

    current_entry = iterator.next_entry;
    pair = current_entry.pair;

    if (current_entry.next != NULL!()) {
        iterator.next_entry = current_entry.next;
    } else {
        chain = (iterator.next_chain + 1);
        iterator.next_entry = NULL!();
        while (chain < hash_table.table_size) {
            if (hash_table.table[chain] != NULL!()) {
                iterator.next_entry = hash_table.table[chain];
                break;
            }
            chain.prefix_plus_plus();
        }
        iterator.next_chain = chain;
    }

    return pair;
}


