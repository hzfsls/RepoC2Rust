pub fn CmptlzMatchSkiper(mut mf: Ptr<CmptMfCtx>, mut amount: u32) {
    mf.readAhead += amount;
    let mut pos: u32;
    let mut temp: u32;
    let mut hash2Value: u32;
    let mut hash3Value: u32;
    let mut hashValue: u32;
    let mut curMatch: u32;
    let niceLen: u32 = mf.niceLen;
    c_do!({
        let mut lenLimit: u32 = mf.srcLen - mf.readPos;
        if CMPTLZ_LIKELY!(niceLen <= lenLimit).as_bool() {
            lenLimit = niceLen;
        } else {
            mf.readPos += 1;
            continue;
        }
        let mut cur: Ptr<u8> = (mf.srcStart + mf.readPos).cast();
        pos = mf.readPos + mf.offset;
        CMPT_HASH_4_CALC!(mf, cur, temp, hash2Value, hash3Value, hashValue);
        curMatch = mf.hash[CMPTLZ_FIX_4_HASH!() + hashValue];
        CMPT_HASH_UPDATE!(mf, hash2Value, hash3Value, hashValue, pos);
        CmptBtSkip(mf.cast(), lenLimit.cast(), pos.cast(), cur.cast(), curMatch.cast());
        CMPT_MF_MOVE_POS!(mf);
    } while amount.prefix_minus_minus() != 0);
}
