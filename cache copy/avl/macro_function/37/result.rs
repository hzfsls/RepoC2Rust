macro_rules! VOS_AVL3_IN_TREE { ($NODE:expr) => { ($NODE.sLHeight != -1) && ($NODE.sRHeight != -1) } }
pub(crate) use VOS_AVL3_IN_TREE;
