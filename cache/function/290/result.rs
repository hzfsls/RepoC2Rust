pub fn rb_tree_node_replace(mut tree: Ptr<RBTree>, mut node1: Ptr<RBTreeNode>, mut node2: Ptr<RBTreeNode>) {
    let mut side: i32 = Default::default();

    if node2 != NULL!() {
        node2.parent = node1.parent.cast();
    }

    if node1.parent == NULL!() {
        tree.root_node = node2.cast();
    } else {
        side = rb_tree_node_side(node1.cast()).cast();
        node1.parent.children[side] = node2.cast();
    }
}
