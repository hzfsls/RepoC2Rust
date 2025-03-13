macro_rules! VOS_AVLL_FIRST {
    ($TREE:expr) => {
        VOS_AVL3_FIRST!($TREE.stTree.cast(), $TREE.stTreeInfo.cast())
    }
}
pub(crate) use VOS_AVLL_FIRST;
