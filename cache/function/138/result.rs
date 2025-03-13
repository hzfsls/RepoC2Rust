pub fn binomial_tree_ref(mut tree: Ptr<BinomialTree>) {
    if tree != NULL!() {
        tree.refcount.prefix_plus_plus();
    }
}
