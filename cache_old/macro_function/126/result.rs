macro_rules! RAPIDLZ_SAFE_LIT_COPY {
    ($curSrc:expr, $leftSrcSize:expr, $curDest:expr, $destEnd:expr, $litLen:expr) => {
        if RAPIDLZ_UNLIKELY!($litLen > $leftSrcSize || memmove_s($curDest, $destEnd - $curDest, $curSrc, $litLen) != EOK) {
            RAPIDLZ_LOG!(RAPIDLZ_DST_SIZE_SMALL, cstr!("litLen:%u dstEnd - dst:%zu\n"), $litLen, $leftSrcSize);
            return RAPIDLZ_ERROR_OUTPUT;
        }
    }
}
pub(crate) use RAPIDLZ_SAFE_LIT_COPY;
