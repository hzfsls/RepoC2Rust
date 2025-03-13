use crate::translation_utils::*;
pub use crate::src::avl_tree_h::*;

#[repr(C)]
#[derive(Default)]
pub struct _AVLTreeNode {
    pub children: Array<Ptr<AVLTreeNode>, 2>,
    pub parent: Ptr<AVLTreeNode>,
    pub key: AVLTreeKey,
    pub value: AVLTreeValue,
    pub height: i32,
}


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _AVLTree {
    pub root_node: Ptr<AVLTreeNode>,
    pub compare_func: AVLTreeCompareFunc,
    pub num_nodes: u32,
}


pub fn avl_tree_new(mut compare_func: AVLTreeCompareFunc) -> Ptr<AVLTree> {
    let mut new_tree: Ptr<AVLTree> = c_malloc!(c_sizeof!(AVLTree));

    if (new_tree == NULL!()).as_bool() {
        return NULL!();
    }

    new_tree.root_node = NULL!();
    new_tree.compare_func = compare_func.cast();
    new_tree.num_nodes = 0;

    return new_tree.cast();
}


pub fn avl_tree_free_subtree(mut tree: Ptr<AVLTree>, mut node: Ptr<AVLTreeNode>) {
    if (node == NULL!()).as_bool() {
        return;
    }

    avl_tree_free_subtree(tree.cast(), node.children[AVL_TREE_NODE_LEFT!()].cast());
    avl_tree_free_subtree(tree.cast(), node.children[AVL_TREE_NODE_RIGHT!()].cast());

    c_free!(node);
}


pub fn avl_tree_free(mut tree: Ptr<AVLTree>) {
    avl_tree_free_subtree(tree.cast(), tree.root_node.cast());
    c_free!(tree);
}


pub fn avl_tree_subtree_height(mut node: Ptr<AVLTreeNode>) -> i32 {
    if (node == NULL!()).as_bool() {
        return 0;
    } else {
        return node.height.cast();
    }
}


pub fn avl_tree_update_height(mut node: Ptr<AVLTreeNode>) {
    let mut left_subtree: Ptr<AVLTreeNode> = Default::default();
    let mut right_subtree: Ptr<AVLTreeNode> = Default::default();
    let mut left_height: i32 = Default::default();
    let mut right_height: i32 = Default::default();

    left_subtree = node.children[AVL_TREE_NODE_LEFT!()].cast();
    right_subtree = node.children[AVL_TREE_NODE_RIGHT!()].cast();
    left_height = avl_tree_subtree_height(left_subtree.cast()).cast();
    right_height = avl_tree_subtree_height(right_subtree.cast()).cast();

    if (left_height > right_height).as_bool() {
        node.height = (left_height + 1).cast();
    } else {
        node.height = (right_height + 1).cast();
    }
}


pub fn avl_tree_node_parent_side(mut node: Ptr<AVLTreeNode>) -> AVLTreeNodeSide {
    if (node.parent.children[AVL_TREE_NODE_LEFT!()] == node).as_bool() {
        return AVL_TREE_NODE_LEFT!();
    } else {
        return AVL_TREE_NODE_RIGHT!();
    }
}


pub fn avl_tree_node_replace(mut tree: Ptr<AVLTree>, mut node1: Ptr<AVLTreeNode>, mut node2: Ptr<AVLTreeNode>) {
    let mut side: i32 = Default::default();

    if (node2 != NULL!()).as_bool() {
        node2.parent = node1.parent.cast();
    }

    if (node1.parent == NULL!()).as_bool() {
        tree.root_node = node2.cast();
    } else {
        side = avl_tree_node_parent_side(node1.cast()).cast();
        node1.parent.children[side] = node2.cast();

        avl_tree_update_height(node1.parent.cast());
    }
}


pub fn avl_tree_rotate(mut tree: Ptr<AVLTree>, mut node: Ptr<AVLTreeNode>, mut direction: AVLTreeNodeSide) -> Ptr<AVLTreeNode> {
    let mut new_root: Ptr<AVLTreeNode> = Default::default();

    new_root = node.children[1 - direction].cast();

    avl_tree_node_replace(tree.cast(), node.cast(), new_root.cast());

    node.children[1 - direction] = new_root.children[direction].cast();
    new_root.children[direction] = node.cast();

    node.parent = new_root.cast();

    if (node.children[1 - direction] != NULL!()).as_bool() {
        node.children[1 - direction].parent = node.cast();
    }

    avl_tree_update_height(new_root.cast());
    avl_tree_update_height(node.cast());

    return new_root.cast();
}


