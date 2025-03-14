macro_rules! VOS_AVLL_INSERT {
    ($TREE:expr, $NODE:expr) => {
        VOS_AVL3_INSERT!($TREE.stTree.cast(), $NODE.cast(), $TREE.stTreeInfo.cast());
    }
}
pub(crate) use VOS_AVLL_INSERT;
