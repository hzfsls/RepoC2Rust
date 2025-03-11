pub fn BzpBuffToBlockRLC(mut bzpf: Ptr<BzpFile>, mut bwt: Ptr<BzpBwtInfo>, mut IsLastdata: bool) {
    while !BZP_BLOCK_FULL!(bwt) && !BZP_BUFF_READ_EMPTY!(bzpf) {
        let mut pos: i32 = bzpf.input.pos.cast();
        let mut ch: u8 = bzpf.input.buf[pos].cast();
        let mut lasch: u8 = bzpf.lasChar.cast();
        if ch != lasch || bzpf.num == BZP_RLC_NUM_UPPER_LIMIT!() {
            BzpAddCharToBlock(lasch.cast(), bzpf.num.cast(), bwt.cast());
            bzpf.lasChar = ch.cast();
            bzpf.num = 1;
        } else {
            bzpf.num.suffix_plus_plus();
        }
        bzpf.input.pos.suffix_plus_plus();
    }
    if IsLastdata && BZP_BUFF_READ_EMPTY!(bzpf) {
        BzpAddCharToBlock(bzpf.lasChar.cast(), bzpf.num.cast(), bwt.cast());
        bzpf.lasChar = BZP_ASCII_SIZE!().cast();
        bzpf.num = 0;
    }
}
