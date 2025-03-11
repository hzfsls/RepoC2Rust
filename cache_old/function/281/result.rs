pub fn RapidlzCompWithExternalDict(mut strmCtx: Ptr<RapidlzStreamCtx>, mut src: Ptr<u8>, mut dest: Ptr<u8>, mut srcSize: i32, mut destSize: i32) -> i32 {
    let mut curSrc: Ptr<u8> = src.cast();
    let mut curSrcAnchor: Ptr<u8> = curSrc.cast();
    let mut srcEnd: Ptr<u8> = curSrc + srcSize;
    let mut curDest: Ptr<u8> = dest.cast();
    let mut destEnd: Ptr<u8> = curDest + destSize;
    if srcSize < RAPIDLZ_LAST_LITERAL_LENGTH!() {
        return RapidlzEncLastLiterals(curSrcAnchor.cast(), srcEnd.cast(), curDest.cast(), destEnd.cast(), dest.cast());
    }

    let mut matchStartLimit: Ptr<u8> = srcEnd - RAPIDLZ_MIN_COMPRESSED_SIZE!() + 1;
    let mut matchEndLimit: Ptr<u8> = srcEnd - RAPIDLZ_LAST_LITERALS!();
    let mut startIndex: u32 = strmCtx.currentOffset;
    let mut base: Ptr<u8> = src.cast::<Ptr<u8>>() - startIndex;

    let mut dict: Ptr<u8> = Default::default();
    let mut dictBase: Ptr<u8> = Default::default();
    let mut dictEnd: Ptr<u8> = Default::default();
    let mut prefixDictStart: Ptr<u8> = Default::default();
    let mut dictSize: u32 = Default::default();
    let mut offset: u32 = 0;

    dict = strmCtx.dict.cast();
    dictSize = strmCtx.dictSize;
    dictBase = dict + dictSize - strmCtx.currentOffset;
    prefixDictStart = 0;
    dictBase = dict + dictSize - strmCtx.currentOffset;
    dictEnd = dict + dictSize;
    strmCtx.dictSize += srcSize;

    let mut prefixDictLimit: u32 = startIndex - dictSize;
    strmCtx.currentOffset += srcSize.cast::<u32>();

    let mut hashValue: u32 = RapidlzHash4CalcValue(curSrc.cast());
    RapidlzHash4PutPos(startIndex, hashValue, strmCtx.hashTable.cast());
    curSrc += 1;
    let mut forwardHashValue: u32 = RapidlzHash4CalcValue(curSrc.cast());

    let mut match: Ptr<u8> = Default::default();
    let mut token: Ptr<u8> = Default::default();
    let mut acceleration: i32 = strmCtx.acceleration;
    loop {
        let mut forwardPos: Ptr<u8> = curSrc;
        let mut jumpStep: i32 = 1;
        let mut searchMatchNb: i32 = acceleration << RAPIDLZ_STEP_FORWARD_BASE!();
        loop {
            hashValue = forwardHashValue;
            let mut current: u32 = (forwardPos - base).cast();
            let mut matchOffset: u32 = RapidlzHash4GetPos(hashValue, strmCtx.hashTable.cast());
            curSrc = forwardPos;
            forwardPos += jumpStep;
            jumpStep = (searchMatchNb >> RAPIDLZ_STEP_FORWARD_BASE!());
            searchMatchNb += 1;

            if RAPIDLZ_UNLIKELY!(forwardPos > matchStartLimit) {
                return RapidlzEncLastLiterals(curSrcAnchor.cast(), srcEnd.cast(), curDest.cast(), destEnd.cast(), dest.cast());
            }

            if matchOffset < startIndex {
                match = dictBase + matchOffset;
                prefixDictStart = dict;
            } else {
                match = base + matchOffset;
                prefixDictStart = src.cast();
            }

            forwardHashValue = RapidlzHash4CalcValue(forwardPos.cast());
            RapidlzHash4PutPos(current, hashValue, strmCtx.hashTable.cast());
            RAPIDLZ_CONTINUE_IF_NOT_A_MATCH!(matchOffset, prefixDictLimit, current);
            if RAPIDLZ_READ32BIT!(curSrc) == RAPIDLZ_READ32BIT!(match) {
                offset = current - matchOffset;
                break;
            }
        }

        RAPIDLZ_EXPAND_FORWARD!(prefixDictStart, match, curSrc, curSrcAnchor);

        token = curDest;
        if !RapidlzStreamEncLiterals(curSrc.cast(), curSrcAnchor.cast(), c_ref!(curDest).cast(), destEnd.cast()) {
            return RAPIDLZ_ENC_NOT_OK!();
        }

        _OFFSET_AND_MATCH:

        RapidlzWriteLE16(curDest.cast(), offset);
        curDest += 2;

        let mut matchLen: u32;
        let mut curSrcMatchEnd: Ptr<u8>;

        if prefixDictStart == dict {
            let mut srcLimitOnlyWithDict: Ptr<u8> = curSrc + (dictEnd - match);
            if srcLimitOnlyWithDict > matchEndLimit {
                srcLimitOnlyWithDict = matchEndLimit;
            }
            curSrcMatchEnd = RapidlzCompressExpandBackward(srcLimitOnlyWithDict.cast(), match + RAPIDLZ_MIN_MATCH!(), curSrc + RAPIDLZ_MIN_MATCH!()).cast();
            matchLen = (curSrcMatchEnd - curSrc - RAPIDLZ_MIN_MATCH!()).cast();
            curSrc = curSrcMatchEnd;

            if curSrc == srcLimitOnlyWithDict {
                curSrcMatchEnd = RapidlzCompressExpandBackward(matchEndLimit.cast(), src.cast(), srcLimitOnlyWithDict.cast()).cast();
                matchLen += (curSrcMatchEnd - curSrc).cast();
                curSrc = curSrcMatchEnd;
            }
        } else {
            curSrcMatchEnd = RapidlzCompressExpandBackward(matchEndLimit.cast(), match + RAPIDLZ_MIN_MATCH!(), curSrc + RAPIDLZ_MIN_MATCH!()).cast();
            matchLen = (curSrcMatchEnd - curSrc - RAPIDLZ_MIN_MATCH!()).cast();
            curSrc = curSrcMatchEnd;
        }
        #[cfg(RAPIDLZ_DEBUG)]
        if RAPIDLZ_UNLIKELY!(RAPIDLZ_LIT_AND_MATCH_COPY_END!(curDest, matchLen) > destEnd) {
            return RAPIDLZ_ENC_NOT_OK!();
        }
        curDest += RapidlzStoreMatchLen(matchLen, token, curDest);
        curSrcAnchor = curSrc;
        if curSrc >= matchStartLimit {
            break;
        }
        let mut hv2: u32 = RapidlzHash4CalcValue(curSrc - 2);
        let mut index: u32 = (curSrc - 2 - base).cast();
        RapidlzHash4PutPos(index, hv2, strmCtx.hashTable.cast());

        hashValue = RapidlzHash4CalcValue(curSrc.cast());
        let mut current: u32 = (curSrc - base).cast();
        let mut matchOffset: u32 = RapidlzHash4GetPos(hashValue, strmCtx.hashTable.cast());
        if matchOffset < startIndex {
            match = dictBase + matchOffset;
            prefixDictStart = dict;
        } else {
            match = base + matchOffset;
            prefixDictStart = src.cast();
        }

        RapidlzHash4PutPos(current, hashValue, strmCtx.hashTable.cast());

        if (matchOffset >= prefixDictLimit) && (matchOffset + RAPIDLZ_MAX_OFFSET!() >= current) {
            if RAPIDLZ_READ32BIT!(curSrc) == RAPIDLZ_READ32BIT!(match) {
                token = curDest;
                curDest += 1;
                *token = 0;
                offset = current - matchOffset;
                goto _OFFSET_AND_MATCH;
            }
        }
        forwardHashValue = RapidlzHash4CalcValue(curSrc + 1);
        curSrc += 1;
    }

    return RapidlzEncLastLiterals(curSrcAnchor.cast(), srcEnd.cast(), curDest.cast(), destEnd.cast(), dest.cast());
}
