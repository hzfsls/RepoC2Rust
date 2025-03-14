pub fn hash_table_free(mut hash_table: Ptr<HashTable>) {
    let mut rover: Ptr<HashTableEntry> = Default::default();
    let mut next: Ptr<HashTableEntry> = Default::default();
    let mut i: u32 = Default::default();

    c_for!(let mut i: u32 = 0; i < hash_table.table_size; i.prefix_plus_plus(); {
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
