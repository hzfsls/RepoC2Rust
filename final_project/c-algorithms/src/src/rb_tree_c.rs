use crate::translation_utils::*;
pub use crate::src::rb_tree_h::*;

#[repr(C)]
#[derive(Default)]
pub struct _RBTreeNode {
    pub color: RBTreeNodeColor,
    pub key: RBTreeKey,
    pub value: RBTreeValue,
    pub parent: Ptr<RBTreeNode>,
    pub children: Array<Ptr<RBTreeNode>, 2>,
}


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _RBTree {
    pub root_node: Ptr<RBTreeNode>,
    pub compare_func: RBTreeCompareFunc,
    pub num_nodes: i32,
}


pub fn rb_tree_node_side(mut node: Ptr<RBTreeNode>) -> RBTreeNodeSide {
    if (node.parent.children[RB_TREE_NODE_LEFT!()] == node).as_bool() {
        return RB_TREE_NODE_LEFT!();
    } else {
        return RB_TREE_NODE_RIGHT!();
    }
}


pub fn rb_tree_node_sibling(mut node: Ptr<RBTreeNode>) -> Ptr<RBTreeNode> {
    let mut side: RBTreeNodeSide = Default::default();
    side = rb_tree_node_side(node.cast()).cast();
    return node.parent.children[1 - side].cast();
}


pub fn rb_tree_node_uncle(mut node: Ptr<RBTreeNode>) -> Ptr<RBTreeNode> {
    return rb_tree_node_sibling(node.parent.cast()).cast();
}


pub fn rb_tree_node_replace(mut tree: Ptr<RBTree>, mut node1: Ptr<RBTreeNode>, mut node2: Ptr<RBTreeNode>) {
    let mut side: i32 = Default::default();

    if (node2 != NULL!()).as_bool() {
        node2.parent = node1.parent.cast();
    }

    if (node1.parent == NULL!()).as_bool() {
        tree.root_node = node2.cast();
    } else {
        side = rb_tree_node_side(node1.cast()).cast();
        node1.parent.children[side] = node2.cast();
    }
}


pub fn rb_tree_rotate(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>, mut direction: RBTreeNodeSide) -> Ptr<RBTreeNode> {
    let mut new_root: Ptr<RBTreeNode> = Default::default();

    new_root = node.children[1 - direction].cast();

    rb_tree_node_replace(tree.cast(), node.cast(), new_root.cast());

    node.children[1 - direction] = new_root.children[direction].cast();
    new_root.children[direction] = node.cast();

    node.parent = new_root.cast();

    if (node.children[1 - direction] != NULL!()).as_bool() {
        node.children[1 - direction].parent = node.cast();
    }

    return new_root.cast();
}


pub fn rb_tree_new(mut compare_func: RBTreeCompareFunc) -> Ptr<RBTree> {
    let mut new_tree: Ptr<RBTree> = c_malloc!(c_sizeof!(RBTree));

    if (new_tree == NULL!()).as_bool() {
        return NULL!();
    }

    new_tree.root_node = NULL!();
    new_tree.num_nodes = 0;
    new_tree.compare_func = compare_func.cast();

    return new_tree.cast();
}


pub fn rb_tree_free_subtree(mut node: Ptr<RBTreeNode>) {
    if (node != NULL!()).as_bool() {
        rb_tree_free_subtree(node.children[RB_TREE_NODE_LEFT!()].cast());
        rb_tree_free_subtree(node.children[RB_TREE_NODE_RIGHT!()].cast());
        c_free!(node);
    }
}


pub fn rb_tree_free(mut tree: Ptr<RBTree>) {
    rb_tree_free_subtree(tree.root_node.cast());
    c_free!(tree);
}


pub fn rb_tree_insert_case1(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    if (node.parent == NULL!()).as_bool() {
        node.color = RB_TREE_NODE_BLACK!();
    } else {
        rb_tree_insert_case2(tree.cast(), node.cast());
    }
}


pub fn rb_tree_insert_case2(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    if (node.parent.color != RB_TREE_NODE_BLACK!()).as_bool() {
        rb_tree_insert_case3(tree.cast(), node.cast());
    }
}


