use crate::translation_utils::*;
pub use crate::include::avl_adapt_h::*;
pub use crate::src::v_avl3_c::VOS_AVL3_Delete;
pub use crate::src::v_avl3_c::VOS_AVL3_Last;
pub use crate::src::v_avl3_c::VOS_AVL3_First;
pub use crate::src::v_avl3_c::VOS_AVL3_Next;
pub use crate::src::v_avl3_c::AVL3_Find_Or_Find_Next;
pub use crate::src::v_avl3_c::VOS_AVL3_Prev;
pub use crate::src::v_avl3_c::VOS_AVL3_Find;
pub use crate::src::v_avl3_c::VOS_AVL3_Insert_Or_Find;

pub type AVL3_COMPARE = FuncPtr<fn(VoidPtr, VoidPtr) -> i64>;


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct avl3_node {
    pub pstParent: Ptr<avl3_node>,
    pub pstLeft: Ptr<avl3_node>,
    pub pstRight: Ptr<avl3_node>,
    pub sLHeight: i16,
    pub sRHeight: i16,
}

pub type AVL3_NODE = avl3_node;


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct avl3_tree_info {
    pub pfCompare: AVL3_COMPARE,
    pub usKeyOffset: u16,
    pub usNodeOffset: u16,
}

pub type AVL3_TREE_INFO = avl3_tree_info;


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct avl3_tree {
    pub pstRoot: Ptr<AVL3_NODE>,
    pub pstFirst: Ptr<AVL3_NODE>,
    pub pstLast: Ptr<AVL3_NODE>,
}

pub type AVL3_TREE = avl3_tree;


macro_rules! V_AVL3_H { () => { } }
pub(crate) use V_AVL3_H;


macro_rules! VOS_AVL3_INIT_TREE {
    ($TREE:expr, $TREE_INFO:expr) => {
        $TREE.pstFirst = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
        $TREE.pstLast = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
        $TREE.pstRoot = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
    }
}
pub(crate) use VOS_AVL3_INIT_TREE;


macro_rules! VOS_AVL3_INIT_NODE {
    ($NODE:expr) => {
        $NODE.pstParent = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
        $NODE.pstLeft = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
        $NODE.pstRight = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
        $NODE.sLHeight = -1;
        $NODE.sRHeight = -1;
    }
}
pub(crate) use VOS_AVL3_INIT_NODE;


macro_rules! VOS_AVL3_INSERT { ($TREE:expr, $NODE:expr, $TREE_INFO:expr) => 
    { 
        AVL_NULL_PTR!() == VOS_AVL3_Insert_Or_Find(c_ref!($TREE), c_ref!($NODE), c_ref!($TREE_INFO)) 
    } 
}
pub(crate) use VOS_AVL3_INSERT;


macro_rules! VOS_AVL3_INSERT_OR_FIND {
    ($TREE:expr, $NODE:expr, $TREE_INFO:expr) => {
        VOS_AVL3_Insert_Or_Find(c_ref!($TREE), c_ref!($NODE), c_ref!($TREE_INFO))
    }
}
pub(crate) use VOS_AVL3_INSERT_OR_FIND;


macro_rules! VOS_AVL3_DELETE {
    ($TREE:expr, $NODE:expr) => {
        VOS_AVL3_Delete(c_ref!($TREE), c_ref!($NODE))
    }
}
pub(crate) use VOS_AVL3_DELETE;


macro_rules! VOS_AVL3_FIND { ($TREE:expr, $KEY:expr, $TREE_INFO:expr) => 
    {
        VOS_AVL3_Find(c_ref!($TREE), $KEY, c_ref!($TREE_INFO))
    }
}
pub(crate) use VOS_AVL3_FIND;


macro_rules! VOS_AVL3_NEXT { ($NODE:expr, $TREE_INFO:expr) => { VOS_AVL3_Next(c_ref!($NODE), c_ref!($TREE_INFO)) } }
pub(crate) use VOS_AVL3_NEXT;


macro_rules! VOS_AVL3_PREV { ($NODE:expr, $TREE_INFO:expr) => 
    {
        VOS_AVL3_Prev(c_ref!($NODE), c_ref!($TREE_INFO))
    }
}
pub(crate) use VOS_AVL3_PREV;


macro_rules! VOS_AVL3_FIRST {
    ($TREE:expr, $TREE_INFO:expr) => {
        VOS_AVL3_First(c_ref!($TREE), c_ref!($TREE_INFO))
    }
}
pub(crate) use VOS_AVL3_FIRST;


macro_rules! VOS_AVL3_LAST { ($TREE:expr, $TREE_INFO:expr) => { VOS_AVL3_Last(c_ref!($TREE), c_ref!($TREE_INFO)) } }
pub(crate) use VOS_AVL3_LAST;


macro_rules! VOS_AVL3_IN_TREE { ($NODE:expr) => { ($NODE.sLHeight != -1) && ($NODE.sRHeight != -1) } }
pub(crate) use VOS_AVL3_IN_TREE;


macro_rules! VOS_AVL3_FIND_NEXT {
    ($TREE:expr, $KEY:expr, $TREE_INFO:expr) => {
        AVL3_Find_Or_Find_Next(c_ref!($TREE), $KEY, AVL_TRUE!(), c_ref!($TREE_INFO))
    }
}
pub(crate) use VOS_AVL3_FIND_NEXT;


macro_rules! VOS_AVL3_FIND_OR_FIND_NEXT {
    ($TREE:expr, $KEY:expr, $TREE_INFO:expr) => {
        AVL3_Find_Or_Find_Next(c_ref!($TREE), $KEY, AVL_FALSE!(), c_ref!($TREE_INFO))
    }
}
pub(crate) use VOS_AVL3_FIND_OR_FIND_NEXT;


macro_rules! VOS_AVL3_MAX { ($X:expr, $Y:expr) => { if $X > $Y { $X } else { $Y } } }
pub(crate) use VOS_AVL3_MAX;


