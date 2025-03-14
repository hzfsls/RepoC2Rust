pub fn bloom_filter_new(mut table_size: u32, mut hash_func: BloomFilterHashFunc, mut num_functions: u32) -> Ptr<BloomFilter> {
    let mut filter: Ptr<BloomFilter> = Default::default();

    if (num_functions > (c_sizeof!(salts) / c_sizeof!(*salts)).as_bool() {
        return NULL!();
    }

    filter = c_malloc!(c_sizeof!(BloomFilter));

    if (filter == NULL!()).as_bool() {
        return NULL!();
    }

    filter.table = c_calloc!((table_size + 7) / 8, 1);

    if (filter.table == NULL!()).as_bool() {
        c_free!(filter);
        return NULL!();
    }

    filter.hash_func = hash_func;
    filter.num_functions = num_functions;
    filter.table_size = table_size;

    return filter.cast();
}
