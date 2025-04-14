pub fn BzpQSortSingle(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut stack: Ptr<BzpQSortInfo>) {
    let mut tl: i32 = stack.tl.cast();
    let mut tr: i32 = stack.tr.cast();
    let mut value: i32 = BzpSelectMidVal(sortBlock.cast(), idx.cast(), tl.cast(), tr.cast()).cast();
    let mut lPos: i32 = tl.cast();
    let mut rPos: i32 = tr.cast();
    let mut ePos: i32 = tl.cast();

    while (ePos <= rPos).as_bool() {
        if (idx[sortBlock[ePos]] < value).as_bool() {
            BzpSwap2Elem(sortBlock.cast(), ePos.cast(), lPos.cast());
            ePos.suffix_plus_plus();
            lPos.suffix_plus_plus();
        } else if (idx[sortBlock[ePos]] == value).as_bool() {
            ePos.suffix_plus_plus();
        } else {
            while (rPos >= ePos).as_bool() && (idx[sortBlock[rPos]] > value).as_bool() {
                rPos.suffix_minus_minus();
            }
            if (rPos < ePos).as_bool() {
                break;
            }
            if (idx[sortBlock[rPos]] == value).as_bool() {
                BzpSwap2Elem(sortBlock.cast(), ePos.cast(), rPos.cast());
            } else if (lPos == ePos).as_bool() {
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

    if (lPos - tl > tr - rPos).as_bool() {
        stack.stackL[stack.cnt] = tl.cast();
        stack.stackR[stack.cnt] = (lPos - 1).cast();
        stack.cnt.suffix_plus_plus();
        stack.stackL[stack.cnt] = (rPos + 1).cast();
        stack.stackR[stack.cnt] = tr.cast();
        stack.cnt.suffix_plus_plus();
    } else {
        stack.stackL[stack.cnt] = (rPos + 1).cast();
        stack.stackR[stack.cnt] = tr.cast();
        stack.cnt.suffix_plus_plus();
        stack.stackL[stack.cnt] = tl.cast();
        stack.stackR[stack.cnt] = (lPos - 1).cast();
        stack.cnt.suffix_plus_plus();
    }
}
