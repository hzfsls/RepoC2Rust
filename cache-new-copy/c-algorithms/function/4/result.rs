pub fn rb_tree_rotate(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>, mut direction: RBTreeNodeSide) -> Ptr<RBTreeNode> {
    let mut new_root: Ptr<RBTreeNode>;

    new_root = node.children[1 - direction];

    rb_tree_node_replace(tree, node, new_root);

    node.children[1 - direction] = new_root.children[direction];
    new_root.children[direction] = node;

    node.parent = new_root;

    if (node.children[1 - direction] != NULL!()).as_bool() {
        node.children[1 - direction].parent = node;
    }

    return new_root;
}
