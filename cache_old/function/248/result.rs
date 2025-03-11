pub fn CmptRcFlushData(mut rcCtx: Ptr<CmptRcCtx>) -> i32 {
    let mut i: i32 = Default::default();
    let mut res: i32 = Default::default();
    c_for!(let mut i = 0; i < 5; i.suffix_plus_plus(); {
        res = CmptRcShiftLow(rcCtx.cast()).cast();
        if res != CMPT_OK!() {
            break;
        }
    });
    return res.cast();
}