pub fn rb_tree_insert_case3(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    let mut grandparent: Ptr<RBTreeNode> = Default::default();
    let mut uncle: Ptr<RBTreeNode> = Default::default();

    grandparent = node.parent.parent.cast();
    uncle = rb_tree_node_uncle(node.cast());

    if (uncle != NULL!() && uncle.color == RB_TREE_NODE_RED!()).as_bool() {
        node.parent.color = RB_TREE_NODE_BLACK!();
        uncle.color = RB_TREE_NODE_BLACK!();
        grandparent.color = RB_TREE_NODE_RED!();

        rb_tree_insert_case1(tree.cast(), grandparent.cast());
    } else {
        rb_tree_insert_case4(tree.cast(), node.cast());
    }
}


pub fn rb_tree_insert_case4(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    let mut next_node: Ptr<RBTreeNode> = Default::default();
    let mut side: RBTreeNodeSide = Default::default();

    side = rb_tree_node_side(node.cast());

    if (side != rb_tree_node_side(node.parent)).as_bool() {
        next_node = node.parent.cast();
        rb_tree_rotate(tree.cast(), node.parent.cast(), (1 - side).cast());
    } else {
        next_node = node.cast();
    }

    rb_tree_insert_case5(tree.cast(), next_node.cast());
}


pub fn rb_tree_insert_case5(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    let mut parent: Ptr<RBTreeNode> = Default::default();
    let mut grandparent: Ptr<RBTreeNode> = Default::default();
    let mut side: RBTreeNodeSide = Default::default();

    parent = node.parent.cast();
    grandparent = parent.parent.cast();

    side = rb_tree_node_side(node.cast());

    rb_tree_rotate(tree.cast(), grandparent.cast(), (1 - side).cast());

    parent.color = RB_TREE_NODE_BLACK!();
    grandparent.color = RB_TREE_NODE_RED!();
}


pub fn rb_tree_insert(mut tree: Ptr<RBTree>, mut key: RBTreeKey, mut value: RBTreeValue) -> Ptr<RBTreeNode> {
    unimplemented!();
}


pub fn rb_tree_lookup_node(mut tree: Ptr<RBTree>, mut key: RBTreeKey) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode> = Default::default();
    let mut side: RBTreeNodeSide = Default::default();
    let mut diff: i32 = Default::default();

    node = tree.root_node.cast();

    while (node != NULL!()).as_bool() {
        diff = (tree.compare_func)(key.cast(), node.key.cast()).cast();

        if diff == 0 {
            return node.cast();
        } else if diff < 0 {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }

        node = node.children[side].cast();
    }

    return NULL!();
}


pub fn rb_tree_lookup(mut tree: Ptr<RBTree>, mut key: RBTreeKey) -> RBTreeValue {
    let mut node: Ptr<RBTreeNode> = Default::default();

    node = rb_tree_lookup_node(tree.cast(), key.cast());

    if (node == NULL!()).as_bool() {
        return RB_TREE_NULL!();
    } else {
        return node.value.cast();
    }
}


pub fn rb_tree_remove_node(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
}


pub fn rb_tree_remove(mut tree: Ptr<RBTree>, mut key: RBTreeKey) -> i32 {
    let mut node: Ptr<RBTreeNode> = Default::default();

    node = rb_tree_lookup_node(tree.cast(), key.cast());

    if (node == NULL!()).as_bool() {
        return 0;
    }

    rb_tree_remove_node(tree.cast(), node.cast());

    return 1;
}


pub fn rb_tree_root_node(mut tree: Ptr<RBTree>) -> Ptr<RBTreeNode> {
    return tree.root_node.cast();
}


pub fn rb_tree_node_key(mut node: Ptr<RBTreeNode>) -> RBTreeKey {
    return node.key.cast();
}


pub fn rb_tree_node_value(mut node: Ptr<RBTreeNode>) -> RBTreeValue {
    return node.value.cast();
}


pub fn rb_tree_node_child(mut node: Ptr<RBTreeNode>, mut side: RBTreeNodeSide) -> Ptr<RBTreeNode> {
    if (side == RB_TREE_NODE_LEFT!() || side == RB_TREE_NODE_RIGHT!()).as_bool() {
        return node.children[side].cast();
    } else {
        return NULL!();
    }
}


pub fn rb_tree_node_parent(mut node: Ptr<RBTreeNode>) -> Ptr<RBTreeNode> {
    return node.parent.cast();
}


pub fn rb_tree_to_array(mut tree: Ptr<RBTree>) -> Ptr<RBTreeValue> {
    return NULL!();
}


pub fn rb_tree_num_entries(mut tree: Ptr<RBTree>) -> i32 {
    return tree.num_nodes.cast();
}


