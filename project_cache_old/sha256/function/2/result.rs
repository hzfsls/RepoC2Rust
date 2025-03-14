pub fn vosSha256LastPadding(mut pucData: Ptr<u8>, mut uiLen: u32, mut pstCtx: Ptr<VOS_SHA256_CTX>, mut puiPaddingLen: Ptr<u32>) -> u32 {
    let mut err: errno_t = Default::default();
    let mut uiBlcLen: u32 = pstCtx.blocklen.cast();
    let mut pucBlock: Ptr<u8> = pstCtx.block.cast::<Ptr<u8>>();

    if (uiLen >= SHA256_BLOCK_SIZE!()).as_bool() || (uiLen + uiBlcLen >= SHA256_BLOCK_SIZE!()).as_bool() {
        err = c_memcpy_s!(pucBlock + uiBlcLen, SHA256_BLOCK_SIZE!() - uiBlcLen, pucData, SHA256_BLOCK_SIZE!() - uiBlcLen);
        if (err != EOK!()).as_bool() {
            pstCtx.corrupted = 1;
            return SHA256_ERROR!();
        }
        vosSha256CompressMul(pstCtx.cast(), pucBlock.cast(), 1);
        *puiPaddingLen = (SHA256_BLOCK_SIZE!() - uiBlcLen).cast();
        pstCtx.blocklen = 0;
        c_memset_s!(pucBlock, SHA256_BLOCK_SIZE!(), 0, SHA256_BLOCK_SIZE!()).cast::<Void>();
    } else {
        err = c_memcpy_s!(pucBlock + uiBlcLen, SHA256_BLOCK_SIZE!() - uiBlcLen, pucData, uiLen);
        if (err != EOK!()).as_bool() {
            pstCtx.corrupted = 1;
            return SHA256_ERROR!();
        }
        pstCtx.blocklen += uiLen.cast();
        return SHA256_ERROR!();
    }

    return SHA256_OK!();
}
