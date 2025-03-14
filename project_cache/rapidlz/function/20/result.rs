pub fn RapidlzStoreLiteralLen(mut litLen: u32, mut curDest: Ptr<u8>) -> usize {
    let mut curDestAnchor: Ptr<u8> = curDest.cast();
    if (litLen >= RAPIDLZ_MAX_4BIT_VALUE!()).as_bool() {
        let mut tmp: u32 = litLen - RAPIDLZ_MAX_4BIT_VALUE!();
        *curDest = (RAPIDLZ_MAX_4BIT_VALUE!() << 4).cast();
        curDest += 1;
        while (tmp >= RAPIDLZ_MAX_BYTE_VALUE!()).as_bool() {
            *curDest = RAPIDLZ_MAX_BYTE_VALUE!().cast();
            curDest += 1;
            tmp -= RAPIDLZ_MAX_BYTE_VALUE!();
        }
        *curDest = tmp.cast::<u8>();
        curDest += 1;
    } else {
        *curDest = (litLen << 4).cast::<u8>();
        curDest += 1;
    }
    return (curDest - curDestAnchor).cast::<usize>();
}
