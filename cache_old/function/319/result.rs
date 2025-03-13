pub fn trie_insert(mut trie: Ptr<Trie>, mut key: Ptr<u8>, mut value: TrieValue) -> i32 {
    let mut rover: Ptr<Ptr<TrieNode>> = Default::default();
    let mut node: Ptr<TrieNode> = Default::default();
    let mut p: Ptr<u8> = Default::default();
    let mut c: i32 = Default::default();

    if value == TRIE_NULL!() {
        return 0;
    }

    node = trie_find_end(trie.cast(), key.cast()).cast();

    if node != NULL!() && node.data != TRIE_NULL!() {
        node.data = value.cast();
        return 1;
    }

    rover = c_ref!(trie.root_node).cast();
    p = key.cast();

    loop {
        node = *rover;

        if node == NULL!() {
            node = c_calloc!(1, c_sizeof!(TrieNode));

            if node == NULL!() {
                trie_insert_rollback(trie.cast(), key.cast::<Ptr<u8>>());

                return 0;
            }

            node.data = TRIE_NULL!();

            *rover = node.cast();
        }

        node.use_count.suffix_plus_plus();

        c = (*p).cast::<u8>().cast::<i32>();

        if c == '\0' as i32 {
            node.data = value.cast();

            break;
        }

        rover = c_ref!(node.next[c]).cast();
        p.suffix_plus_plus();
    }

    return 1;
}
