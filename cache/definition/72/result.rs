#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _RapidlzUnalign64 {
    pub v: i64,
}

pub type RapidlzUnalign64 = _RapidlzUnalign64;
