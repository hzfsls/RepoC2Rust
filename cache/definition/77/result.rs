#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _RapidlzCCtx {
    pub hashTable: Ptr<u8>,
    pub hashType: u8,
    pub hashBits: u8,
    pub step: u8,
    pub bufferLimit: u8,
}

pub type RapidlzCCtx = _RapidlzCCtx;
