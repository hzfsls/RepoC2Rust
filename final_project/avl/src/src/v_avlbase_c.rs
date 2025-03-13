use crate::translation_utils::*;
pub use crate::src::v_avlbase_h::*;
pub use crate::include::v_avl_base_h::*;

pub fn VOS_V_AVLBaseInit(mut pscKey: Ptr<u8>) -> u32 {
    // (void)pscKey; is not needed in Rust
    return 0;
}


pub fn VOS_V_AVLBaseFini() -> u32 {
    return 0;
}


pub fn VosAvlRotateRight(mut ppstSubTree: Ptr<Ptr<AVLBASE_NODE_S>>) {
    let mut pstLeftSon: Ptr<AVLBASE_NODE_S> = (*ppstSubTree).pstLeft.cast();

    (*ppstSubTree).pstLeft = pstLeftSon.pstRight.cast();
    if ((*ppstSubTree).pstLeft != AVL_NULL_PTR!()).as_bool() {
        (*ppstSubTree).pstLeft.pstParent = (*ppstSubTree).cast();
    }

    (*ppstSubTree).sLHeight = pstLeftSon.sRHeight.cast();
    pstLeftSon.pstParent = (*ppstSubTree).pstParent.cast();
    pstLeftSon.pstRight = (*ppstSubTree).cast();
    pstLeftSon.pstRight.pstParent = pstLeftSon.cast();
    pstLeftSon.sRHeight = (1 + VOS_V2_AVL_MAX!((*ppstSubTree).sRHeight, (*ppstSubTree).sLHeight)).cast();

    *ppstSubTree = pstLeftSon.cast();

    return;
}


pub fn VosAvlRotateLeft(mut ppstSubTree: Ptr<Ptr<AVLBASE_NODE_S>>) {
    let mut pstRightSon: Ptr<AVLBASE_NODE_S> = (*ppstSubTree).pstRight.cast();

    (*ppstSubTree).pstRight = pstRightSon.pstLeft.cast();
    if ((*ppstSubTree).pstRight != AVL_NULL_PTR!()).as_bool() {
        (*ppstSubTree).pstRight.pstParent = (*ppstSubTree).cast();
    }

    (*ppstSubTree).sRHeight = pstRightSon.sLHeight.cast();
    pstRightSon.pstParent = (*ppstSubTree).pstParent.cast();
    pstRightSon.pstLeft = (*ppstSubTree).cast();
    pstRightSon.pstLeft.pstParent = pstRightSon.cast();
    pstRightSon.sLHeight = (1 + VOS_V2_AVL_MAX!((*ppstSubTree).sRHeight, (*ppstSubTree).sLHeight)).cast();

    *ppstSubTree = pstRightSon.cast();

    return;
}


pub fn VosAvlUpdateSwapNode(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstSwapNode: Ptr<AVLBASE_NODE_S>, mut pstBaseNode: Ptr<AVLBASE_NODE_S>) {
    pstSwapNode.pstParent = pstBaseNode.pstParent.cast();
    pstSwapNode.pstRight = pstBaseNode.pstRight.cast();
    pstSwapNode.pstLeft = pstBaseNode.pstLeft.cast();
    pstSwapNode.sRHeight = pstBaseNode.sRHeight.cast();
    pstSwapNode.sLHeight = pstBaseNode.sLHeight.cast();
    pstSwapNode.pstRight.pstParent = pstSwapNode.cast();
    pstSwapNode.pstLeft.pstParent = pstSwapNode.cast();

    if (pstBaseNode.pstParent == AVL_NULL_PTR!()).as_bool() {
        pstTree.pstRoot = pstSwapNode.cast();
    } else if (pstBaseNode.pstParent.pstRight == pstBaseNode).as_bool() {
        pstSwapNode.pstParent.pstRight = pstSwapNode.cast();
    } else {
        pstSwapNode.pstParent.pstLeft = pstSwapNode.cast();
    }
}


pub fn VosAvlMoveNodeToNewPos(mut pstNode: Ptr<AVLBASE_NODE_S>, mut pstNewParent: Ptr<AVLBASE_NODE_S>, mut pstNewLeftSon: Ptr<AVLBASE_NODE_S>, mut pstNewRightSon: Ptr<AVLBASE_NODE_S>) {
    pstNode.pstParent = pstNewParent.cast();
    pstNode.pstLeft = pstNewLeftSon.cast();
    pstNode.pstRight = pstNewRightSon.cast();
    pstNode.sLHeight = 0;
    pstNode.sRHeight = 0;

    if (pstNewLeftSon != AVL_NULL_PTR!()).as_bool() {
        pstNode.pstLeft.pstParent = pstNode.cast();
        pstNode.sLHeight = 1;
    }

    if (pstNewRightSon != AVL_NULL_PTR!()).as_bool() {
        pstNode.pstRight.pstParent = pstNode.cast();
        pstNode.sRHeight = 1;
    }
}


