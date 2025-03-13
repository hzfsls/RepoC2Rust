use crate::translation_utils::*;
pub use crate::src::binomial_heap_h::*;

pub type BinomialTree = _BinomialTree;


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _BinomialTree {
    pub value: BinomialHeapValue,
    pub order: u16,
    pub refcount: u16,
    pub subtrees: Ptr<Ptr<BinomialTree>>,
}


#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _BinomialHeap {
    pub heap_type: BinomialHeapType,
    pub compare_func: BinomialHeapCompareFunc,
    pub num_values: u32,
    pub roots: Ptr<Ptr<BinomialTree>>,
    pub roots_length: u32,
}


pub fn binomial_heap_cmp(mut heap: Ptr<BinomialHeap>, mut data1: BinomialHeapValue, mut data2: BinomialHeapValue) -> i32 {
    if (heap.heap_type == BINOMIAL_HEAP_TYPE_MIN!()).as_bool() {
        return (heap.compare_func)(data1.cast(), data2.cast()).cast();
    } else {
        return (-(heap.compare_func)(data1.cast(), data2.cast())).cast();
    }
}


pub fn binomial_tree_ref(mut tree: Ptr<BinomialTree>) {
    if (tree != NULL!()).as_bool() {
        tree.refcount.prefix_plus_plus();
    }
}


pub fn binomial_tree_unref(mut tree: Ptr<BinomialTree>) {
    let mut i: i32 = Default::default();

    if (tree == NULL!()) {
        return;
    }

    tree.refcount.suffix_minus_minus();

    if (tree.refcount == 0) {
        c_for!(let mut i: i32 = 0; i < tree.order.cast(); i.prefix_plus_plus(); {
            binomial_tree_unref(tree.subtrees[i]);
        });

        c_free!(tree.subtrees);
        c_free!(tree);
    }
}


pub fn binomial_tree_merge(mut heap: Ptr<BinomialHeap>, mut tree1: Ptr<BinomialTree>, mut tree2: Ptr<BinomialTree>) -> Ptr<BinomialTree> {
    let mut new_tree: Ptr<BinomialTree> = Default::default();
    let mut tmp: Ptr<BinomialTree> = Default::default();
    let mut i: i32 = Default::default();

    if (binomial_heap_cmp(heap, tree1.value, tree2.value) > 0) {
        tmp = tree1;
        tree1 = tree2;
        tree2 = tmp;
    }

    new_tree = c_malloc!(c_sizeof!(BinomialTree));

    if (new_tree == NULL!()) {
        return NULL!();
    }

    new_tree.refcount = 0;
    new_tree.order = (tree1.order + 1).cast::<u16>();

    new_tree.value = tree1.value;

    new_tree.subtrees = c_malloc!(c_sizeof!(Ptr<BinomialTree>) * new_tree.order);

    if (new_tree.subtrees == NULL!()) {
        c_free!(new_tree);
        return NULL!();
    }

    c_memcpy!(new_tree.subtrees, tree1.subtrees, c_sizeof!(Ptr<BinomialTree>) * tree1.order);
    let tmp0 = new_tree.order - 1;
    new_tree.subtrees[tmp0] = tree2;

    c_for!(let mut i: i32 = 0; i < new_tree.order.cast(); i.prefix_plus_plus(); {
        binomial_tree_ref(new_tree.subtrees[i]);
    });

    return new_tree;
}


pub fn binomial_heap_merge_undo(mut new_roots: Ptr<Ptr<BinomialTree>>, mut count: u32) {
    let mut i: u32 = Default::default();
    c_for!(let mut i = 0; i <= count; i.prefix_plus_plus(); {
        binomial_tree_unref(new_roots[i].cast());
    });
    c_free!(new_roots);
}


pub fn binomial_heap_merge(mut heap: Ptr<BinomialHeap>, mut other: Ptr<BinomialHeap>) -> i32 {
    let mut new_roots: Ptr<Ptr<BinomialTree>>;
    let mut new_roots_length: u32;
    let mut vals: Array<Ptr<BinomialTree>, 3> = Default::default();
    let mut num_vals: i32;
    let mut carry: Ptr<BinomialTree> = Default::default();
    let mut new_carry: Ptr<BinomialTree> = Default::default();
    let mut max: u32;
    let mut i: u32;

    if (heap.roots_length > other.roots_length).as_bool() {
        max = heap.roots_length + 1;
    } else {
        max = other.roots_length + 1;
    }

    new_roots = c_malloc!(max * c_sizeof!(Ptr<BinomialTree>));

    if (new_roots == NULL!()).as_bool() {
        return 0;
    }

    new_roots_length = 0;
    carry = NULL!();

    c_for!(let mut i: u32 = 0; i < max; i.prefix_plus_plus(); {
        num_vals = 0;

        if (i < heap.roots_length && heap.roots[i] != NULL!()).as_bool() {
            vals[num_vals] = heap.roots[i].cast();
            num_vals += 1;
        }

        if (i < other.roots_length && other.roots[i] != NULL!()).as_bool() {
            vals[num_vals] = other.roots[i].cast();
            num_vals += 1;
        }

        if (carry != NULL!()).as_bool() {
            vals[num_vals] = carry.cast();
            num_vals += 1;
        }

        if ((num_vals & 1) != 0).as_bool() {
            new_roots[i] = vals[num_vals - 1].cast();
            binomial_tree_ref(new_roots[i].cast());
            new_roots_length = i + 1;
        } else {
            new_roots[i] = NULL!();
        }

        if ((num_vals & 2) != 0).as_bool() {
            new_carry = binomial_tree_merge(heap.cast(), vals[0].cast(), vals[1].cast());

            if (new_carry == NULL!()).as_bool() {
                binomial_heap_merge_undo(new_roots.cast(), i.cast());

                binomial_tree_unref(carry.cast());

                return 0;
            }
        } else {
            new_carry = NULL!();
        }

        binomial_tree_unref(carry.cast());

        carry = new_carry.cast();

        binomial_tree_ref(carry.cast());
    });

    c_for!(let mut i: u32 = 0; i < heap.roots_length; i.prefix_plus_plus(); {
        if (heap.roots[i] != NULL!()).as_bool() {
            binomial_tree_unref(heap.roots[i].cast());
        }
    });

    c_free!(heap.roots);
    heap.roots = new_roots.cast();
    heap.roots_length = new_roots_length;

    return 1;
}


