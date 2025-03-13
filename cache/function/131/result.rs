pub fn sortedarray_free(mut sortedarray: Ptr<SortedArray>) {
    if sortedarray != NULL!() {
        c_free!(sortedarray.data);
        c_free!(sortedarray);
    }
}
