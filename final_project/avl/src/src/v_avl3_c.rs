use crate::translation_utils::*;
pub use crate::include::v_avl3_h::*;
pub use crate::src::v_avl3_inner_h::*;
pub use crate::src::v_avlbase_h::*;

pub fn VOS_V_AVL3Init(mut pscKey: Ptr<u8>) -> u32 {
    (pscKey).cast::<Void>();
    return 0;
}


pub fn VOS_V_AVL3Fini() -> u32 {
    return 0;
}


pub fn AVL3_Find_Or_Find_Next(mut pstTree: Ptr<AVL3_TREE>, mut pKey: Ptr<Void>, mut bFlag: u32, mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL3_NODE> = Default::default();
    let mut pFoundNode: Ptr<Void> = AVL_NULL_PTR!();
    let mut iResult: i32 = Default::default();
    let mut iKeyOffset: i32 = Default::default();

    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo).as_bool() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot.cast();
    if (pstNode == AVL_NULL_PTR!()).as_bool() {
        return AVL_NULL_PTR!();
    }

    iKeyOffset = GET_KEYOFFSET!(pstTreeInfo).cast();

    loop {
        iResult = (pstTreeInfo.pfCompare)(pKey.cast(), (pstNode.cast::<Ptr<u8>>() + iKeyOffset).cast::<Ptr<Void>>()).cast();
        if iResult > 0 {
            if (pstNode.pstRight == AVL_NULL_PTR!()).as_bool() {
                pFoundNode = VOS_AVL3_Next(pstNode.cast(), pstTreeInfo.cast()).cast();
                break;
            }
            pstNode = pstNode.pstRight.cast();
        } else if iResult < 0 {
            if (pstNode.pstLeft == AVL_NULL_PTR!()).as_bool() {
                pFoundNode = (pstNode.cast::<Ptr<u8>>() - pstTreeInfo.usNodeOffset).cast::<Ptr<Void>>();
                break;
            }
            pstNode = pstNode.pstLeft.cast();
        } else {
            if bFlag != 0 {
                pFoundNode = VOS_AVL3_Next(pstNode.cast(), pstTreeInfo.cast()).cast();
            } else {
                pFoundNode = (pstNode.cast::<Ptr<u8>>() - pstTreeInfo.usNodeOffset).cast::<Ptr<Void>>();
            }
            break;
        }
    }

    return pFoundNode.cast();
}


pub fn VOS_AVL3_Insert_Or_Find(mut pstTree: Ptr<AVL3_TREE>, mut pstNode: Ptr<AVL3_NODE>, mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) -> Ptr<Void> {
    let mut pstParentNode: Ptr<AVL3_NODE> = Default::default();
    let mut iResult: i32 = Default::default();
    let mut iKeyOffset: i32 = Default::default();

    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo).as_bool() || (pstNode == AVL_NULL_PTR!()).as_bool() {
        return AVL_NULL_PTR!();
    }

    pstNode.sRHeight = 0;
    pstNode.sLHeight = 0;

    if (pstTree.pstRoot == AVL_NULL_PTR!()).as_bool() {
        pstTree.pstRoot = pstNode.cast();
        pstTree.pstFirst = pstNode.cast();
        pstTree.pstLast = pstNode.cast();
        return AVL_NULL_PTR!();
    }

    pstParentNode = pstTree.pstRoot.cast();

    iKeyOffset = GET_KEYOFFSET!(pstTreeInfo).cast();
    while (pstParentNode != AVL_NULL_PTR!()).as_bool() {
        iResult = (pstTreeInfo.pfCompare)((c_ref!(pstNode).cast::<Ptr<u8>>() + iKeyOffset).cast::<Ptr<Void>>(),
                                         (c_ref!(pstParentNode).cast::<Ptr<u8>>() + iKeyOffset).cast::<Ptr<Void>>()).cast();
        if iResult > 0 {
            if (pstParentNode.pstRight != AVL_NULL_PTR!()).as_bool() {
                pstParentNode = pstParentNode.pstRight.cast();
                continue;
            }
            VosAvlNodeRightInsert(pstTree.cast::<Ptr<AVLBASE_TREE_S>>(), pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>(), pstNode.cast::<Ptr<AVLBASE_NODE_S>>());
        } else if iResult < 0 {
            if (pstParentNode.pstLeft != AVL_NULL_PTR!()).as_bool() {
                pstParentNode = pstParentNode.pstLeft.cast();
                continue;
            }
            VosAvlNodeLeftInsert(pstTree.cast::<Ptr<AVLBASE_TREE_S>>(), pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>(), pstNode.cast::<Ptr<AVLBASE_NODE_S>>());
        } else {
            pstNode.sRHeight = -1;
            pstNode.sLHeight = -1;
            return (c_ref!(pstParentNode).cast::<Ptr<u8>>() - pstTreeInfo.usNodeOffset).cast::<Ptr<Void>>();
        }
        break;
    }

    VosAvlBalanceTree(pstTree.cast::<Ptr<AVLBASE_TREE_S>>(), pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>());

    return AVL_NULL_PTR!();
}


