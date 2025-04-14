pub fn RapidlzZeroBytesEncode(mut dest: Ptr<u8>, mut destSize: i32) -> i32 {
    RAPIDLZ_RETURN_IF_NOT_TRUE!(!(destSize <= 0).as_bool(), RAPIDLZ_ENC_NOT_OK!());
    dest[0] = 0;
    return 1;
}
