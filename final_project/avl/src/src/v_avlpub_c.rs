use crate::translation_utils::*;
pub use crate::include::v_avl_base_h::*;
pub use crate::src::v_avlbase_h::*;

pub fn VOS_AVL_Insert_Or_Find(mut pstTree: Ptr<AVL_TREE>, mut pstNode: Ptr<AVL_NODE>) -> Ptr<Void> {
    let mut pstParentNode: Ptr<AVL_NODE> = Default::default();
    let mut iResult: i32 = Default::default();

    if (pstTree == AVL_NULL_PTR!()).as_bool() || (pstNode == AVL_NULL_PTR!()).as_bool() || (VOS_AVL_IN_TREE!(*pstNode)).as_bool() {
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
    while (pstParentNode != AVL_NULL_PTR!()).as_bool() {
        iResult = (pstTree.pfnCompare)(pstNode.pKey.cast(), pstParentNode.pKey.cast()).cast();
        if iResult > 0 {
            if (pstParentNode.pstRight != AVL_NULL_PTR!()).as_bool() {
                pstParentNode = pstParentNode.pstRight.cast();
                continue;
            }

            VosAvlNodeRightInsert((c_ref!(pstTree.pstRoot).cast::<Ptr<AVLBASE_TREE_S>>()).cast(), pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>(), pstNode.cast::<Ptr<AVLBASE_NODE_S>>());
            break;
        } else if iResult < 0 {
            if (pstParentNode.pstLeft != AVL_NULL_PTR!()).as_bool() {
                pstParentNode = pstParentNode.pstLeft.cast();
                continue;
            }

            VosAvlNodeLeftInsert((c_ref!(pstTree.pstRoot).cast::<Ptr<AVLBASE_TREE_S>>()).cast(), pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>(), pstNode.cast::<Ptr<AVLBASE_NODE_S>>());
            break;
        }

        pstNode.sRHeight = -1;
        pstNode.sLHeight = -1;
        return pstParentNode.pSelf.cast();
    }

    if (pstParentNode != AVL_NULL_PTR!()).as_bool() {
        VosAvlBalanceTree((c_ref!(pstTree.pstRoot).cast::<Ptr<AVLBASE_TREE_S>>()).cast(), pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>());
    }

    return AVL_NULL_PTR!();
}


pub fn VOS_AVL_Delete(mut pstTree: Ptr<AVL_TREE>, mut pstNode: Ptr<AVL_NODE>) {
    let mut pstBaseNode: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut pstBaseTree: Ptr<AVLBASE_TREE_S> = Default::default();

    if (pstTree == AVL_NULL_PTR!()).as_bool() || (pstNode == AVL_NULL_PTR!()).as_bool() || (!VOS_AVL_IN_TREE!(*pstNode)).as_bool() {
        return;
    }

    pstBaseNode = pstNode.cast::<Ptr<AVLBASE_NODE_S>>();
    pstBaseTree = c_ref!(pstTree.pstRoot).cast::<Ptr<Void>>().cast::<Ptr<AVLBASE_TREE_S>>();
    VosAvlDelete(pstBaseNode.cast(), pstBaseTree.cast());
    return;
}


pub fn VOS_AVL_Find(mut pstTree: Ptr<AVL_TREE>, mut pKey: Ptr<Void>) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL_NODE> = Default::default();
    let mut iResult: i32 = Default::default();

    if (pstTree == AVL_NULL_PTR!()).as_bool() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot.cast();

    while (pstNode != AVL_NULL_PTR!()).as_bool() {
        iResult = (pstTree.pfnCompare)(pKey.cast(), pstNode.pKey.cast()).cast();
        if iResult > 0 {
            pstNode = pstNode.pstRight.cast();
        } else if iResult < 0 {
            pstNode = pstNode.pstLeft.cast();
        } else {
            break;
        }
    }

    return if pstNode != AVL_NULL_PTR!() { pstNode.pSelf.cast() } else { AVL_NULL_PTR!() };
}


pub fn VOS_AVL_Next(mut pstNode: Ptr<AVL_NODE>) -> Ptr<Void> {
    let mut pstNodeTmp: Ptr<AVL_NODE> = pstNode.cast();
    if (pstNodeTmp == AVL_NULL_PTR!()).as_bool() || (!VOS_AVL_IN_TREE!(*pstNodeTmp)).as_bool() {
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

    return if pstNodeTmp != AVL_NULL_PTR!() { pstNodeTmp.pSelf.cast() } else { AVL_NULL_PTR!() };
}


pub fn VOS_AVL_Prev(mut pstNode: Ptr<AVL_NODE>) -> Ptr<Void> {
    let mut pstNodeTmp: Ptr<AVL_NODE> = pstNode.cast();
    if (pstNodeTmp == AVL_NULL_PTR!()).as_bool() || (!VOS_AVL_IN_TREE!(*pstNodeTmp)).as_bool() {
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

    return if pstNodeTmp != AVL_NULL_PTR!() { pstNodeTmp.pSelf.cast() } else { AVL_NULL_PTR!() };
}


pub fn VOS_AVL_Find_Or_Find_Next(mut pstTree: Ptr<AVL_TREE>, mut pKey: Ptr<Void>, mut bValue: u32) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL_NODE> = Default::default();
    let mut pFoundNode: Ptr<Void> = AVL_NULL_PTR!();
    let mut iResult: i32 = Default::default();

    if (pstTree == AVL_NULL_PTR!()).as_bool() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot.cast();

    if (pstNode == AVL_NULL_PTR!()).as_bool() {
        return pFoundNode.cast();
    }

    loop {
        iResult = (pstTree.pfnCompare)(pKey.cast(), pstNode.pKey.cast()).cast();
        if iResult > 0 {
            if (pstNode.pstRight == AVL_NULL_PTR!()).as_bool() {
                pFoundNode = VOS_AVL_Next(pstNode.cast()).cast();
                break;
            }
            pstNode = pstNode.pstRight.cast();
        } else if iResult < 0 {
            if (pstNode.pstLeft == AVL_NULL_PTR!()).as_bool() {
                pFoundNode = pstNode.pSelf.cast();
                break;
            }
            pstNode = pstNode.pstLeft.cast();
        } else {
            if bValue != 0 {
                pFoundNode = VOS_AVL_Next(pstNode.cast()).cast();
            } else {
                pFoundNode = pstNode.pSelf.cast();
            }
            break;
        }
    }

    return pFoundNode.cast();
}