pub fn VOS_AVL3_Delete(mut pstTree: Ptr<AVL3_TREE>, mut pstNode: Ptr<AVL3_NODE>) {
    let mut pstBaseNode: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut pstBaseTree: Ptr<AVLBASE_TREE_S> = Default::default();
    if (pstTree == AVL_NULL_PTR!()).as_bool() || (pstNode == AVL_NULL_PTR!()).as_bool() {
        return;
    }
    pstBaseNode = pstNode.cast::<Ptr<AVLBASE_NODE_S>>();
    pstBaseTree = pstTree.cast::<Ptr<AVLBASE_TREE_S>>();
    VosAvlDelete(pstBaseNode.cast(), pstBaseTree.cast());
}


pub fn VOS_AVL3_Find(mut pstTree: Ptr<AVL3_TREE>, mut pstKey: Ptr<Void>, mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL3_NODE> = Default::default();
    let mut iResult: i32 = Default::default();
    let mut iKeyOffset: i32 = Default::default();

    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo).as_bool() {
        return AVL_NULL_PTR!();
    }

    pstNode = pstTree.pstRoot.cast();
    iKeyOffset = GET_KEYOFFSET!(pstTreeInfo).cast();

    while (pstNode != AVL_NULL_PTR!()).as_bool() {
        iResult = (pstTreeInfo.pfCompare)(pstKey.cast(), (pstNode.cast::<Ptr<u8>>() + iKeyOffset).cast::<Ptr<Void>>()).cast();
        if iResult > 0 {
            pstNode = pstNode.pstRight.cast();
        } else if iResult < 0 {
            pstNode = pstNode.pstLeft.cast();
        } else {
            break;
        }
    }

    return GET_NODE_START_ADDRESS!(pstNode, pstTreeInfo.usNodeOffset).cast();
}


pub fn VOS_AVL3_First(mut pstTree: Ptr<AVL3_TREE>, mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL3_NODE> = Default::default();

    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo).as_bool() {
        return AVL_NULL_PTR!();
    }

    pstNode = pstTree.pstFirst.cast();

    return GET_NODE_START_ADDRESS!(pstNode, pstTreeInfo.usNodeOffset);
}


pub fn VOS_AVL3_Last(mut pstTree: Ptr<AVL3_TREE>, mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL3_NODE> = Default::default();

    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo).as_bool() {
        return AVL_NULL_PTR!();
    }

    pstNode = pstTree.pstLast.cast();

    return GET_NODE_START_ADDRESS!(pstNode, pstTreeInfo.usNodeOffset);
}


pub fn VOS_AVL3_Next(mut pstNode: Ptr<AVL3_NODE>, mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) -> Ptr<Void> {
    let mut pstNodeTmp: Ptr<AVL3_NODE> = pstNode.cast();
    if (pstNodeTmp == AVL_NULL_PTR!()).as_bool() || (pstTreeInfo == AVL_NULL_PTR!()).as_bool() {
        return AVL_NULL_PTR!();
    }

    if (pstNodeTmp.pstRight != AVL_NULL_PTR!()).as_bool() {
        pstNodeTmp = pstNodeTmp.pstRight.cast();
        FIND_LEFTMOST_NODE!(pstNodeTmp);
    } else {
        while (pstNodeTmp != AVL_NULL_PTR!()).as_bool() {
            if (pstNodeTmp.pstParent == AVL_NULL_PTR!()).as_bool() || (pstNodeTmp.pstParent.pstLeft == pstNodeTmp).as_bool() {
                pstNodeTmp = pstNodeTmp.pstParent.cast();
                break;
            }
            pstNodeTmp = pstNodeTmp.pstParent.cast();
        }
    }

    return GET_NODE_START_ADDRESS!(pstNodeTmp, pstTreeInfo.usNodeOffset);
}


pub fn VOS_AVL3_Prev(mut pstNode: Ptr<AVL3_NODE>, mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) -> Ptr<Void> {
    let mut pstNodeTmp: Ptr<AVL3_NODE> = pstNode.cast();
    if (pstNodeTmp == AVL_NULL_PTR!()).as_bool() || (pstTreeInfo == AVL_NULL_PTR!()).as_bool() {
        return AVL_NULL_PTR!();
    }

    if (pstNodeTmp.pstLeft != AVL_NULL_PTR!()).as_bool() {
        pstNodeTmp = pstNodeTmp.pstLeft.cast();
        FIND_RIGHTMOST_NODE!(pstNodeTmp);
    } else {
        while (pstNodeTmp != AVL_NULL_PTR!()).as_bool() {
            if (pstNodeTmp.pstParent == AVL_NULL_PTR!()).as_bool() || (pstNodeTmp.pstParent.pstRight == pstNodeTmp).as_bool() {
                pstNodeTmp = pstNodeTmp.pstParent.cast();
                break;
            }
            pstNodeTmp = pstNodeTmp.pstParent.cast();
        }
    }

    return GET_NODE_START_ADDRESS!(pstNodeTmp, pstTreeInfo.usNodeOffset);
}


