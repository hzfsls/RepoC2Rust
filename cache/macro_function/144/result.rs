macro_rules! SAFE_COPY_MATCH { ($dstCurr:expr, $matchSrc:expr, $matchLength:expr) =>
    {
        while $matchLength > 0
        {
            *$dstCurr.plus_plus() = *$matchSrc.plus_plus();
            $matchLength -= 1;
        }
    }
}
pub(crate) use SAFE_COPY_MATCH;
