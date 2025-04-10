pub fn bloom_filter_query(mut bloomfilter: Ptr<BloomFilter>, mut value: BloomFilterValue) -> i32 {
    let mut hash: u32 = Default::default();
    let mut subhash: u32 = Default::default();
    let mut index: u32 = Default::default();
    let mut i: u32 = Default::default();
    let mut b: u8 = Default::default();
    let mut bit: i32 = Default::default();

    hash = bloomfilter.hash_func(value).cast();

    c_for!(let mut i: u32 = 0; i < bloomfilter.num_functions.cast(); i.prefix_plus_plus(); {
        subhash = (hash ^ salts[i]).cast();
        index = (subhash % bloomfilter.table_size).cast();
        b = bloomfilter.table[index / 8].cast();
        bit = (1 << (index % 8)).cast();
        if ((b & bit) == 0).as_bool() {
            return 0;
        }
    });

    return 1;
}
