pub fn rb_tree_node_key(mut node: Ptr<RBTreeNode>) -> RBTreeKey {
    return node.key.cast();
}
