pub fn RapidlzDecWithExternalDict(mut src: Ptr<u8>, mut dest: Ptr<u8>, mut srcSize: i32, mut outBufferSize: i32, mut dictStart: Ptr<u8>, mut dictSize: i32) -> i32 {
    let mut curSrc: Ptr<u8> = src.cast();
    let mut srcEnd: Ptr<u8> = (curSrc + srcSize).cast();
    let mut curDest: Ptr<u8> = dest.cast();
    let mut destEnd: Ptr<u8> = (curDest + outBufferSize).cast();
    let mut srcEndFast: Ptr<u8> = (srcEnd - RAPIDLZ_COPY_PROTECT_SIZE!()).cast();
    let mut destEndFast: Ptr<u8> = (destEnd - RAPIDLZ_COPY_PROTECT_SIZE!()).cast();
    let mut dictEnd: Ptr<u8> = (dictStart + dictSize).cast();

    let mut token: u32 = Default::default();
    let mut len: u32 = Default::default();
    let mut offset: u16 = Default::default();
    let mut matchSrc: Ptr<u8> = Default::default();
    let mut temp: u32 = Default::default();
    let mut leftSrcSize: usize = Default::default();
    loop {
        token = (*curSrc).cast();
        curSrc += 1;

        len = token >> 4;
        if RAPIDLZ_LIKELY!(len < RAPIDLZ_MAX_4BIT_VALUE!()).as_bool() {
            if RAPIDLZ_LIKELY!(RAPIDLZ_DICT_FAST_COPY_AVAIL!(curSrc, len, srcEndFast, curDest, destEndFast)).as_bool() {
                RapidlzCopy16Byte(curDest.cast(), curSrc.cast());
                RAPIDLZ_POSITION_UPDATE!(curSrc, curDest, len);
            } else {
                leftSrcSize = (srcEnd - curSrc).cast();
                RAPIDLZ_SAFE_COPY_TILL_END!(curSrc, leftSrcSize, curDest, destEnd, len);
            }
        } else {
            RAPIDLZ_READ_OPTIONAL_LENGTH!(len, curSrc, srcEnd, temp);
            if RAPIDLZ_LIKELY!(RAPIDLZ_DICT_FAST_COPY_AVAIL!(curSrc, len, srcEndFast, curDest, destEndFast)).as_bool() {
                RapidlzWildCopy16(curSrc.cast(), curDest.cast(), (curDest + len).cast());
                RAPIDLZ_POSITION_UPDATE!(curSrc, curDest, len);
            } else {
                leftSrcSize = (srcEnd - curSrc).cast();
                RAPIDLZ_SAFE_COPY_TILL_END!(curSrc, leftSrcSize, curDest, destEnd, len);
            }
        }

        offset = RapidlzReadLE16Bit(curSrc.cast());
        curSrc += 2;
        matchSrc = (curDest - offset).cast();

        len = token & RAPIDLZ_MAX_4BIT_VALUE!();

        RAPIDLZ_GET_MATCH_LEN!(len, curSrc, srcEnd, temp);

        #[cfg(RAPIDLZ_DEBUG)]
        RAPIDLZ_RETURN_IF_NOT_TRUE!(!(curDest + len > destEnd - RAPIDLZ_LAST_LITERALS!()), RAPIDLZ_DEC_NOT_OK!());

        if (matchSrc >= dest.cast()).as_bool() {
            if RAPIDLZ_LIKELY!((curDest + len) <= (destEndFast - RAPIDLZ_COPY_PROTECT_SIZE!() + RAPIDLZ_LAST_LITERALS!()).as_bool() {
                RapidlzCopyMatchFast(curDest.cast(), matchSrc.cast(), offset.cast(), len.cast());
                curDest += len;
            } else {
                if RAPIDLZ_LIKELY!(len < 1024).as_bool() {
                    RAPIDLZ_FAST_SAFE_COPY_BY_BYTES!(curDest, matchSrc, len);
                } else {
                    RapidlzSafeCopyMatchFast(curDest.cast(), matchSrc.cast(), destEnd.cast(), offset.cast(), len.cast());
                    curDest += len;
                }
            }
        } else {
            let mut err: errno_t = Default::default();
            if (len <= (dest - matchSrc).cast()).as_bool() {
                err = c_memmove_s!(curDest, (destEnd - curDest).cast(), (dictEnd - (dest - matchSrc)).cast(), len.cast());
                curDest += len;
            } else {
                let mut externCopySize: usize = (dest - matchSrc).cast();
                let mut innerCopySize: usize = (len - externCopySize).cast();
                err = c_memcpy_s!(curDest, (destEnd - curDest).cast(), (dictEnd - externCopySize).cast(), externCopySize.cast());
                curDest += externCopySize;
                if (innerCopySize > (curDest - dest).cast()).as_bool() {
                    let mut copySrc: Ptr<u8> = dest.cast();
                    while (innerCopySize != 0).as_bool() {
                        *curDest = *copySrc;
                        curDest += 1;
                        copySrc += 1;
                        innerCopySize -= 1;
                    }
                } else {
                    err = c_memcpy_s!(curDest, (destEnd - curDest).cast(), dest.cast(), innerCopySize.cast());
                    curDest += innerCopySize;
                }
            }
            #[cfg(RAPIDLZ_DEBUG)]
            RAPIDLZ_RETURN_IF_NOT_EOK!(err, RAPIDLZ_DEC_NOT_OK!());
            #[cfg(not(RAPIDLZ_DEBUG))]
            let _ = err;
        }
    }

    return (curDest - dest).cast::<i32>();
}
