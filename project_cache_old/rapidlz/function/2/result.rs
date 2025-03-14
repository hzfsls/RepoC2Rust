pub fn RapidlzCountTailZero64(mut x: u64) -> u8 {
    #[cfg(all(target_arch = "x86_64", target_feature = "bmi1"))]
    {
        return x.trailing_zeros() as u8;
    }
    if (x == 0).as_bool() {
        return 0;
    }
    let mut val: u64 = x;
    let mut num: u8 = 0;
    while ((val & 1) == 0).as_bool() {
        val >>= 1;
        num += 1;
    }
    return num.cast();
}
