pub fn RapidlzStreamEncLiterals(mut curSrc: Ptr<u8>, mut curSrcAnchor: Ptr<u8>, mut curDest: Ptr<Ptr<u8>>, mut destEnd: Ptr<u8>) -> bool {
    let mut litLen: u32 = (curSrc - curSrcAnchor).cast();
    RAPIDLZ_RETURN_IF_NOT_TRUE!(!(RAPIDLZ_UNLIKELY!(RAPIDLZ_LITERAL_LEN_COPY_END!(*curDest, litLen) > destEnd).as_bool()), false);
    *curDest += RapidlzStoreLiteralLen(litLen.cast(), *curDest.cast()).cast();
    RapidlzWildCopy8(curSrcAnchor.cast(), *curDest.cast(), (*curDest + litLen).cast());
    *curDest += litLen;
    return true;
}
