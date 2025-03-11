pub fn RapidlzHighBit64(mut x: u64) -> u8 {
    RAPIDLZ_ASSERT!(x != 0);

    #[cfg(all(target_arch = "x86_64", target_feature = "bmi1"))]
    {
        return ((__builtin_clzll(x) ^ 63) as u8).cast();
    }

    let mut pos: u8 = 64;
    let mut value: u64 = x;

    if value == 0 {
        return 0;
    }
    if (value & 0xFFFFFFFF00000000) == 0 {
        value <<= 32;
        pos -= 32;
    }
    if (value & 0xFFFF000000000000) == 0 {
        value <<= 16;
        pos -= 16;
    }
    if (value & 0xFF00000000000000) == 0 {
        value <<= 8;
        pos -= 8;
    }
    if (value & 0xF000000000000000) == 0 {
        value <<= 4;
        pos -= 4;
    }
    if (value & 0xC000000000000000) == 0 {
        value <<= 2;
        pos -= 2;
    }
    if (value & 0x8000000000000000) == 0 {
        value <<= 1;
        pos -= 1;
    }

    return (pos - 1).cast();
}
