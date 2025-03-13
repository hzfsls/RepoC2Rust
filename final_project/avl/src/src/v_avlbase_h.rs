use crate::translation_utils::*;
pub use crate::src::v_avlbase_c::VosAvlRotateLeft;
pub use crate::src::v_avlbase_c::VosAvlSwapRightMost;
pub use crate::src::v_avlbase_c::VosAvlRebalance;
pub use crate::src::v_avlbase_c::VosAvlRotateRight;
pub use crate::src::v_avlbase_c::VosAvlBalanceTree;
pub use crate::src::v_avlbase_c::VosAvlSwapLeftMost;
pub use crate::src::v_avlbase_c::VosAvlDeleteCheck;
pub use crate::src::v_avlbase_c::VosAvlDelete;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct AVLBaseNode {
    pub pstParent: Ptr<AVLBaseNode>,
    pub pstLeft: Ptr<AVLBaseNode>,
    pub pstRight: Ptr<AVLBaseNode>,
    pub sLHeight: i16,
    pub sRHeight: i16,
}

pub type AVLBASE_NODE_S = AVLBaseNode;


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct AVLBaseTree {
    pub pstRoot: Ptr<AVLBASE_NODE_S>,
    pub pstFirst: Ptr<AVLBASE_NODE_S>,
    pub pstLast: Ptr<AVLBASE_NODE_S>,
}

pub type AVLBASE_TREE_S = AVLBaseTree;


macro_rules! V_AVLBASE_H { () => { } }
pub(crate) use V_AVLBASE_H;


macro_rules! FIND_LEFTMOST_NODE { ($pstNode:expr) =>
    {
        while $pstNode.pstLeft != AVL_NULL_PTR!()
        {
            $pstNode = $pstNode.pstLeft;
        }
    }
}
pub(crate) use FIND_LEFTMOST_NODE;


macro_rules! FIND_RIGHTMOST_NODE { ($pstNode:expr) =>
    {
        while $pstNode.pstRight != AVL_NULL_PTR!()
        {
            $pstNode = $pstNode.pstRight;
        }
    }
}
pub(crate) use FIND_RIGHTMOST_NODE;


pub fn VosAvlNodeRightInsert(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstParentNode: Ptr<AVLBASE_NODE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) {
    pstNode.pstParent = pstParentNode.cast();
    pstParentNode.pstRight = pstNode.cast();
    pstParentNode.sRHeight = 1;
    if (pstParentNode == pstTree.pstLast).as_bool() {
        pstTree.pstLast = pstNode.cast();
    }
}


pub fn VosAvlNodeLeftInsert(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstParentNode: Ptr<AVLBASE_NODE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) {
    pstNode.pstParent = pstParentNode.cast();
    pstParentNode.pstLeft = pstNode.cast();
    pstParentNode.sLHeight = 1;
    if (pstParentNode == pstTree.pstFirst).as_bool() {
        pstTree.pstFirst = pstNode.cast();
    }
}


