pub fn binary_heap_num_entries(mut heap: Ptr<BinaryHeap>) -> u32 {
    return heap.num_values.cast();
}
