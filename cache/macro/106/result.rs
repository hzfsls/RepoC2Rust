macro_rules! BZP_BLOCK_FULL { () => { (bwt.nBlock >= bwt.nBlockMax).as_bool() } }
pub(crate) use BZP_BLOCK_FULL;
