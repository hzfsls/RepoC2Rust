pub fn RapidlzIsLE() -> i32 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64"))]
    {
        return if cfg!(target_endian = "little") { 1 } else { 0 };
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm", target_arch = "aarch64")))]
    {
        let mut n: i32 = 1;
        return (*c_ref!(n).cast::<Ptr<u8>>()).cast::<i32>();
    }
}
