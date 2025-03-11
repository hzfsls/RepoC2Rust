#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _CmptlzOpt {
    pub state: CmptlzState,
    pub price: u32,
    pub posPrev: u32,
    pub backPrev: u32,
    pub backs: Array<u32, { CMPTLZ_NUM_REPS!() }>,
}

pub type CmptlzOpt = _CmptlzOpt;
