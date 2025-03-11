pub fn BzpWriteToArray(mut val: i32, mut n: i32, mut data: Ptr<BzpOutComdata>) {
    while data.nBuf >= BZP_BITS8!() {
        let tmp = data.num.suffix_plus_plus();
        data.out[tmp] = (data.buf >> BZP_BITS24!()).cast::<u8>();
        data.nBuf -= BZP_BITS8!();
        data.buf <<= BZP_BITS8!();
    }
    data.buf |= (val << (BZP_BITS32!() - n - data.nBuf)).cast::<u32>();
    data.nBuf += n;
}
