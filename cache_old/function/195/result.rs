pub fn CmptLzDecDirectProcess(mut decCtx: Ptr<CmptLzDecCtx>, mut dicPosLimit: usize, mut bufLimit: Ptr<u8>) -> i32 {
    let mut decRes: u32;
    let mut pbMask: u32 = ((1 as u32) << decCtx.prop.posBits) - 1;
    let mut procPos: u32;
    let mut mkState: u32;
    let mut posState: u32;

    let mut range: u32 = decCtx.range;
    let mut rangeCode: u32 = decCtx.code;
    let mut rangeBound: u32 = 0;
    let mut probSlot: Ptr<CmptLzDecProb>;
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx.cast());

    c_do!({
        procPos = decCtx.processedPos;
        mkState = decCtx.state;
        posState = CMPTLZ_CALC_POS_STATE!(procPos, pbMask);
        probSlot = CmptLzGetIsMatchProb(probsMatrix.cast()) + posState + mkState;

        CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, decCtx.buf);
        if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound) {
            CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, decCtx.buf);
            decRes = CmptLzLitDec(decCtx.cast(), c_ref!(range).cast(), c_ref!(rangeCode).cast(), c_ref!(rangeBound).cast());
        } else {
            CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, decCtx.buf);

            probSlot = CmptLzGetIsRepProb(probsMatrix.cast()) + mkState;

            if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound) {
                CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
                CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, decCtx.buf);
                decRes = CmptLzMatchDec(decCtx.cast(), c_ref!(range).cast(), c_ref!(rangeCode).cast(), c_ref!(rangeBound).cast(), dicPosLimit.cast(), posState.cast());
            } else {
                CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
                CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, decCtx.buf);
                decRes = CmptLzRepDec(decCtx.cast(), c_ref!(range).cast(), c_ref!(rangeCode).cast(), c_ref!(rangeBound).cast(), dicPosLimit.cast(), posState.cast());
            }
            if decRes != CMPT_OK!() {
                break;
            }
        }
    } while decCtx.dictPos < dicPosLimit && decCtx.buf < bufLimit && decCtx.remainLen < CMPTLZ_MATCH_MAX_LEN!());

    decCtx.range = range;
    decCtx.code = rangeCode;

    return decRes.cast::<i32>();
}
