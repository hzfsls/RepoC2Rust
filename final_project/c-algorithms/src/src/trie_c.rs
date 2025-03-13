use crate::translation_utils::*;
pub use crate::src::trie_h::*;

pub type TrieNode = _TrieNode;


#[repr(C)]
#[derive(Default)]
pub struct _TrieNode {
    pub data: TrieValue,
    pub use_count: u32,
    pub next: Array<Ptr<TrieNode>, 256>,
}


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _Trie {
    pub root_node: Ptr<TrieNode>,
}


pub fn trie_new() -> Ptr<Trie> {
    let mut new_trie: Ptr<Trie> = c_malloc!(c_sizeof!(Trie));

    if (new_trie == NULL!()).as_bool() {
        return NULL!();
    }

    new_trie.root_node = NULL!();

    return new_trie.cast();
}


pub fn trie_free_list_push(mut list: Ptr<Ptr<TrieNode>>, mut node: Ptr<TrieNode>) {
    unimplemented!();
}


pub fn trie_free_list_pop(mut list: Ptr<Ptr<TrieNode>>) -> Ptr<TrieNode> {
    let mut result: Ptr<TrieNode> = Default::default();

    result = *list;
    *list = result.data.cast();

    return result.cast();
}


pub fn trie_free(mut trie: Ptr<Trie>) {
    let mut free_list: Ptr<TrieNode> = NULL!();
    let mut node: Ptr<TrieNode> = Default::default();
    let mut i: i32 = Default::default();

    if (trie.root_node != NULL!()).as_bool() {
        trie_free_list_push(c_ref!(free_list).cast(), trie.root_node.cast());
    }

    while (free_list != NULL!()).as_bool() {
        node = trie_free_list_pop(c_ref!(free_list).cast());

        c_for!(let mut i: i32 = 0; i < 256; i.prefix_plus_plus(); {
            if (node.next[i] != NULL!()).as_bool() {
                trie_free_list_push(c_ref!(free_list).cast(), node.next[i].cast());
            }
        });

        c_free!(node);
    }

    c_free!(trie);
}


pub fn trie_find_end(mut trie: Ptr<Trie>, mut key: Ptr<u8>) -> Ptr<TrieNode> {
    unimplemented!();
}


pub fn trie_find_end_binary(mut trie: Ptr<Trie>, mut key: Ptr<u8>, mut key_length: i32) -> Ptr<TrieNode> {
    let mut node: Ptr<TrieNode> = Default::default();
    let mut j: i32 = Default::default();
    let mut c: u8 = Default::default();

    node = trie.root_node.cast();

    c_for!(let mut j = 0; j < key_length; j.suffix_plus_plus(); {
        if (node == NULL!()).as_bool() {
            return NULL!();
        }

        c = key[j].cast::<u8>();

        node = node.next[c].cast();
    });

    return node.cast();
}


pub fn trie_insert_rollback(mut trie: Ptr<Trie>, mut key: Ptr<u8>) {
    let mut node: Ptr<TrieNode> = Default::default();
    let mut prev_ptr: Ptr<Ptr<TrieNode>> = Default::default();
    let mut next_node: Ptr<TrieNode> = Default::default();
    let mut next_prev_ptr: Ptr<Ptr<TrieNode>> = Default::default();
    let mut p: Ptr<u8> = Default::default();

    node = trie.root_node.cast();
    prev_ptr = c_ref!(trie.root_node).cast();
    p = key.cast();

    while (node != NULL!()).as_bool() {
        next_prev_ptr = c_ref!(node.next[*p]).cast();
        next_node = *next_prev_ptr;
        p = p + 1;

        node.use_count -= 1;

        if (node.use_count == 0).as_bool() {
            c_free!(node);

            if (prev_ptr != NULL!()).as_bool() {
                *prev_ptr = NULL!();
            }

            next_prev_ptr = NULL!();
        }

        node = next_node.cast();
        prev_ptr = next_prev_ptr.cast();
    }
}


pub fn trie_insert(mut trie: Ptr<Trie>, mut key: Ptr<u8>, mut value: TrieValue) -> i32 {
    let mut rover: Ptr<Ptr<TrieNode>> = Default::default();
    let mut node: Ptr<TrieNode> = Default::default();
    let mut p: Ptr<u8> = Default::default();
    let mut c: i32 = Default::default();

    if (value == TRIE_NULL!()).as_bool() {
        return 0;
    }

    node = trie_find_end(trie.cast(), key.cast()).cast();

    if (node != NULL!()).as_bool() && (node.data != TRIE_NULL!()).as_bool() {
        node.data = value.cast();
        return 1;
    }

    rover = c_ref!(trie.root_node).cast();
    p = key.cast();

    loop {
        node = *rover;

        if (node == NULL!()).as_bool() {
            node = c_calloc!(1, c_sizeof!(TrieNode));

            if (node == NULL!()).as_bool() {
                trie_insert_rollback(trie.cast(), key.cast::<Ptr<u8>>());

                return 0;
            }

            node.data = TRIE_NULL!();

            *rover = node.cast();
        }

        node.use_count.prefix_plus_plus();

        c = (*p).cast::<u8>().cast::<i32>();

        if (c == '\0' as i32).as_bool() {
            node.data = value.cast();

            break;
        }

        rover = c_ref!(node.next[c]).cast();
        p.prefix_plus_plus();
    }

    return 1;
}


