pub fn CmptlzIsLE() -> i32 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64"))]
    {
        return (__BYTE_ORDER__!() == __ORDER_LITTLE_ENDIAN__!()).cast();
    }
    let mut n: i32 = 1;
    return (*c_ref!(n).cast::<Ptr<u8>>()).cast();
}
