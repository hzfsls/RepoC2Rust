pub fn bloom_filter_query(mut bloomfilter: Ptr<BloomFilter>, mut value: BloomFilterValue) -> i32 {
    let mut hash: u32 = Default::default();
    let mut subhash: u32 = Default::default();
    let mut index: u32 = Default::default();
    let mut i: u32 = Default::default();
    let mut b: u8 = Default::default();
    let mut bit: i32 = Default::default();

    hash = (bloomfilter.hash_func)(value);

    c_for!(let mut i: u32 = 0; i < bloomfilter.num_functions; i.prefix_plus_plus(); {
        subhash = (hash ^ salts[i]);
        index = (subhash % bloomfilter.table_size);
        let tmp0 = (index / 8);
        b = bloomfilter.table[tmp0];
        bit = (1 << (index % 8));
        if ((b & bit.cast::<u8>()) == 0) {
            return 0;
        }
    });

    return 1;
}
