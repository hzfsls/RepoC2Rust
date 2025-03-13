use crate::translation_utils::*;
pub use crate::include::v_avl3_h::*;
pub use crate::include::v_avl3_h::AVL3_COMPARE;
pub use crate::include::v_avl3_h::AVL3_NODE;

pub type AVLL_COMPARE = AVL3_COMPARE;


pub type AVLL_NODE = AVL3_NODE;


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct avll_tree {
    pub stTree: AVL3_TREE,
    pub stTreeInfo: AVL3_TREE_INFO,
}

pub type AVLL_TREE = avll_tree;


macro_rules! V_AVLL_H { () => { } }
pub(crate) use V_AVLL_H;


macro_rules! VOS_AVLL_INIT_TREE {
    ($TREE:expr, $COMPARE:expr, $KEY_OFF:expr, $NODE_OFF:expr) => {
        $TREE.stTreeInfo.pfCompare = $COMPARE;
        $TREE.stTreeInfo.usKeyOffset = $KEY_OFF;
        $TREE.stTreeInfo.usNodeOffset = $NODE_OFF;
        VOS_AVL3_INIT_TREE!($TREE.stTree, $TREE.stTreeInfo);
    }
}
pub(crate) use VOS_AVLL_INIT_TREE;


macro_rules! VOS_AVLL_INIT_NODE { ($NODE:expr) => { VOS_AVL3_INIT_NODE!($NODE) } }
pub(crate) use VOS_AVLL_INIT_NODE;


macro_rules! VOS_AVLL_INSERT { ($TREE:expr, $NODE:expr) => 
    {
        VOS_AVL3_INSERT!($TREE.stTree, $NODE, $TREE.stTreeInfo);
    }
}
pub(crate) use VOS_AVLL_INSERT;


macro_rules! VOS_AVLL_INSERT_OR_FIND {
    ($TREE:expr, $NODE:expr) => {
        VOS_AVL3_INSERT_OR_FIND!($TREE.stTree, $NODE, $TREE.stTreeInfo)
    }
}
pub(crate) use VOS_AVLL_INSERT_OR_FIND;


macro_rules! VOS_AVLL_DELETE { ($TREE:expr, $NODE:expr) => { VOS_AVL3_DELETE!($TREE.stTree, $NODE) } }
pub(crate) use VOS_AVLL_DELETE;


macro_rules! VOS_AVLL_FIND { ($TREE:expr, $KEY:expr) => { VOS_AVL3_FIND!($TREE.stTree, $KEY, $TREE.stTreeInfo) } }
pub(crate) use VOS_AVLL_FIND;


macro_rules! VOS_AVLL_NEXT { ($TREE:expr, $NODE:expr) => { VOS_AVL3_NEXT!($NODE, $TREE.stTreeInfo) } }
pub(crate) use VOS_AVLL_NEXT;


macro_rules! VOS_AVLL_PREV { ($TREE:expr, $NODE:expr) => { VOS_AVL3_PREV!($NODE, $TREE.stTreeInfo) } }
pub(crate) use VOS_AVLL_PREV;


macro_rules! VOS_AVLL_FIRST {
    ($TREE:expr) => {
        VOS_AVL3_FIRST!($TREE.stTree.cast(), $TREE.stTreeInfo.cast())
    }
}
pub(crate) use VOS_AVLL_FIRST;


macro_rules! VOS_AVLL_LAST { ($TREE:expr) => { VOS_AVL3_LAST!($TREE.stTree, $TREE.stTreeInfo) } }
pub(crate) use VOS_AVLL_LAST;


macro_rules! VOS_AVLL_IN_TREE { ($NODE:expr) => { VOS_AVL3_IN_TREE!($NODE) } }
pub(crate) use VOS_AVLL_IN_TREE;


macro_rules! VOS_AVLL_FIND_NEXT { ($TREE:expr, $KEY:expr) => 
    {
        VOS_AVL3_FIND_NEXT!($TREE.stTree, $KEY, $TREE.stTreeInfo)
    }
}
pub(crate) use VOS_AVLL_FIND_NEXT;


macro_rules! VOS_AVLL_FIND_OR_FIND_NEXT {
    ($TREE:expr, $KEY:expr) => {
        VOS_AVL3_FIND_OR_FIND_NEXT!($TREE.stTree.cast(), $KEY.cast(), $TREE.stTreeInfo.cast())
    }
}
pub(crate) use VOS_AVLL_FIND_OR_FIND_NEXT;


