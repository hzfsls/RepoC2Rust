macro_rules! ALWAYS_NO_INLINE { () => { #[inline(never)] } }
pub(crate) use ALWAYS_NO_INLINE;
