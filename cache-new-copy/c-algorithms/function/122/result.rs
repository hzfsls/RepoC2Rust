pub fn trie_insert(mut trie: Ptr<Trie>, mut key: Ptr<u8>, mut value: TrieValue) -> i32 {
    let mut rover: Ptr<Ptr<TrieNode>>;
    let mut node: Ptr<TrieNode>;
    let mut p: Ptr<u8>;
    let mut c: i32;

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

        if (c == 0).as_bool() {
            node.data = value.cast();

            break;
        }

        rover = c_ref!(node.next[c]).cast();
        p.prefix_plus_plus();
    }

    return 1;
}
