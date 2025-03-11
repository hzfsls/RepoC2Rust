#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _CmptlzCompParam {
    pub level: i32,
    pub dictSize: u32,
    pub litCtx: i32,
    pub litPos: i32,
    pub posBits: i32,
    pub fastBytes: i32,
    pub numThreads: i32,
    pub protData: Ptr<u8>,
    pub protSize: usize,
    pub memHook: Ptr<CmptLzMemHook>,
}

pub type CmptlzCompParam = _CmptlzCompParam;
