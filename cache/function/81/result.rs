pub fn BzpReadBits(mut nBit: i32, mut inData: Ptr<InDeComdata>) -> u32 {
    let mut res: u32 = 0;

    while inData.nBuf < nBit {
        if inData.input.nBuf == inData.input.pos {
            inData.input.nBuf = c_fread!(inData.input.buf, c_sizeof!(char), c_sizeofval!(inData.input.buf), inData.input.filePtr);
            inData.input.pos = 0;
        }
        let tmp = inData.input.pos;
        let mut data: i32 = (inData.input.buf[tmp]).cast::<u32>().cast();

        inData.buf = (inData.buf << BZP_BITS8!()) | data.cast::<u32>();
        inData.input.pos.suffix_plus_plus();
        inData.nBuf += BZP_BITS8!();
    }
    res = inData.buf >> (inData.nBuf - nBit);
    res = res & ((1 << nBit) - 1);
    inData.nBuf -= nBit;
    return res.cast();
}
