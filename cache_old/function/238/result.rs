pub fn CmptlzMatchFinder(mut mf: Ptr<CmptMfCtx>, mut pCount: Ptr<u32>, mut matches: Ptr<CmptlzMatchPair>) -> u32 {
    if CMPTLZ_UNLIKELY!(mf.srcLen - mf.readPos < mf.niceLen) {
        *pCount = 0;
        mf.readPos.suffix_plus_plus();
        mf.readAhead.suffix_plus_plus();
        return 0;
    }
    let mut count: u32 = CmptlzBt4Finder(mf.cast(), matches.cast()).cast();
    if count == 0 {
        *pCount = 0;
        mf.readAhead.suffix_plus_plus();
        return 0;
    }
    let mut longestLen: u32 = matches[count - 1].len.cast();
    if longestLen == mf.niceLen {
        let mut bytesAvail: u32 = CMPTLZ_FIND_MIN!(mf.srcLen - mf.readPos + 1, CMPT_MF_LONGEST_MATCH!()).cast();
        let mut p1: Ptr<u8> = (mf.srcStart + mf.readPos - 1).cast::<Ptr<u8>>();
        let mut p2: Ptr<u8> = (p1 - matches[count - 1].dist - 1).cast::<Ptr<u8>>();
        longestLen = CmptMemCmpLenSafe(p1.cast(), p2.cast(), longestLen.cast(), bytesAvail.cast()).cast();
    }
    *pCount = count.cast();
    mf.readAhead.suffix_plus_plus();
    return longestLen.cast();
}
