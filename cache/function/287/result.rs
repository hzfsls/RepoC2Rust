pub fn rb_tree_node_side(mut node: Ptr<RBTreeNode>) -> RBTreeNodeSide {
    if node.parent.children[RB_TREE_NODE_LEFT!()] == node {
        return RB_TREE_NODE_LEFT!();
    } else {
        return RB_TREE_NODE_RIGHT!();
    }
}