pub fn avl_tree_node_balance(mut tree: Ptr<AVLTree>, mut node: Ptr<AVLTreeNode>) -> Ptr<AVLTreeNode> {
    let mut left_subtree: Ptr<AVLTreeNode> = Default::default();
    let mut right_subtree: Ptr<AVLTreeNode> = Default::default();
    let mut child: Ptr<AVLTreeNode> = Default::default();
    let mut diff: i32 = Default::default();

    left_subtree = node.children[AVL_TREE_NODE_LEFT!()].cast();
    right_subtree = node.children[AVL_TREE_NODE_RIGHT!()].cast();

    diff = (avl_tree_subtree_height(right_subtree.cast()) - avl_tree_subtree_height(left_subtree.cast())).cast();

    if (diff >= 2).as_bool() {
        child = right_subtree.cast();

        if (avl_tree_subtree_height(child.children[AVL_TREE_NODE_RIGHT!()].cast()) <
            avl_tree_subtree_height(child.children[AVL_TREE_NODE_LEFT!()].cast())).as_bool() {
            avl_tree_rotate(tree.cast(), right_subtree.cast(), AVL_TREE_NODE_RIGHT!().cast());
        }

        node = avl_tree_rotate(tree.cast(), node.cast(), AVL_TREE_NODE_LEFT!().cast());
    } else if (diff <= -2).as_bool() {
        child = node.children[AVL_TREE_NODE_LEFT!()].cast();

        if (avl_tree_subtree_height(child.children[AVL_TREE_NODE_LEFT!()].cast()) <
            avl_tree_subtree_height(child.children[AVL_TREE_NODE_RIGHT!()].cast())).as_bool() {
            avl_tree_rotate(tree.cast(), left_subtree.cast(), AVL_TREE_NODE_LEFT!().cast());
        }

        node = avl_tree_rotate(tree.cast(), node.cast(), AVL_TREE_NODE_RIGHT!().cast());
    }

    avl_tree_update_height(node.cast());

    return node.cast();
}


pub fn avl_tree_balance_to_root(mut tree: Ptr<AVLTree>, mut node: Ptr<AVLTreeNode>) {
    let mut rover: Ptr<AVLTreeNode> = node.cast();

    while (rover != NULL!()).as_bool() {
        rover = avl_tree_node_balance(tree.cast(), rover.cast()).cast();
        rover = rover.parent.cast();
    }
}


pub fn avl_tree_insert(mut tree: Ptr<AVLTree>, mut key: AVLTreeKey, mut value: AVLTreeValue) -> Ptr<AVLTreeNode> {
    unimplemented!();
}


pub fn avl_tree_node_get_replacement(mut tree: Ptr<AVLTree>, mut node: Ptr<AVLTreeNode>) -> Ptr<AVLTreeNode> {
    let mut left_subtree: Ptr<AVLTreeNode> = Default::default();
    let mut right_subtree: Ptr<AVLTreeNode> = Default::default();
    let mut result: Ptr<AVLTreeNode> = Default::default();
    let mut child: Ptr<AVLTreeNode> = Default::default();
    let mut left_height: i32 = Default::default();
    let mut right_height: i32 = Default::default();
    let mut side: i32 = Default::default();

    left_subtree = node.children[AVL_TREE_NODE_LEFT!()].cast();
    right_subtree = node.children[AVL_TREE_NODE_RIGHT!()].cast();

    if (left_subtree == NULL!() && right_subtree == NULL!()).as_bool() {
        return NULL!();
    }

    left_height = avl_tree_subtree_height(left_subtree.cast()).cast();
    right_height = avl_tree_subtree_height(right_subtree.cast()).cast();

    if (left_height < right_height).as_bool() {
        side = AVL_TREE_NODE_RIGHT!();
    } else {
        side = AVL_TREE_NODE_LEFT!();
    }

    result = node.children[side].cast();

    while (result.children[1 - side] != NULL!()).as_bool() {
        result = result.children[1 - side].cast();
    }

    child = result.children[side].cast();
    avl_tree_node_replace(tree.cast(), result.cast(), child.cast());

    avl_tree_update_height(result.parent.cast());

    return result.cast();
}


