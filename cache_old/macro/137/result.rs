macro_rules! CMPTLZ_HIDDEN { () => { #[cfg_attr(target_os = "linux", link_section = ".hidden")] } }
pub(crate) use CMPTLZ_HIDDEN;