pub fn binomial_heap_new(mut heap_type: BinomialHeapType, mut compare_func: BinomialHeapCompareFunc) -> Ptr<BinomialHeap> {
    let mut new_heap: Ptr<BinomialHeap> = c_calloc!(1, c_sizeof!(BinomialHeap));

    if (new_heap == NULL!()).as_bool() {
        return NULL!();
    }

    new_heap.heap_type = heap_type.cast();
    new_heap.compare_func = compare_func.cast();

    return new_heap.cast();
}


pub fn binomial_heap_free(mut heap: Ptr<BinomialHeap>) {
    let mut i: u32 = Default::default();

    c_for!(let mut i = 0; i < heap.roots_length; i.suffix_plus_plus(); {
        binomial_tree_unref(heap.roots[i].cast());
    });

    c_free!(heap.roots);
    c_free!(heap);
}


pub fn binomial_heap_insert(mut heap: Ptr<BinomialHeap>, mut value: BinomialHeapValue) -> i32 {
    let mut fake_heap: BinomialHeap = Default::default();
    let mut new_tree: Ptr<BinomialTree> = Default::default();
    let mut result: i32 = Default::default();

    new_tree = c_malloc!(c_sizeof!(BinomialTree));

    if (new_tree == NULL!()).as_bool() {
        return 0;
    }

    new_tree.value = value.cast();
    new_tree.order = 0;
    new_tree.refcount = 1;
    new_tree.subtrees = NULL!();

    fake_heap.heap_type = heap.heap_type.cast();
    fake_heap.compare_func = heap.compare_func.cast();
    fake_heap.num_values = 1;
    fake_heap.roots = c_ref!(new_tree).cast();
    fake_heap.roots_length = 1;

    result = binomial_heap_merge(heap.cast(), c_ref!(fake_heap).cast()).cast();

    if (result != 0).as_bool() {
        heap.num_values.prefix_plus_plus();
    }

    binomial_tree_unref(new_tree.cast());

    return result.cast();
}


pub fn binomial_heap_pop(mut heap: Ptr<BinomialHeap>) -> BinomialHeapValue {
    let mut least_tree: Ptr<BinomialTree> = Default::default();
    let mut fake_heap: BinomialHeap = Default::default();
    let mut result: BinomialHeapValue = Default::default();
    let mut i: u32 = Default::default();
    let mut least_index: u32 = Default::default();

    if (heap.num_values == 0).as_bool() {
        return BINOMIAL_HEAP_NULL!();
    }

    least_index = UINT_MAX!();

    c_for!(i = 0; i < heap.roots_length; i.prefix_plus_plus(); {
        if (heap.roots[i] == NULL!()).as_bool() {
            continue;
        }

        if (least_index == UINT_MAX!() || binomial_heap_cmp(heap.cast(), heap.roots[i].value.cast(), heap.roots[least_index].value.cast()) < 0).as_bool() {
            least_index = i.cast();
        }
    });

    least_tree = heap.roots[least_index].cast();
    heap.roots[least_index] = NULL!();

    fake_heap.heap_type = heap.heap_type.cast();
    fake_heap.compare_func = heap.compare_func.cast();
    fake_heap.roots = least_tree.subtrees.cast();
    fake_heap.roots_length = least_tree.order.cast();

    if binomial_heap_merge(heap.cast(), c_ref!(fake_heap).cast()).as_bool() {
        result = least_tree.value.cast();
        binomial_tree_unref(least_tree.cast());

        heap.num_values -= 1;

        return result.cast();
    } else {
        heap.roots[least_index] = least_tree.cast();

        return BINOMIAL_HEAP_NULL!();
    }
}


pub fn binomial_heap_num_entries(mut heap: Ptr<BinomialHeap>) -> u32 {
    return heap.num_values.cast();
}