pub fn avl_tree_remove_node(mut tree: Ptr<AVLTree>, mut node: Ptr<AVLTreeNode>) {
    let mut swap_node: Ptr<AVLTreeNode> = Default::default();
    let mut balance_startpoint: Ptr<AVLTreeNode> = Default::default();
    let mut i: i32 = Default::default();

    swap_node = avl_tree_node_get_replacement(tree, node);

    if (swap_node == NULL!()) {
        avl_tree_node_replace(tree, node, NULL!());

        balance_startpoint = node.parent;
    } else {
        if (swap_node.parent == node) {
            balance_startpoint = swap_node;
        } else {
            balance_startpoint = swap_node.parent;
        }

        c_for!(let mut i: i32 = 0; i < 2; i.prefix_plus_plus(); {
            let tmp0 = i;
            swap_node.children[tmp0];

            if (swap_node.children[i] != NULL!()) {
                swap_node.children[i].parent = swap_node;
            }
        });

        swap_node.height = node.height;

        avl_tree_node_replace(tree, node, swap_node);
    }

    c_free!(node);

    tree.num_nodes -= 1;

    avl_tree_balance_to_root(tree, balance_startpoint);
}


pub fn avl_tree_remove(mut tree: Ptr<AVLTree>, mut key: AVLTreeKey) -> i32 {
    let mut node: Ptr<AVLTreeNode> = Default::default();

    node = avl_tree_lookup_node(tree.cast(), key.cast());

    if (node == NULL!()).as_bool() {
        return 0;
    }

    avl_tree_remove_node(tree.cast(), node.cast());

    return 1;
}


pub fn avl_tree_lookup_node(mut tree: Ptr<AVLTree>, mut key: AVLTreeKey) -> Ptr<AVLTreeNode> {
    let mut node: Ptr<AVLTreeNode> = Default::default();
    let mut diff: i32 = Default::default();

    node = tree.root_node.cast();

    while (node != NULL!()).as_bool() {
        diff = (tree.compare_func)(key.cast(), node.key.cast()).cast();

        if diff == 0 {
            return node.cast();
        } else if diff < 0 {
            node = node.children[AVL_TREE_NODE_LEFT!()].cast();
        } else {
            node = node.children[AVL_TREE_NODE_RIGHT!()].cast();
        }
    }

    return NULL!();
}


pub fn avl_tree_lookup(mut tree: Ptr<AVLTree>, mut key: AVLTreeKey) -> AVLTreeValue {
    let mut node: Ptr<AVLTreeNode> = Default::default();

    node = avl_tree_lookup_node(tree.cast(), key.cast());

    if (node == NULL!()).as_bool() {
        return AVL_TREE_NULL!();
    } else {
        return node.value.cast();
    }
}


pub fn avl_tree_root_node(mut tree: Ptr<AVLTree>) -> Ptr<AVLTreeNode> {
    return tree.root_node.cast();
}


pub fn avl_tree_node_key(mut node: Ptr<AVLTreeNode>) -> AVLTreeKey {
    return node.key.cast();
}


pub fn avl_tree_node_value(mut node: Ptr<AVLTreeNode>) -> AVLTreeValue {
    return node.value.cast();
}


pub fn avl_tree_node_child(mut node: Ptr<AVLTreeNode>, mut side: AVLTreeNodeSide) -> Ptr<AVLTreeNode> {
    if (side == AVL_TREE_NODE_LEFT!() || side == AVL_TREE_NODE_RIGHT!()).as_bool() {
        return node.children[side].cast();
    } else {
        return NULL!();
    }
}


pub fn avl_tree_node_parent(mut node: Ptr<AVLTreeNode>) -> Ptr<AVLTreeNode> {
    return node.parent.cast();
}


pub fn avl_tree_num_entries(mut tree: Ptr<AVLTree>) -> u32 {
    return tree.num_nodes.cast();
}


pub fn avl_tree_to_array_add_subtree(mut subtree: Ptr<AVLTreeNode>, mut array: Ptr<AVLTreeValue>, mut index: Ptr<i32>) {
    if (subtree == NULL!()).as_bool() {
        return;
    }

    avl_tree_to_array_add_subtree(subtree.children[AVL_TREE_NODE_LEFT!()].cast(), array.cast(), index.cast());

    array[*index] = subtree.key.cast();
    (*index).prefix_plus_plus();

    avl_tree_to_array_add_subtree(subtree.children[AVL_TREE_NODE_RIGHT!()].cast(), array.cast(), index.cast());
}


pub fn avl_tree_to_array(mut tree: Ptr<AVLTree>) -> Ptr<AVLTreeValue> {
    let mut array: Ptr<AVLTreeValue> = Default::default();
    let mut index: i32 = Default::default();

    array = c_malloc!(c_sizeof!(AVLTreeValue) * tree.num_nodes);

    if (array == NULL!()).as_bool() {
        return NULL!();
    }

    index = 0;

    avl_tree_to_array_add_subtree(tree.root_node.cast(), array.cast(), c_ref!(index).cast());

    return array.cast();
}


