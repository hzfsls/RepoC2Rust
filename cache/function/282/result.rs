pub fn RapidlzCompWithPrefixDict(mut strmCtx: Ptr<RapidlzStreamCtx>, mut src: Ptr<u8>, mut dest: Ptr<u8>, mut srcSize: i32, mut destSize: i32) -> i32 {
    let mut curSrc: Ptr<u8> = src.cast();
    let mut curSrcAnchor: Ptr<u8> = curSrc.cast();
    let mut srcEnd: Ptr<u8> = (curSrc + srcSize).cast();
    let mut curDest: Ptr<u8> = dest.cast();
    let mut destEnd: Ptr<u8> = (curDest + destSize).cast();
    if srcSize < RAPIDLZ_LAST_LITERAL_LENGTH!() {
        return RapidlzEncLastLiterals(curSrcAnchor.cast(), srcEnd.cast(), curDest.cast(), destEnd.cast(), dest.cast());
    }
    let mut matchStartLimit: Ptr<u8> = (srcEnd - RAPIDLZ_MIN_COMPRESSED_SIZE!() + 1).cast();
    let mut matchEndLimit: Ptr<u8> = (srcEnd - RAPIDLZ_LAST_LITERALS!()).cast();
    let mut startIndex: u32 = strmCtx.currentOffset.cast();
    let mut base: Ptr<u8> = (src - startIndex).cast();

    let mut prefixDictStart: Ptr<u8>;
    let mut dictSize: u32;
    dictSize = strmCtx.dictSize.cast();
    prefixDictStart = (src - dictSize).cast();
    strmCtx.dictSize += srcSize.cast();
    let mut prefixDictLimit: u32 = (startIndex - dictSize).cast();
    strmCtx.currentOffset += srcSize.cast::<u32>();

    let mut hashValue: u32 = RapidlzHash4CalcValue(curSrc.cast());
    RapidlzHash4PutPos(startIndex.cast(), hashValue.cast(), strmCtx.hashTable.cast());
    curSrc += 1;
    let mut forwardHashValue: u32 = RapidlzHash4CalcValue(curSrc.cast());

    let mut match: Ptr<u8>;
    let mut token: Ptr<u8>;
    let mut acceleration: i32 = strmCtx.acceleration.cast();
    loop {
        let mut forwardPos: Ptr<u8> = curSrc.cast();
        let mut step: i32 = 1;
        let mut searchMatchNb: i32 = (acceleration << RAPIDLZ_STEP_FORWARD_BASE!()).cast();
        loop {
            hashValue = forwardHashValue.cast();
            let mut current: u32 = (forwardPos - base).cast();
            let mut matchOffset: u32 = RapidlzHash4GetPos(hashValue.cast(), strmCtx.hashTable.cast());
            curSrc = forwardPos.cast();
            forwardPos += step;
            step = (searchMatchNb.suffix_plus_plus() >> RAPIDLZ_STEP_FORWARD_BASE!()).cast();

            if RAPIDLZ_UNLIKELY!(forwardPos > matchStartLimit) {
                return RapidlzEncLastLiterals(curSrcAnchor.cast(), srcEnd.cast(), curDest.cast(), destEnd.cast(), dest.cast());
            }

            match = (base + matchOffset).cast();
            forwardHashValue = RapidlzHash4CalcValue(forwardPos.cast());
            RapidlzHash4PutPos(current.cast(), hashValue.cast(), strmCtx.hashTable.cast());

            if matchOffset < prefixDictLimit {
                continue;
            }
            if (matchOffset + RAPIDLZ_MAX_OFFSET!()) < current {
                continue;
            }
            if RAPIDLZ_READ32BIT!(curSrc) == RAPIDLZ_READ32BIT!(match) {
                break;
            }
        }

        RAPIDLZ_EXPAND_FORWARD!(prefixDictStart, match, curSrc, curSrcAnchor);

        token = curDest.cast();
        if !RapidlzStreamEncLiterals(curSrc.cast(), curSrcAnchor.cast(), c_ref!(curDest).cast(), destEnd.cast()).as_bool() {
            return RAPIDLZ_ENC_NOT_OK!();
        }

        _OFFSET_AND_MATCH:

        RapidlzWriteLE16(curDest.cast(), (curSrc - match).cast());
        curDest += 2;

        let mut matchLen: u32;
        let mut curSrcMatchEnd: Ptr<u8>;

        curSrcMatchEnd = RapidlzCompressExpandBackward(matchEndLimit.cast(), (match + RAPIDLZ_MIN_MATCH!()).cast(), (curSrc + RAPIDLZ_MIN_MATCH!()).cast()).cast();
        matchLen = (curSrcMatchEnd - curSrc - RAPIDLZ_MIN_MATCH!()).cast();
        curSrc = curSrcMatchEnd.cast();
        #[cfg(RAPIDLZ_DEBUG)]
        if RAPIDLZ_UNLIKELY!(RAPIDLZ_LIT_AND_MATCH_COPY_END!(curDest, matchLen) > destEnd) {
            return RAPIDLZ_ENC_NOT_OK!();
        }
        curDest += RapidlzStoreMatchLen(matchLen.cast(), token.cast(), curDest.cast());

        curSrcAnchor = curSrc.cast();
        if curSrc >= matchStartLimit {
            break;
        }
        let mut hv: u32 = RapidlzHash4CalcValue((curSrc - 2).cast());
        let mut index: u32 = (curSrc - 2 - base).cast();
        RapidlzHash4PutPos(index.cast(), hv.cast(), strmCtx.hashTable.cast());

        hashValue = RapidlzHash4CalcValue(curSrc.cast());
        current = (curSrc - base).cast();
        matchOffset = RapidlzHash4GetPos(hashValue.cast(), strmCtx.hashTable.cast());

        match = (base + matchOffset).cast();

        RapidlzHash4PutPos(current.cast(), hashValue.cast(), strmCtx.hashTable.cast());
        if (matchOffset >= prefixDictLimit) && ((matchOffset + RAPIDLZ_MAX_OFFSET!()) >= current) {
            if RAPIDLZ_READ32BIT!(curSrc) == RAPIDLZ_READ32BIT!(match) {
                token = curDest.cast();
                curDest.suffix_plus_plus();
                *token = 0;
                goto!(_OFFSET_AND_MATCH);
            }
        }
        forwardHashValue = RapidlzHash4CalcValue(curSrc.prefix_plus_plus().cast());
    }

    return RapidlzEncLastLiterals(curSrcAnchor.cast(), srcEnd.cast(), curDest.cast(), destEnd.cast(), dest.cast());
}
