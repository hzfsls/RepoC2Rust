pub fn CmptLzDecAllocate(mut decCtx: Ptr<CmptLzDecCtx>, mut protData: Ptr<u8>, mut protSize: u32, mut memHook: Ptr<CmptLzMemHook>) -> i32 {
    let mut res: i32;
    let mut dictMask: u32;
    let mut dictBufSize: usize;
    let mut decProt: CmptLzDecProt = Default::default();

    if decCtx == NULL!() || protData == NULL!() || memHook == NULL!() {
        return CMPT_ERROR_UNSUPPORTED!();
    }

    res = CmptLzPropsDecode(protData.cast(), protSize.cast(), c_ref!(decProt).cast()).cast();
    if res != CMPT_OK!() {
        return res;
    }
    res = CmptLzDecAllocateProbs(decCtx.cast(), c_ref!(decProt).cast(), memHook.cast()).cast();
    if res != CMPT_OK!() {
        return res;
    }

    let mut dictSize: u32 = decProt.dicSize;
    if dictSize >= (1 << CMPTLZ_BIG_DICT_LG_SIZE!()) {
        dictMask = (1 << CMPTLZ_MID_DICT_LG_SIZE!()) - 1;
    } else if dictSize >= (1 << CMPTLZ_MID_DICT_LG_SIZE!()) {
        dictMask = (1 << CMPTLZ_SMALL_DICT_LG_SIZE!()) - 1;
    } else {
        dictMask = CMPTLZ_DICT_MIN_LEN!() - 1;
    }

    dictBufSize = ((dictSize + dictMask) & !dictMask).cast::<usize>();
    if dictBufSize < dictSize.cast() {
        dictBufSize = dictSize.cast();
    }

    if decCtx.dict == NULL!() {
        decCtx.dict = CmptLzDecMemAlloc(memHook.cast(), CMPTLZ_DICT_HANDLE!(), dictBufSize).cast();
    } else {
        if dictBufSize != decCtx.dictBufSize {
            CmptLzFreeDict(decCtx.cast(), memHook.cast());
            decCtx.dict = CmptLzDecMemAlloc(memHook.cast(), CMPTLZ_DICT_HANDLE!(), dictBufSize).cast();
        }
    }

    if decCtx.dict == NULL!() {
        CmptLzDecFreeProbs(decCtx.cast(), memHook.cast());
        return CMPT_ERROR_MEM!();
    }

    decCtx.dictBufSize = dictBufSize;
    decCtx.prop = decProt;

    return CMPT_OK!();
}
