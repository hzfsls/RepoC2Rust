pub fn BzpMTFDeCode(mut inData: Ptr<InDeComdata>, mut huffman: Ptr<BzpHuffmanDecode>, mut debwt: Ptr<BzpBwtDecodeInfo>) -> i32 {
    debwt.nBlock = 0;
    let mut ch: u8;
    let mut ninUse: i32 = huffman.alphaSize - BZP_EXTRA_CHARS_NUM!();
    let mut eob: i32 = ninUse + 1;
    let mut val: i32 = BzpHuffmanDecodeStep(huffman, inData);
    while (val != eob) && (val != -1) {
        if (val == 0) || (val == 1) {
            let mut res: i32 = 0;
            let mut basenum: i32 = 1;
            while (val == 0) || (val == 1) {
                res = res + (val + 1) * basenum;
                basenum <<= 1;
                val = BzpHuffmanDecodeStep(huffman, inData);
            }
            c_for!(let mut j: i32 = 0; j < res; j.suffix_plus_plus(); {
                let tmp0 = debwt.nBlock.suffix_plus_plus();
                debwt.block[tmp0] = inData.list[0].cast();
            });
        } else {
            let mut pos: i32 = val - 1;
            ch = inData.list[pos].cast();
            let tmp0 = debwt.nBlock.suffix_plus_plus();
            debwt.block[tmp0] = ch;

            c_for!(let mut j: i32 = pos; j > 0; j.suffix_minus_minus(); {
                inData.list[j] = inData.list[j - 1];
            });
            inData.list[0] = ch.cast();
            val = BzpHuffmanDecodeStep(huffman, inData);
        }
    }
    if (val == -1) {
        return BZP_ERROR_DATA!();
    }
    return BZP_OK!();
}
