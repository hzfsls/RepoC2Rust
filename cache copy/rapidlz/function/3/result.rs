pub fn RapidlzCountLeadZero64(mut x: u64) -> u8 {
    #[cfg(all(target_arch = "x86_64", target_feature = "lzcnt"))]
    {
        return x.leading_zeros() as u8;
    }
    if (x == 0).as_bool() {
        return 0;
    }
    let mut num: u8 = 0;
    let mut val: u64 = x;
    while (val & 0x8000000000000000u64 == 0).as_bool() {
        val <<= 1;
        num += 1;
    }
    return num.cast();
}
