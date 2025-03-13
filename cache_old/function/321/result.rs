pub fn trie_remove_binary(mut trie: Ptr<Trie>, mut key: Ptr<u8>, mut key_length: i32) -> i32 {
    let mut node: Ptr<TrieNode> = Default::default();
    let mut next: Ptr<TrieNode> = Default::default();
    let mut last_next_ptr: Ptr<Ptr<TrieNode>> = Default::default();
    let mut p: i32 = Default::default();
    let mut c: i32 = Default::default();

    node = trie_find_end_binary(trie.cast(), key.cast(), key_length.cast()).cast();

    if node != NULL!() && node.data != TRIE_NULL!() {
        node.data = TRIE_NULL!();
    } else {
        return 0;
    }

    node = trie.root_node.cast();
    last_next_ptr = c_ref!(trie.root_node).cast();
    p = 0;

    loop {
        c = key[p].cast::<i32>();
        next = node.next[c].cast();

        node.use_count -= 1;

        if node.use_count <= 0 {
            c_free!(node.cast());

            if last_next_ptr != NULL!() {
                *last_next_ptr = NULL!();
                last_next_ptr = NULL!();
            }
        }

        if p == key_length {
            break;
        } else {
            p += 1;
        }

        if last_next_ptr != NULL!() {
            last_next_ptr = c_ref!(node.next[c]).cast();
        }

        node = next.cast();
    }

    return 1;
}
