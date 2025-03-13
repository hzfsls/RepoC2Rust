use crate::translation_utils::*;
pub use crate::src::bloom_filter_h::*;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _BloomFilter {
    pub hash_func: BloomFilterHashFunc,
    pub table: Ptr<u8>,
    pub table_size: u32,
    pub num_functions: u32,
}


pub const salts: Array<u32, 64> = arr![
    0x1953c322, 0x588ccf17, 0x64bf600c, 0xa6be3f3d, 0x341a02ea, 0x15b03217, 0x3b062858, 0x5956fd06,
    0x18b5624f, 0xe3be0b46, 0x20ffcd5c, 0xa35dfd2b, 0x1fc4a9bf, 0x57c45d5c, 0xa8661c4a, 0x4f1b74d2,
    0x5a6dde13, 0x3b18dac6, 0x05a8afbf, 0xbbda2fe2, 0xa2520d78, 0xe7934849, 0xd541bc75, 0x09a55b57,
    0x9b345ae2, 0xfc2d26af, 0x38679cef, 0x81bd1e0d, 0x654681ae, 0x4b3d87ad, 0xd5ff10fb, 0x23b32f67,
    0xafc7e366, 0xdd955ead, 0xe7c34b1c, 0xfeace0a6, 0xeb16f09d, 0x3c57a72d, 0x2c8294c5, 0xba92662a,
    0xcd5b2d14, 0x743936c8, 0x2489beff, 0xc6c56e00, 0x74a4f606, 0xb244a94a, 0x5edfc423, 0xf1901934,
    0x24af7691, 0xf6c98b25, 0xea25af46, 0x76d5f2e6, 0x5e33cdf2, 0x445eb357, 0x88556bd2, 0x70d1da7a,
    0x54449368, 0x381020bc, 0x1c0520bf, 0xf7e44942, 0xa27e2a58, 0x66866fc5, 0x12519ce7, 0x437a8456,
];


pub fn bloom_filter_new(mut table_size: u32, mut hash_func: BloomFilterHashFunc, mut num_functions: u32) -> Ptr<BloomFilter> {
    unimplemented!();
}


pub fn bloom_filter_free(mut bloomfilter: Ptr<BloomFilter>) {
    c_free!(bloomfilter.table);
    c_free!(bloomfilter);
}


pub fn bloom_filter_insert(mut bloomfilter: Ptr<BloomFilter>, mut value: BloomFilterValue) {
    let mut hash: u32 = Default::default();
    let mut subhash: u32 = Default::default();
    let mut index: u32 = Default::default();
    let mut i: u32 = Default::default();
    let mut b: u8 = Default::default();

    hash = (bloomfilter.hash_func)(value.cast());

    c_for!(let mut i: u32 = 0; i < bloomfilter.num_functions.cast(); i.prefix_plus_plus(); {
        subhash = hash ^ salts[i];
        index = subhash % bloomfilter.table_size;
        b = (1 << (index % 8)).cast::<u8>();
        bloomfilter.table[index / 8] |= b;
    });
}


pub fn bloom_filter_query(mut bloomfilter: Ptr<BloomFilter>, mut value: BloomFilterValue) -> i32 {
    unimplemented!();
}


pub fn bloom_filter_read(mut bloomfilter: Ptr<BloomFilter>, mut array: Ptr<u8>) {
    let mut array_size: u32 = Default::default();
    array_size = (bloomfilter.table_size + 7) / 8;
    c_memcpy!(array, bloomfilter.table, array_size);
}


pub fn bloom_filter_load(mut bloomfilter: Ptr<BloomFilter>, mut array: Ptr<u8>) {
    let mut array_size: u32 = Default::default();
    array_size = (bloomfilter.table_size + 7) / 8;
    c_memcpy!(bloomfilter.table, array, array_size);
}


pub fn bloom_filter_union(mut filter1: Ptr<BloomFilter>, mut filter2: Ptr<BloomFilter>) -> Ptr<BloomFilter> {
    let mut result: Ptr<BloomFilter> = Default::default();
    let mut i: u32 = Default::default();
    let mut array_size: u32 = Default::default();

    if (filter1.table_size != filter2.table_size || filter1.num_functions != filter2.num_functions ||
        filter1.hash_func != filter2.hash_func).as_bool() {
        return NULL!();
    }

    result = bloom_filter_new(filter1.table_size.cast(), filter1.hash_func.cast(), filter1.num_functions.cast());

    if (result == NULL!()).as_bool() {
        return NULL!();
    }

    array_size = (filter1.table_size + 7) / 8;

    c_for!(let mut i: u32 = 0; i < array_size; i.prefix_plus_plus(); {
        result.table[i] = filter1.table[i] | filter2.table[i];
    });

    return result.cast();
}


pub fn bloom_filter_intersection(mut filter1: Ptr<BloomFilter>, mut filter2: Ptr<BloomFilter>) -> Ptr<BloomFilter> {
    let mut result: Ptr<BloomFilter> = Default::default();
    let mut i: u32 = Default::default();
    let mut array_size: u32 = Default::default();

    if (filter1.table_size != filter2.table_size || filter1.num_functions != filter2.num_functions || filter1.hash_func != filter2.hash_func).as_bool() {
        return NULL!();
    }

    result = bloom_filter_new(filter1.table_size.cast(), filter1.hash_func.cast(), filter1.num_functions.cast());

    if (result == NULL!()).as_bool() {
        return NULL!();
    }

    array_size = (filter1.table_size + 7) / 8;

    c_for!(let mut i: u32 = 0; i < array_size; i.prefix_plus_plus(); {
        result.table[i] = (filter1.table[i] & filter2.table[i]).cast();
    });

    return result.cast();
}


