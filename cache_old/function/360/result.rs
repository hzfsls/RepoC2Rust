pub fn CmptLzPropsDecode(mut protData: Ptr<u8>, mut protSize: u32, mut decProt: Ptr<CmptLzDecProt>) -> i32 {
    let mut dictSize: u32 = Default::default();

    if protSize < CMPTLZ_PROPS_SIZE!() {
        return CMPT_ERROR_UNSUPPORTED!();
    } else {
        dictSize = (protData[1] | ((protData[2] as u32) << 8) | ((protData[3] as u32) << 16) | ((protData[4] as u32) << 24)).cast();
    }

    if dictSize < CMPTLZ_DICT_MIN_LEN!() {
        dictSize = CMPTLZ_DICT_MIN_LEN!();
    }
    decProt.dicSize = dictSize.cast();

    let mut firstData: u8 = protData[0].cast();
    if firstData >= (CMPTLZ_LIT_CTX_MAX!() * CMPTLZ_POS_STATE_MAX!() * CMPTLZ_LIT_POS_MAX!()) {
        return CMPT_ERROR_UNSUPPORTED!();
    }

    decProt.litCtx = (firstData % CMPTLZ_LIT_CTX_MAX!()).cast();
    firstData /= CMPTLZ_LIT_CTX_MAX!();
    decProt.posBits = (firstData / CMPTLZ_POS_STATE_MAX!()).cast();
    decProt.litPos = (firstData % CMPTLZ_LIT_POS_MAX!()).cast();

    return CMPT_OK!();
}
