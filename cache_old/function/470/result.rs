pub fn RapidlzEncLastLiterals(mut curSrcAnchor: Ptr<u8>, mut srcEnd: Ptr<u8>, mut curDest: Ptr<u8>, mut destEnd: Ptr<u8>, mut destStart: Ptr<u8>) -> i32 {
    let mut lastLiteralsLen: usize = (srcEnd - curSrcAnchor).cast();
    RAPIDLZ_RETURN_IF_NOT_TRUE!(!(RAPIDLZ_LITERAL_LEN_COPY_END!(curDest, lastLiteralsLen) > destEnd), RAPIDLZ_ENC_NOT_OK!());
    curDest += RapidlzStoreLiteralLen(lastLiteralsLen.cast(), curDest.cast()).cast();
    RAPIDLZ_RETURN_IF_NOT_EOK!(c_memcpy_s!(curDest, destEnd - curDest, curSrcAnchor, lastLiteralsLen), RAPIDLZ_ENC_NOT_OK!());
    curDest += lastLiteralsLen;
    return (curDest.cast::<Ptr<Void>>() - destStart.cast::<Ptr<Void>>()).cast::<i32>();
}
