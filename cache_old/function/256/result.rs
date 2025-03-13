pub fn hash_table_iter_next(mut iterator: Ptr<HashTableIterator>) -> HashTablePair {
    let mut current_entry: Ptr<HashTableEntry> = Default::default();
    let mut hash_table: Ptr<HashTable> = Default::default();
    let mut pair: HashTablePair = HashTablePair { key: NULL!(), value: NULL!() };
    let mut chain: u32 = Default::default();

    hash_table = iterator.hash_table.cast();

    if iterator.next_entry == NULL!() {
        return pair;
    }

    current_entry = iterator.next_entry.cast();
    pair = current_entry.pair.cast();

    if current_entry.next != NULL!() {
        iterator.next_entry = current_entry.next.cast();
    } else {
        chain = (iterator.next_chain + 1).cast();
        iterator.next_entry = NULL!();
        while chain < hash_table.table_size {
            if hash_table.table[chain] != NULL!() {
                iterator.next_entry = hash_table.table[chain].cast();
                break;
            }
            chain.prefix_plus_plus();
        }
        iterator.next_chain = chain.cast();
    }

    return pair.cast();
}
