pub fn binomial_heap_merge(mut heap: Ptr<BinomialHeap>, mut other: Ptr<BinomialHeap>) -> i32 {
    let mut new_roots: Ptr<Ptr<BinomialTree>>;
    let mut new_roots_length: u32;
    let mut vals: Array<Ptr<BinomialTree>, 3> = Default::default();
    let mut num_vals: i32;
    let mut carry: Ptr<BinomialTree> = Default::default();
    let mut new_carry: Ptr<BinomialTree> = Default::default();
    let mut max: u32;
    let mut i: u32;

    if (heap.roots_length > other.roots_length) {
        max = heap.roots_length + 1;
    } else {
        max = other.roots_length + 1;
    }

    new_roots = c_malloc!(max * c_sizeof!(Ptr<BinomialTree>));

    if (new_roots == NULL!()) {
        return 0;
    }

    new_roots_length = 0;
    carry = NULL!();

    c_for!(let mut i: u32 = 0; i < max; i.prefix_plus_plus(); {
        num_vals = 0;

        if (i < heap.roots_length && heap.roots[i] != NULL!()) {
            let tmp0 = num_vals;
            vals[tmp0];
            num_vals += 1;
        }

        if (i < other.roots_length && other.roots[i] != NULL!()) {
            vals[num_vals] = other.roots[i];
            num_vals += 1;
        }

        if (carry != NULL!()) {
            vals[num_vals] = carry;
            num_vals += 1;
        }

        if ((num_vals & 1) != 0) {
            new_roots[i] = vals[num_vals - 1];
            binomial_tree_ref(new_roots[i]);
            new_roots_length = i + 1;
        } else {
            new_roots[i] = NULL!();
        }

        if ((num_vals & 2) != 0) {
            new_carry = binomial_tree_merge(heap, vals[0], vals[1]);

            if (new_carry == NULL!()) {
                binomial_heap_merge_undo(new_roots, i);

                binomial_tree_unref(carry);

                return 0;
            }
        } else {
            new_carry = NULL!();
        }

        binomial_tree_unref(carry);

        carry = new_carry;

        binomial_tree_ref(carry);
    });

    c_for!(let mut i: u32 = 0; i < heap.roots_length; i.prefix_plus_plus(); {
        if (heap.roots[i] != NULL!()) {
            binomial_tree_unref(heap.roots[i]);
        }
    });

    c_free!(heap.roots);
    heap.roots = new_roots;
    heap.roots_length = new_roots_length;

    return 1;
}
