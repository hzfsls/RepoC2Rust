use crate::translation_utils::*;
pub use crate::src::binary_heap_h::*;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _BinaryHeap {
    pub heap_type: BinaryHeapType,
    pub values: Ptr<BinaryHeapValue>,
    pub num_values: u32,
    pub alloced_size: u32,
    pub compare_func: BinaryHeapCompareFunc,
}


pub fn binary_heap_cmp(mut heap: Ptr<BinaryHeap>, mut data1: BinaryHeapValue, mut data2: BinaryHeapValue) -> i32 {
    if (heap.heap_type == BINARY_HEAP_TYPE_MIN!()).as_bool() {
        return (heap.compare_func)(data1.cast(), data2.cast()).cast();
    } else {
        return (-(heap.compare_func)(data1.cast(), data2.cast())).cast();
    }
}


pub fn binary_heap_new(mut heap_type: BinaryHeapType, mut compare_func: BinaryHeapCompareFunc) -> Ptr<BinaryHeap> {
    let mut heap: Ptr<BinaryHeap> = c_malloc!(c_sizeof!(BinaryHeap));

    if (heap == NULL!()).as_bool() {
        return NULL!();
    }

    heap.heap_type = heap_type.cast();
    heap.num_values = 0;
    heap.compare_func = compare_func.cast();

    heap.alloced_size = 16;
    heap.values = c_malloc!(c_sizeof!(BinaryHeapValue) * heap.alloced_size);

    if (heap.values == NULL!()).as_bool() {
        c_free!(heap);
        return NULL!();
    }

    return heap.cast();
}


pub fn binary_heap_free(mut heap: Ptr<BinaryHeap>) {
    c_free!(heap.values);
    c_free!(heap);
}


pub fn binary_heap_insert(mut heap: Ptr<BinaryHeap>, mut value: BinaryHeapValue) -> i32 {
    let mut new_values: Ptr<BinaryHeapValue> = Default::default();
    let mut index: u32 = Default::default();
    let mut new_size: u32 = Default::default();
    let mut parent: u32 = Default::default();

    if (heap.num_values >= heap.alloced_size).as_bool() {
        new_size = heap.alloced_size * 2;
        new_values = c_realloc!(heap.values, c_sizeof!(BinaryHeapValue) * new_size);

        if (new_values == NULL!()).as_bool() {
            return 0;
        }

        heap.alloced_size = new_size;
        heap.values = new_values;
    }

    index = heap.num_values;
    heap.num_values.prefix_plus_plus();

    while (index > 0).as_bool() {
        parent = (index - 1) / 2;

        if (binary_heap_cmp(heap, heap.values[parent], value) < 0).as_bool() {
            break;
        } else {
            heap.values[index] = heap.values[parent].cast();
            index = parent;
        }
    }

    heap.values[index] = value.cast();
    return 1;
}


pub fn binary_heap_pop(mut heap: Ptr<BinaryHeap>) -> BinaryHeapValue {
    let mut result: BinaryHeapValue = Default::default();
    let mut new_value: BinaryHeapValue = Default::default();
    let mut index: u32 = Default::default();
    let mut next_index: u32 = Default::default();
    let mut child1: u32 = Default::default();
    let mut child2: u32 = Default::default();

    if (heap.num_values == 0) {
        return BINARY_HEAP_NULL!();
    }

    result = heap.values[0];

    let tmp0 = heap.num_values - 1;
    new_value = heap.values[tmp0];
    heap.num_values -= 1;

    index = 0;

    loop {
        child1 = index * 2 + 1;
        child2 = index * 2 + 2;

        if (child1 < heap.num_values) && (binary_heap_cmp(heap, new_value, heap.values[child1]) > 0) {
            if (child2 < heap.num_values) && (binary_heap_cmp(heap, heap.values[child1], heap.values[child2]) > 0) {
                next_index = child2;
            } else {
                next_index = child1;
            }
        } else if (child2 < heap.num_values) && (binary_heap_cmp(heap, new_value, heap.values[child2]) > 0) {
            next_index = child2;
        } else {
            heap.values[index] = new_value;
            break;
        }

        heap.values[index] = heap.values[next_index];

        index = next_index;
    }

    return result;
}


pub fn binary_heap_num_entries(mut heap: Ptr<BinaryHeap>) -> u32 {
    return heap.num_values.cast();
}


