use crate::translation_utils::*;
pub use crate::src::trie_c::trie_lookup;
pub use crate::src::trie_c::trie_insert;
pub use crate::src::trie_c::trie_remove_binary;
pub use crate::src::trie_c::trie_insert_binary;
pub use crate::src::trie_c::trie_new;
pub use crate::src::trie_c::_Trie;
pub use crate::src::trie_c::trie_num_entries;
pub use crate::src::trie_c::trie_free;
pub use crate::src::trie_c::trie_lookup_binary;
pub use crate::src::trie_c::trie_remove;

pub type Trie = _Trie;


pub type TrieValue = VoidPtr;


macro_rules! ALGORITHM_TRIE_H { () => { } }
pub(crate) use ALGORITHM_TRIE_H;


macro_rules! TRIE_NULL { () => { NULL!() } }
pub(crate) use TRIE_NULL;


