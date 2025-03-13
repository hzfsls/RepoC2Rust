#[repr(C)]
#[derive(Default)]
pub struct _BinaryHeap {
    pub heap_type: BinaryHeapType,
    pub values: Ptr<BinaryHeapValue>,
    pub num_values: u32,
    pub alloced_size: u32,
    pub compare_func: BinaryHeapCompareFunc,
}
