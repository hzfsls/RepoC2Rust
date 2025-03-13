use crate::translation_utils::*;

macro_rules! V_AVL3_INNER_H { () => { } }
pub(crate) use V_AVL3_INNER_H;


macro_rules! TREE_OR_TREEINFO_IS_NULL { ($pstTree:expr, $pstTreeInfo:expr) => { ($pstTree == AVL_NULL_PTR!()) || ($pstTreeInfo == AVL_NULL_PTR!()) } }
pub(crate) use TREE_OR_TREEINFO_IS_NULL;


macro_rules! GET_NODE_START_ADDRESS {
    ($pstNode:expr, $usOffset:expr) => {
        if $pstNode != AVL_NULL_PTR!() {
            $pstNode.cast::<Ptr<u8>>() - $usOffset
        } else {
            AVL_NULL_PTR!()
        }
    }
}
pub(crate) use GET_NODE_START_ADDRESS;


macro_rules! GET_KEYOFFSET { ($pstTreeInfo:expr) => { ($pstTreeInfo.usKeyOffset - $pstTreeInfo.usNodeOffset) as i32 } }
pub(crate) use GET_KEYOFFSET;


