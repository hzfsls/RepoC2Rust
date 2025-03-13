pub fn VOS_AVL3_Prev(mut pstNode: Ptr<AVL3_NODE>, mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) -> Ptr<Void> {
    let mut pstNodeTmp: Ptr<AVL3_NODE> = pstNode.cast();
    if pstNodeTmp == AVL_NULL_PTR!() || pstTreeInfo == AVL_NULL_PTR!() {
        return AVL_NULL_PTR!();
    }

    if pstNodeTmp.pstLeft != AVL_NULL_PTR!() {
        pstNodeTmp = pstNodeTmp.pstLeft.cast();
        FIND_RIGHTMOST_NODE!(pstNodeTmp);
    } else {
        while pstNodeTmp != AVL_NULL_PTR!() {
            if pstNodeTmp.pstParent == AVL_NULL_PTR!() || pstNodeTmp.pstParent.pstRight == pstNodeTmp {
                pstNodeTmp = pstNodeTmp.pstParent.cast();
                break;
            }
            pstNodeTmp = pstNodeTmp.pstParent.cast();
        }
    }

    return GET_NODE_START_ADDRESS!(pstNodeTmp, pstTreeInfo.usNodeOffset);
}
