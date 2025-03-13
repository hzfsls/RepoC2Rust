#[repr(C)]
#[derive(Default)]
pub struct _ArrayList {
    pub data: Ptr<ArrayListValue>,
    pub length: u32,
    pub _alloced: u32,
}
