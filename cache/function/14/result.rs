pub fn binomial_tree_unref(mut tree: Ptr<BinomialTree>) {
    let mut i: i32 = Default::default();

    if (tree == NULL!()) {
        return;
    }

    tree.refcount.suffix_minus_minus();

    if (tree.refcount == 0) {

        c_for!(let mut i: i32 = 0; i < tree.order.cast(); i.prefix_plus_plus(); {
            binomial_tree_unref(tree.subtrees[i]);
        });

        c_free!(tree.subtrees);
        c_free!(tree);
    }
}
