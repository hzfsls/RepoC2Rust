pub fn BzpFlushbuf(mut outData: Ptr<BzpOutComdata>) {
    while outData.nBuf > 0 {
        let tmp = outData.num;
        outData.out[tmp] = (outData.buf >> BZP_BITS24!()).cast::<u8>();
        outData.num += 1;
        outData.nBuf -= BZP_BITS8!();
        outData.buf <<= BZP_BITS8!();
    }
}
