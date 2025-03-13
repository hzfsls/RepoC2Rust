pub fn bloom_filter_insert(mut bloomfilter: Ptr<BloomFilter>, mut value: BloomFilterValue) {
    let mut hash: u32 = Default::default();
    let mut subhash: u32 = Default::default();
    let mut index: u32 = Default::default();
    let mut i: u32 = Default::default();
    let mut b: u8 = Default::default();

    hash = (bloomfilter.hash_func)(value.cast()).cast();

    c_for!(let mut i: u32 = 0; i < bloomfilter.num_functions; i.prefix_plus_plus(); {
        subhash = hash ^ salts[i].cast();
        index = subhash % bloomfilter.table_size.cast();
        b = (1 << (index % 8)).cast::<u8>();
        bloomfilter.table[index / 8] |= b.cast();
    });
}