pub fn VosAvlSwapRightMost(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstSubTree: Ptr<AVLBASE_NODE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) {
    let mut pstSwapNode: Ptr<AVLBASE_NODE_S> = pstSubTree.cast();
    let mut pstSwapParent: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut pstSwapLeft: Ptr<AVLBASE_NODE_S> = Default::default();

    FIND_RIGHTMOST_NODE!(pstSwapNode);

    if (pstSwapNode.sRHeight != 0).as_bool() || (pstSwapNode.sLHeight > 1).as_bool() {
        return;
    }

    pstSwapParent = pstSwapNode.pstParent.cast();
    pstSwapLeft = pstSwapNode.pstLeft.cast();

    VosAvlUpdateSwapNode(pstTree.cast(), pstSwapNode.cast(), pstNode.cast());
    VosAvlMoveNodeToNewPos(pstNode.cast(), pstSwapParent.cast(), pstSwapLeft.cast(), AVL_NULL_PTR!());

    pstNode.pstParent.pstRight = pstNode.cast();

    return;
}


pub fn VosAvlSwapLeftMost(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstSubTree: Ptr<AVLBASE_NODE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) {
    let mut pstSwapNode: Ptr<AVLBASE_NODE_S> = pstSubTree.cast();
    let mut pstSwapParent: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut pstSwapRight: Ptr<AVLBASE_NODE_S> = Default::default();

    FIND_LEFTMOST_NODE!(pstSwapNode);

    if (pstSwapNode.sLHeight != 0).as_bool() || (pstSwapNode.sRHeight > 1).as_bool() {
        return;
    }

    pstSwapParent = pstSwapNode.pstParent.cast();
    pstSwapRight = pstSwapNode.pstRight.cast();

    VosAvlUpdateSwapNode(pstTree.cast(), pstSwapNode.cast(), pstNode.cast());
    VosAvlMoveNodeToNewPos(pstNode.cast(), pstSwapParent.cast(), AVL_NULL_PTR!(), pstSwapRight.cast());

    pstNode.pstParent.pstLeft = pstNode.cast();

    return;
}


pub fn VosAvlRebalance(mut ppstSubTree: Ptr<Ptr<AVLBASE_NODE_S>>) {
    let mut iMoment: i32 = Default::default();

    iMoment = ((*ppstSubTree).sRHeight - (*ppstSubTree).sLHeight).cast();

    if (iMoment > 1).as_bool() {
        if ((*ppstSubTree).pstRight.sLHeight > (*ppstSubTree).pstRight.sRHeight).as_bool() {
            VosAvlRotateRight(c_ref!((*ppstSubTree).pstRight).cast());
        }

        VosAvlRotateLeft(ppstSubTree.cast());
    } else if (iMoment < -1).as_bool() {
        if ((*ppstSubTree).pstLeft.sRHeight > (*ppstSubTree).pstLeft.sLHeight).as_bool() {
            VosAvlRotateLeft(c_ref!((*ppstSubTree).pstLeft).cast());
        }

        VosAvlRotateRight(ppstSubTree.cast());
    }

    return;
}


pub fn VosAvlBalanceTree(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) {
    let mut pstNodeTmp: Ptr<AVLBASE_NODE_S> = pstNode.cast();
    while (pstNodeTmp.pstParent != AVL_NULL_PTR!()).as_bool() {
        if (pstNodeTmp.pstParent.pstRight == pstNodeTmp).as_bool() {
            pstNodeTmp = pstNodeTmp.pstParent.cast();
            VosAvlRebalance(c_ref!(pstNodeTmp.pstRight).cast());
            pstNodeTmp.sRHeight = (1 + VOS_V2_AVL_MAX!(pstNodeTmp.pstRight.sRHeight, pstNodeTmp.pstRight.sLHeight)).cast();
        } else {
            pstNodeTmp = pstNodeTmp.pstParent.cast();
            VosAvlRebalance(c_ref!(pstNodeTmp.pstLeft).cast());
            pstNodeTmp.sLHeight = (1 + VOS_V2_AVL_MAX!(pstNodeTmp.pstLeft.sRHeight, pstNodeTmp.pstLeft.sLHeight)).cast();
        }
    }
    if (pstNodeTmp.sLHeight != pstNodeTmp.sRHeight).as_bool() {
        VosAvlRebalance(c_ref!(pstTree.pstRoot).cast());
    }
    return;
}


