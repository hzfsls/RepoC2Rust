macro_rules! VOS_AVLL_FIND_OR_FIND_NEXT {
    ($TREE:expr, $KEY:expr) => {
        VOS_AVL3_FIND_OR_FIND_NEXT!($TREE.stTree.cast(), $KEY.cast(), $TREE.stTreeInfo.cast())
    }
}
pub(crate) use VOS_AVLL_FIND_OR_FIND_NEXT;
