pub fn hash_table_remove(mut hash_table: Ptr<HashTable>, mut key: HashTableKey) -> i32 {
    let mut rover: Ptr<Ptr<HashTableEntry>>;
    let mut entry: Ptr<HashTableEntry>;
    let mut pair: Ptr<HashTablePair>;
    let mut index: u32;
    let mut result: i32;

    index = (hash_table.hash_func)(key) % hash_table.table_size;

    result = 0;
    rover = c_ref!(hash_table.table[index]);

    while (*rover != NULL!()) {
        pair = c_ref!((*rover).pair);

        if (hash_table.equal_func)(key, pair.key) != 0 {
            entry = *rover;

            *rover = entry.next;

            hash_table_free_entry(hash_table, entry);

            hash_table.entries -= 1;

            result = 1;

            break;
        }

        rover = c_ref!((*rover).next);
    }

    return result;
}
