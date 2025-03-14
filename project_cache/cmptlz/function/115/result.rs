pub fn CmptPriceDistWithLen(mut encCtx: Ptr<CmptLzEncCtx>, mut dist: u32, mut len: u32, mut posState: u32) -> u32 {
    let distState: u32 = CMPT_GET_DIST_STATE!(len);
    let mut price: u32 = Default::default();
    if (dist < 128).as_bool() {
        price = encCtx.priceDistTable[distState][dist];
    } else {
        let distSlot: u32 = PosSloter!(dist);
        price = encCtx.priceDistSlotTable[distState][distSlot] + encCtx.priceAlignTable[dist & ((1 << CMPTLZ_ALIGN_BITS!()) - 1)];
    }
    price += CmptPriceLen(c_ref!(encCtx.matchLenEncoder).cast(), len.cast(), posState.cast());
    return price.cast();
}
