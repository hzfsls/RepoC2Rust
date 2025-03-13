#[repr(C)]
#[derive(Default)]
pub struct _BinomialTree {
    pub value: BinomialHeapValue,
    pub order: u16,
    pub refcount: u16,
    pub subtrees: Ptr<Ptr<BinomialTree>>,
}
