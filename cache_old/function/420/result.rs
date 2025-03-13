pub fn CmptlzBt4Finder(mut mf: Ptr<CmptMfCtx>, mut matches: Ptr<CmptlzMatchPair>) -> u32 {
    let niceLen: u32 = mf.niceLen.cast();
    let cur: Ptr<u8> = (mf.srcStart + mf.readPos).cast::<Ptr<u8>>();
    let pos: u32 = (mf.readPos + mf.offset).cast();
    let mut temp: u32 = Default::default();
    let mut hash2Value: u32 = Default::default();
    let mut hash3Value: u32 = Default::default();
    let mut hashValue: u32 = Default::default();
    let mut longestLen: u32 = 1;
    let mut matchesCount: u32 = 0;
    CMPT_HASH_4_CALC!(mf, cur, temp, hash2Value, hash3Value, hashValue);

    let delta2: u32 = (pos - mf.hash[hash2Value]).cast();
    let delta3: u32 = (pos - mf.hash[CMPTLZ_FIX_3_HASH!() + hash3Value]).cast();
    let curMatch: u32 = mf.hash[CMPTLZ_FIX_4_HASH!() + hashValue].cast();
    CMPT_HASH_UPDATE!(mf, hash2Value, hash3Value, hashValue, pos);
    CMPT_HASH_FIND_2_BYTES!(mf, delta2, longestLen, matchesCount, cur, matches);
    CMPT_HASH_FIND_3_BYTES!(mf, delta2, delta3, longestLen, matchesCount, cur, matches);

    if matchesCount != 0 {
        longestLen = CmptMemCmpLenSafe(cur.cast(), (cur - delta2).cast(), longestLen.cast(), niceLen.cast()).cast();
        matches[matchesCount - 1].len = longestLen.cast();
        if longestLen == niceLen {
            CmptBtSkip(mf.cast(), niceLen.cast(), pos.cast(), cur.cast(), curMatch.cast());
            CMPT_MF_MOVE_POS!(mf);
            return matchesCount.cast();
        }
    }

    if longestLen < CMPT_MF_MATCH_3_BYTES!() {
        longestLen = CMPT_MF_MATCH_3_BYTES!();
    }
    matchesCount = (CmptBtFind(mf.cast(), curMatch.cast(), (matches + matchesCount).cast(), longestLen.cast()) - matches).cast::<u32>();

    CMPT_MF_MOVE_POS!(mf);
    return matchesCount.cast();
}
