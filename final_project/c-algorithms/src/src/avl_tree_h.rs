use crate::translation_utils::*;
pub use crate::src::avl_tree_c::avl_tree_remove;
pub use crate::src::avl_tree_c::avl_tree_num_entries;
pub use crate::src::avl_tree_c::avl_tree_root_node;
pub use crate::src::avl_tree_c::avl_tree_subtree_height;
pub use crate::src::avl_tree_c::avl_tree_free;
pub use crate::src::avl_tree_c::avl_tree_lookup;
pub use crate::src::avl_tree_c::avl_tree_insert;
pub use crate::src::avl_tree_c::avl_tree_lookup_node;
pub use crate::src::avl_tree_c::avl_tree_new;
pub use crate::src::avl_tree_c::avl_tree_node_parent;
pub use crate::src::avl_tree_c::_AVLTreeNode;
pub use crate::src::avl_tree_c::avl_tree_to_array;
pub use crate::src::avl_tree_c::_AVLTree;
pub use crate::src::avl_tree_c::avl_tree_node_value;
pub use crate::src::avl_tree_c::avl_tree_node_child;
pub use crate::src::avl_tree_c::avl_tree_remove_node;
pub use crate::src::avl_tree_c::avl_tree_node_key;

pub type AVLTree = _AVLTree;


pub type AVLTreeKey = VoidPtr;


pub type AVLTreeValue = VoidPtr;


pub type AVLTreeNode = _AVLTreeNode;


pub type AVLTreeNodeSide = i32;
macro_rules! AVL_TREE_NODE_LEFT { () => { 0 } }
pub(crate) use AVL_TREE_NODE_LEFT;
macro_rules! AVL_TREE_NODE_RIGHT { () => { 1 } }
pub(crate) use AVL_TREE_NODE_RIGHT;


pub type AVLTreeCompareFunc = FuncPtr<fn(AVLTreeValue, AVLTreeValue) -> i32>;


macro_rules! ALGORITHM_AVLTREE_H { () => { } }
pub(crate) use ALGORITHM_AVLTREE_H;


macro_rules! AVL_TREE_NULL { () => { NULL!() } }
pub(crate) use AVL_TREE_NULL;


