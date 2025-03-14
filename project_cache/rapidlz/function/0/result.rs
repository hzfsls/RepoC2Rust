pub fn RapidlzIsLE() -> i32 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        return (__BYTE_ORDER__!() == __ORDER_LITTLE_ENDIAN__!()).cast();
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        let mut n: i32 = 1;
        return (*c_ref!(n).cast::<Ptr<u8>>()).cast();
    }
}
