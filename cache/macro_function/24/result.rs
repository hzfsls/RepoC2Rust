macro_rules! VOS_AVL3_FIND { ($TREE:expr, $KEY:expr, $TREE_INFO:expr) => 
    {
        VOS_AVL3_Find(c_ref!($TREE), $KEY, c_ref!($TREE_INFO))
    }
}
pub(crate) use VOS_AVL3_FIND;
