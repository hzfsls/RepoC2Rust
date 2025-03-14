#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct TagCmptLzEncCtx {
    pub level: i32,
    pub litCtx: i32,
    pub litPos: i32,
    pub posBits: i32,
    pub dicSize: u32,
    pub endMarker: i32,
    pub numFastBytes: u32,

    pub encNeedFinish: bool,
    pub nowpos64: u64,
    pub cmptlzResponse: u32,

    pub state: CmptlzState,
    pub litMarcov: LitMarcov,
    pub reps: Array<u32, { CMPTLZ_NUM_REPS!() }>,

    pub isRep: Array<CmptlzProb, { CMPTLZ_NUM_STATES!() }>,
    pub isRepG0: Array<CmptlzProb, { CMPTLZ_NUM_STATES!() }>,
    pub isRepG1: Array<CmptlzProb, { CMPTLZ_NUM_STATES!() }>,
    pub isRepG2: Array<CmptlzProb, { CMPTLZ_NUM_STATES!() }>,
    pub isMatch: Array<Array<CmptlzProb, { CMPTLZ_NUM_PB_STATES_MAX!() }>, { CMPTLZ_NUM_STATES!() }>,
    pub isRep0Long: Array<Array<CmptlzProb, { CMPTLZ_NUM_PB_STATES_MAX!() }>, { CMPTLZ_NUM_STATES!() }>,
    pub probDistSlot: Array<Array<CmptlzProb, { 1 << CMPTLZ_DIST_SLOT_BITS!() }>, { CMPTLZ_DIST_STATE_TOTAL!() }>,
    pub probDistSpecial: Array<CmptlzProb, { CMPT_DIST_LIMIT_2!() }>,
    pub probAlign: Array<CmptlzProb, { 1 << CMPTLZ_ALIGN_BITS!() }>,

    pub posMask: u32,
    pub pbMask: u64,
    pub lpMask: u64,

    pub rcCtx: Ptr<CmptRcCtx>,

    pub mfCtx: Ptr<CmptMfCtx>,
    pub matches: Array<CmptlzMatchPair, { CMPT_MF_LONGEST_MATCH!() + 1 }>,
    pub matchesCount: u32,
    pub longestMatchLen: u32,

    pub backRes: u32,
    pub lenRes: u32,
    pub optEndIndex: u32,
    pub optsCurIndex: u32,
    pub opts: Array<CmptlzOpt, { CMPT_DP_OPTMAX!() }>,

    pub matchLenEncoder: CmptLenEncoder,
    pub repLenEncoder: CmptLenEncoder,
    pub repLenPriceCount: i32,

    pub matchPriceCount: i32,
    pub priceRootTable: Array<u32, { CMPT_PRIICE_TABLE_SIZE!() }>,
    pub priceDistSlotTable: Array<Array<u32, { 1 << CMPTLZ_DIST_SLOT_BITS!() }>, { CMPTLZ_DIST_STATE_TOTAL!() }>,
    pub priceDistTable: Array<Array<u32, { 1 << 7 }>, { CMPTLZ_DIST_STATE_TOTAL!() }>,
    pub priceAlignTable: Array<u32, { 1 << CMPTLZ_ALIGN_BITS!() }>,
    pub distTableSize: u32,
}
