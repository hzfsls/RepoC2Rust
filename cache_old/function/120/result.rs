pub fn BzpBinaryLiftingSort(mut bwt: Ptr<BzpBwtInfo>) {
    let mut ftab: Array<i32, { BZP_ASCII_SIZE!() }> = Default::default();
    c_memset_s!(ftab, c_sizeofval!(ftab), 0, c_sizeofval!(ftab)).cast::<Void>();
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        ftab[bwt.block[i]] += 1;
    });
    c_for!(let mut i: i32 = 1; i < BZP_ASCII_SIZE!(); i.suffix_plus_plus(); {
        ftab[i] += ftab[i - 1];
    });
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        let mut ch: i32 = bwt.block[i];
        ftab[ch] -= 1;
        bwt.sortBlock[ftab[ch]] = i;
    });
    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!(); i.suffix_plus_plus(); {
        bwt.isStartPos[ftab[i]] = 1;
    });
    let mut M: i32 = 1;
    let mut sortflag: bool = true;
    while M < bwt.nBlock && sortflag {
        let mut st: i32 = 0;
        sortflag = false;
        c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
            if bwt.isStartPos[i] {
                st = i;
            }
            let mut pos: i32 = bwt.sortBlock[i] - M;
            if pos < 0 {
                pos += bwt.nBlock;
            }
            bwt.idx[pos] = st;
        });
        let mut l: i32 = 0;
        let mut r: i32 = 1;
        while l < bwt.nBlock {
            while r < bwt.nBlock && bwt.isStartPos[r] != 1 {
                r += 1;
            }
            r -= 1;
            if l < r {
                sortflag = true;
                BzpQuickSort(bwt.sortBlock.cast(), bwt.idx.cast(), l, r);
                BzpUpdateflag(bwt.cast(), l, r);
            }
            l = r + 1;
            r = l + 1;
        }
        M <<= 1;
    }
}