pub fn VosAVLSearchReplaceNodeInRTree(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();

    if (pstNode.pstRight.pstLeft == AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode = pstNode.pstRight.cast();
        pstReplaceNode.pstLeft = pstNode.pstLeft.cast();
        pstReplaceNode.pstLeft.pstParent = pstReplaceNode.cast();
        pstReplaceNode.sLHeight = pstNode.sLHeight.cast();
    } else {
        VosAvlSwapLeftMost(pstTree.cast(), pstNode.pstRight.cast(), pstNode.cast());
        pstReplaceNode = pstNode.pstRight.cast();
    }

    return pstReplaceNode.cast();
}


pub fn VosAvlSearchReplaceNodeInLTree(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();

    if (pstNode.pstLeft.pstRight == AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode = pstNode.pstLeft.cast();
        pstReplaceNode.pstRight = pstNode.pstRight.cast();
        pstReplaceNode.pstRight.pstParent = pstReplaceNode.cast();
        pstReplaceNode.sRHeight = pstNode.sRHeight.cast();
    } else {
        VosAvlSwapRightMost(pstTree.cast(), pstNode.pstLeft.cast(), pstNode.cast());
        pstReplaceNode = pstNode.pstLeft.cast();
    }

    return pstReplaceNode.cast();
}


pub fn VosAvlSearchReplaceNode(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();

    if (pstNode.sRHeight > pstNode.sLHeight).as_bool() {
        pstReplaceNode = VosAVLSearchReplaceNodeInRTree(pstTree.cast(), pstNode.cast());
    } else {
        pstReplaceNode = VosAvlSearchReplaceNodeInLTree(pstTree.cast(), pstNode.cast());
    }

    return pstReplaceNode.cast();
}


pub fn VosAvlDeleteCheck(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();

    if (pstNode.pstLeft == AVL_NULL_PTR!()).as_bool() && (pstNode.pstRight == AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode = AVL_NULL_PTR!();

        if (pstTree.pstFirst == pstNode).as_bool() {
            pstTree.pstFirst = pstNode.pstParent.cast();
        }

        if (pstTree.pstLast == pstNode).as_bool() {
            pstTree.pstLast = pstNode.pstParent.cast();
        }
    } else if (pstNode.pstLeft == AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode = pstNode.pstRight.cast();

        if (pstTree.pstFirst == pstNode).as_bool() {
            pstTree.pstFirst = pstReplaceNode.cast();
        }
    } else if (pstNode.pstRight == AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode = pstNode.pstLeft.cast();

        if (pstTree.pstLast == pstNode).as_bool() {
            pstTree.pstLast = pstReplaceNode.cast();
        }
    } else {
        pstReplaceNode = VosAvlSearchReplaceNode(pstTree.cast(), pstNode.cast()).cast();
    }
    return pstReplaceNode.cast();
}


pub fn VosAvlDelete(mut pstBaseNode: Ptr<AVLBASE_NODE_S>, mut pstBaseTree: Ptr<AVLBASE_TREE_S>) {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut pstParentNode: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut sNewHeight: i16 = 0;

    pstReplaceNode = VosAvlDeleteCheck(pstBaseTree.cast(), pstBaseNode.cast()).cast();

    pstParentNode = pstBaseNode.pstParent.cast();

    pstBaseNode.pstParent = AVL_NULL_PTR!();
    pstBaseNode.pstRight = AVL_NULL_PTR!();
    pstBaseNode.pstLeft = AVL_NULL_PTR!();
    pstBaseNode.sRHeight = -1;
    pstBaseNode.sLHeight = -1;

    if (pstReplaceNode != AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode.pstParent = pstParentNode.cast();
        sNewHeight = (1 + VOS_V2_AVL_MAX!(pstReplaceNode.sLHeight, pstReplaceNode.sRHeight)).cast();
    }

    if (pstParentNode != AVL_NULL_PTR!()).as_bool() {
        if (pstParentNode.pstRight == pstBaseNode).as_bool() {
            pstParentNode.pstRight = pstReplaceNode.cast();
            pstParentNode.sRHeight = sNewHeight.cast();
        } else {
            pstParentNode.pstLeft = pstReplaceNode.cast();
            pstParentNode.sLHeight = sNewHeight.cast();
        }

        VosAvlBalanceTree(pstBaseTree.cast(), pstParentNode.cast());
    } else {
        pstBaseTree.pstRoot = pstReplaceNode.cast();
    }

    return;
}


