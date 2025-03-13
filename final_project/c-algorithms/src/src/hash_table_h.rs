use crate::translation_utils::*;
pub use crate::src::hash_table_c::_HashTableEntry;
pub use crate::src::hash_table_c::hash_table_lookup;
pub use crate::src::hash_table_c::hash_table_iterate;
pub use crate::src::hash_table_c::hash_table_register_free_functions;
pub use crate::src::hash_table_c::hash_table_remove;
pub use crate::src::hash_table_c::hash_table_iter_has_more;
pub use crate::src::hash_table_c::hash_table_num_entries;
pub use crate::src::hash_table_c::hash_table_insert;
pub use crate::src::hash_table_c::hash_table_new;
pub use crate::src::hash_table_c::hash_table_iter_next;
pub use crate::src::hash_table_c::hash_table_free;
pub use crate::src::hash_table_c::_HashTable;

pub type HashTable = _HashTable;


pub type HashTableIterator = _HashTableIterator;


pub type HashTableEntry = _HashTableEntry;


pub type HashTableKey = VoidPtr;


pub type HashTableValue = VoidPtr;


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _HashTablePair {
    pub key: HashTableKey,
    pub value: HashTableValue,
}

pub type HashTablePair = _HashTablePair;


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _HashTableIterator {
    pub hash_table: Ptr<HashTable>,
    pub next_entry: Ptr<HashTableEntry>,
    pub next_chain: u32,
}


pub type HashTableHashFunc = FuncPtr<fn(HashTableKey) -> u32>;


pub type HashTableEqualFunc = FuncPtr<fn(HashTableKey, HashTableKey) -> i32>;


pub type HashTableKeyFreeFunc = FuncPtr<fn(HashTableKey)>;


pub type HashTableValueFreeFunc = FuncPtr<fn(HashTableValue)>;


macro_rules! ALGORITHM_HASH_TABLE_H { () => { } }
pub(crate) use ALGORITHM_HASH_TABLE_H;


macro_rules! HASH_TABLE_NULL { () => { NULL!() } }
pub(crate) use HASH_TABLE_NULL;


