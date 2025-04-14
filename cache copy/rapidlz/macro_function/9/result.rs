macro_rules! SAFE_COPY_MATCH { ($dstCurr:expr, $matchSrc:expr, $matchLength:expr) =>
    {
        let mut length = $matchLength;
        while length > 0 {
            *$dstCurr = *$matchSrc;
            $dstCurr.plus_plus();
            $matchSrc.plus_plus();
            length -= 1;
        }
    }
}
pub(crate) use SAFE_COPY_MATCH;
