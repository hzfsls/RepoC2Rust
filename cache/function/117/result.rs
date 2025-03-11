pub fn BzpQSortSingle(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut stack: Ptr<BzpQSortInfo>) {
    let mut tl: i32 = stack.tl.cast();
    let mut tr: i32 = stack.tr.cast();
    let mut value: i32 = BzpSelectMidVal(sortBlock.cast(), idx.cast(), tl.cast(), tr.cast()).cast();
    let mut lPos: i32 = tl.cast();
    let mut rPos: i32 = tr.cast();
    let mut ePos: i32 = tl.cast();

    while ePos <= rPos {
        if idx[sortBlock[ePos]] < value {
            BzpSwap2Elem(sortBlock.cast(), ePos.cast(), lPos.cast());
            ePos.suffix_plus_plus();
            lPos.suffix_plus_plus();
        } else if idx[sortBlock[ePos]] == value {
            ePos.suffix_plus_plus();
        } else {
            while rPos >= ePos && idx[sortBlock[rPos]] > value {
                rPos.suffix_minus_minus();
            }
            if rPos < ePos {
                break;
            }
            if idx[sortBlock[rPos]] == value {
                BzpSwap2Elem(sortBlock.cast(), ePos.cast(), rPos.cast());
            } else if lPos == ePos {
                BzpSwap2Elem(sortBlock.cast(), ePos.cast(), rPos.cast());
                lPos.suffix_plus_plus();
            } else {
                BzpSwap3Elem(sortBlock.cast(), lPos.cast(), ePos.cast(), rPos.cast());
                lPos.suffix_plus_plus();
            }
            ePos.suffix_plus_plus();
            rPos.suffix_minus_minus();
        }
    }

    if lPos - tl > tr - rPos {
        let tmp = stack.cnt;
        stack.stackL[tmp] = tl.cast();
        let tmp = stack.cnt;
        stack.stackR[tmp] = (lPos - 1).cast();
        stack.cnt.suffix_plus_plus();
        let tmp = stack.cnt;
        stack.stackL[tmp] = (rPos + 1).cast();
        let tmp = stack.cnt;
        stack.stackR[tmp] = tr.cast();
        stack.cnt.suffix_plus_plus();
    } else {
        let tmp = stack.cnt;
        stack.stackL[tmp] = (rPos + 1).cast();
        let tmp = stack.cnt;
        stack.stackR[tmp] = tr.cast();
        stack.cnt.suffix_plus_plus();
        let tmp = stack.cnt;
        stack.stackL[tmp] = tl.cast();
        let tmp = stack.cnt;
        stack.stackR[tmp] = (lPos - 1).cast();
        stack.cnt.suffix_plus_plus();
    }
}
