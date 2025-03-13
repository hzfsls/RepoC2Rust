pub fn RapidlzCopy8Byte(mut dst: Ptr<Void>, mut src: Ptr<Void>) {
    #[cfg(target_feature = "neon")]
    {
        vst1_u8!(dst.cast::<Ptr<u8>>(), vld1_u8!(src.cast::<Ptr<u8>>()));
    }
    #[cfg(not(target_feature = "neon"))]
    {
        RAPIDLZ_WRITE64BIT!(dst.cast(), RAPIDLZ_READ64BIT!(src.cast()));
    }
}
