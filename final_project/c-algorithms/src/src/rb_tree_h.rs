use crate::translation_utils::*;
pub use crate::src::rb_tree_c::rb_tree_lookup;
pub use crate::src::rb_tree_c::rb_tree_root_node;
pub use crate::src::rb_tree_c::_RBTreeNode;
pub use crate::src::rb_tree_c::rb_tree_insert;
pub use crate::src::rb_tree_c::rb_tree_node_parent;
pub use crate::src::rb_tree_c::rb_tree_num_entries;
pub use crate::src::rb_tree_c::rb_tree_node_child;
pub use crate::src::rb_tree_c::_RBTree;
pub use crate::src::rb_tree_c::rb_tree_new;
pub use crate::src::rb_tree_c::rb_tree_remove_node;
pub use crate::src::rb_tree_c::rb_tree_lookup_node;
pub use crate::src::rb_tree_c::rb_tree_node_value;
pub use crate::src::rb_tree_c::rb_tree_to_array;
pub use crate::src::rb_tree_c::rb_tree_remove;
pub use crate::src::rb_tree_c::rb_tree_free;
pub use crate::src::rb_tree_c::rb_tree_node_key;

pub type RBTree = _RBTree;


pub type RBTreeKey = VoidPtr;


pub type RBTreeValue = VoidPtr;


pub type RBTreeNode = _RBTreeNode;


pub type RBTreeCompareFunc = FuncPtr<fn(RBTreeValue, RBTreeValue) -> i32>;


pub type RBTreeNodeColor = i32;
macro_rules! RB_TREE_NODE_RED { () => { 0 } }
pub(crate) use RB_TREE_NODE_RED;
macro_rules! RB_TREE_NODE_BLACK { () => { 1 } }
pub(crate) use RB_TREE_NODE_BLACK;


pub type RBTreeNodeSide = i32;
macro_rules! RB_TREE_NODE_LEFT { () => { 0 } }
pub(crate) use RB_TREE_NODE_LEFT;
macro_rules! RB_TREE_NODE_RIGHT { () => { 1 } }
pub(crate) use RB_TREE_NODE_RIGHT;


macro_rules! ALGORITHM_RB_TREE_H { () => { } }
pub(crate) use ALGORITHM_RB_TREE_H;


macro_rules! RB_TREE_NULL { () => { NULL!() } }
pub(crate) use RB_TREE_NULL;


