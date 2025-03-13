use crate::translation_utils::*;
pub use crate::include::avl_adapt_h::*;
pub use crate::src::v_avlpub_c::VOS_AVL_Find_Or_Find_Next;
pub use crate::src::v_avlpub_c::VOS_AVL_Find;
pub use crate::src::v_avlpub_c::VOS_AVL_Insert_Or_Find;
pub use crate::src::v_avlpub_c::VOS_AVL_Delete;
pub use crate::src::v_avlpub_c::VOS_AVL_Prev;
pub use crate::src::v_avlpub_c::VOS_AVL_Next;

pub type AVL_V2_COMPARE_FUNC = FuncPtr<fn(VoidPtr, VoidPtr) -> i64>;


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct avl_node {
    pub pstParent: Ptr<avl_node>,
    pub pstLeft: Ptr<avl_node>,
    pub pstRight: Ptr<avl_node>,
    pub sLHeight: i16,
    pub sRHeight: i16,
    pub pSelf: VoidPtr,
    pub pKey: VoidPtr,
}

pub type AVL_NODE = avl_node;


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct avl_tree {
    pub pfnCompare: AVL_V2_COMPARE_FUNC,
    pub pstRoot: Ptr<AVL_NODE>,
    pub pstFirst: Ptr<AVL_NODE>,
    pub pstLast: Ptr<AVL_NODE>,
}

pub type AVL_TREE = avl_tree;


macro_rules! V_AVL_BASE_H { () => { } }
pub(crate) use V_AVL_BASE_H;


macro_rules! VOS_AVL_INIT_TREE {
    ($TREE:expr, $COMPARE:expr) => {
        $TREE.pfnCompare = $COMPARE;
        $TREE.pstFirst = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
        $TREE.pstLast = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
        $TREE.pstRoot = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
    }
}
pub(crate) use VOS_AVL_INIT_TREE;


macro_rules! VOS_AVL_INIT_NODE {
    ($NODE:expr, $SELF:expr, $KEY:expr) => {
        $NODE.pstParent = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
        $NODE.pstLeft = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
        $NODE.pstRight = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
        $NODE.pSelf = $SELF;
        $NODE.pKey = $KEY;
        $NODE.sLHeight = -1;
        $NODE.sRHeight = -1;
    }
}
pub(crate) use VOS_AVL_INIT_NODE;


macro_rules! VOS_AVL_INSERT { ($TREE:expr, $NODE:expr) => { VOS_AVL_Insert_Or_Find(c_ref!($TREE), c_ref!($NODE)) == AVL_NULL_PTR!() } }
pub(crate) use VOS_AVL_INSERT;


macro_rules! VOS_AVL_INSERT_OR_FIND {
    ($TREE:expr, $NODE:expr) => {
        VOS_AVL_Insert_Or_Find(c_ref!($TREE), c_ref!($NODE))
    }
}
pub(crate) use VOS_AVL_INSERT_OR_FIND;


macro_rules! VOS_AVL_DELETE {
    ($TREE:expr, $NODE:expr) => {
        VOS_AVL_Delete(c_ref!($TREE), c_ref!($NODE));
    }
}
pub(crate) use VOS_AVL_DELETE;


macro_rules! VOS_AVL_FIND { ($TREE:expr, $KEY:expr) => { VOS_AVL_Find(c_ref!($TREE), $KEY) } }
pub(crate) use VOS_AVL_FIND;


macro_rules! VOS_AVL_NEXT { ($NODE:expr) => { VOS_AVL_Next(c_ref!($NODE)) } }
pub(crate) use VOS_AVL_NEXT;


macro_rules! VOS_AVL_PREV { ($NODE:expr) => { VOS_AVL_Prev(c_ref!($NODE)) } }
pub(crate) use VOS_AVL_PREV;


macro_rules! VOS_AVL_FIRST {
    ($TREE:expr) => {
        if c_ref!($TREE).pstFirst != AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>() {
            c_ref!($TREE).pstFirst.pSelf
        } else {
            AVL_NULL_PTR!()
        }
    }
}
pub(crate) use VOS_AVL_FIRST;


macro_rules! VOS_AVL_LAST {
    ($TREE:expr) => {
        if c_ref!($TREE).pstLast != AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>() {
            c_ref!($TREE).pstLast.pSelf
        } else {
            AVL_NULL_PTR!()
        }
    }
}
pub(crate) use VOS_AVL_LAST;


macro_rules! VOS_AVL_IN_TREE {
    ($NODE:expr) => {
        ($NODE.sLHeight != -1) && ($NODE.sRHeight != -1)
    }
}
pub(crate) use VOS_AVL_IN_TREE;


macro_rules! VOS_AVL_FIND_NEXT { ($TREE:expr, $KEY:expr) => { VOS_AVL_Find_Or_Find_Next(c_ref!($TREE), $KEY, AVL_TRUE!()) } }
pub(crate) use VOS_AVL_FIND_NEXT;


macro_rules! VOS_AVL_FIND_OR_FIND_NEXT {
    ($TREE:expr, $KEY:expr) => {
        VOS_AVL_Find_Or_Find_Next(c_ref!($TREE), $KEY, AVL_FALSE!())
    }
}
pub(crate) use VOS_AVL_FIND_OR_FIND_NEXT;


macro_rules! VOS_V2_AVL_MAX { ($X:expr, $Y:expr) => { if $X > $Y { $X } else { $Y } } }
pub(crate) use VOS_V2_AVL_MAX;