pub fn trie_insert_binary(mut trie: Ptr<Trie>, mut key: Ptr<u8>, mut key_length: i32, mut value: TrieValue) -> i32 {
    let mut rover: Ptr<Ptr<TrieNode>> = Default::default();
    let mut node: Ptr<TrieNode> = Default::default();
    let mut p: i32 = Default::default();
    let mut c: i32 = Default::default();

    if (value == TRIE_NULL!()).as_bool() {
        return 0;
    }

    node = trie_find_end_binary(trie.cast(), key.cast(), key_length.cast()).cast();

    if (node != NULL!()).as_bool() && (node.data != TRIE_NULL!()).as_bool() {
        node.data = value.cast();
        return 1;
    }

    rover = c_ref!(trie.root_node).cast();
    p = 0;

    loop {
        node = *rover;

        if (node == NULL!()).as_bool() {
            node = c_calloc!(1, c_sizeof!(TrieNode));

            if (node == NULL!()).as_bool() {
                trie_insert_rollback(trie.cast(), key.cast());
                return 0;
            }

            node.data = TRIE_NULL!();
            *rover = node.cast();
        }

        node.use_count.prefix_plus_plus();

        c = key[p].cast::<u8>().cast::<i32>();

        if (p == key_length).as_bool() {
            node.data = value.cast();
            break;
        }

        rover = c_ref!(node.next[c]).cast();
        p.prefix_plus_plus();
    }

    return 1;
}


pub fn trie_remove_binary(mut trie: Ptr<Trie>, mut key: Ptr<u8>, mut key_length: i32) -> i32 {
    let mut node: Ptr<TrieNode> = Default::default();
    let mut next: Ptr<TrieNode> = Default::default();
    let mut last_next_ptr: Ptr<Ptr<TrieNode>> = Default::default();
    let mut p: i32 = Default::default();
    let mut c: u8 = Default::default();

    node = trie_find_end_binary(trie.cast(), key.cast(), key_length.cast()).cast();

    if (node != NULL!()).as_bool() && (node.data != TRIE_NULL!()).as_bool() {
        node.data = TRIE_NULL!();
    } else {
        return 0;
    }

    node = trie.root_node.cast();
    last_next_ptr = c_ref!(trie.root_node).cast();
    p = 0;

    loop {
        c = key[p].cast();
        next = node.next[c].cast();

        node.use_count -= 1;

        if (node.use_count <= 0).as_bool() {
            c_free!(node);

            if (last_next_ptr != NULL!()).as_bool() {
                *last_next_ptr = NULL!();
                last_next_ptr = NULL!();
            }
        }

        if (p == key_length).as_bool() {
            break;
        } else {
            p += 1;
        }

        if (last_next_ptr != NULL!()).as_bool() {
            last_next_ptr = c_ref!(node.next[c]).cast();
        }

        node = next.cast();
    }

    return 1;
}


pub fn trie_remove(mut trie: Ptr<Trie>, mut key: Ptr<u8>) -> i32 {
    unimplemented!();
}


pub fn trie_lookup(mut trie: Ptr<Trie>, mut key: Ptr<u8>) -> TrieValue {
    let mut node: Ptr<TrieNode> = Default::default();

    node = trie_find_end(trie.cast(), key.cast());

    if (node != NULL!()).as_bool() {
        return node.data.cast();
    } else {
        return TRIE_NULL!();
    }
}


pub fn trie_lookup_binary(mut trie: Ptr<Trie>, mut key: Ptr<u8>, mut key_length: i32) -> TrieValue {
    let mut node: Ptr<TrieNode> = Default::default();

    node = trie_find_end_binary(trie.cast(), key.cast(), key_length.cast());

    if (node != NULL!()).as_bool() {
        return node.data.cast();
    } else {
        return TRIE_NULL!();
    }
}


pub fn trie_num_entries(mut trie: Ptr<Trie>) -> u32 {
    if (trie.root_node == NULL!()).as_bool() {
        return 0;
    } else {
        return trie.root_node.use_count.cast();
    }
}


