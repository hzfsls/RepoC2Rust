pub fn rb_tree_node_parent(mut node: Ptr<RBTreeNode>) -> Ptr<RBTreeNode> {
    return node.parent.cast();
}
