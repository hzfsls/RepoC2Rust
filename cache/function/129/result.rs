pub fn BzpBuildHuffmanTree(mut huffman: Ptr<BzpHuffmanInfo>) {
    BzpHuffmanInitArray(huffman.cast());
    BzpHeapInit(huffman.cast());
    let mut idx1: i32 = Default::default();
    let mut idx2: i32 = Default::default();
    while huffman.nHeap > 1 {
        idx1 = huffman.heap[1].cast();
        let tmp = huffman.nHeap.suffix_minus_minus();
        huffman.heap[1] = huffman.heap[tmp].cast();
        BzpHeapAdjustDown(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap.cast());
        idx2 = huffman.heap[1].cast();
        let tmp = huffman.nHeap.suffix_minus_minus();
        huffman.heap[1] = huffman.heap[tmp].cast();
        BzpHeapAdjustDown(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap.cast());
        let tmp = huffman.nWeight;
        huffman.weight[tmp] = BzpHuffmanWeightAdd(huffman.weight[idx1].cast(), huffman.weight[idx2].cast()).cast();
        huffman.parent[idx1] = huffman.nWeight.cast();
        huffman.parent[idx2] = huffman.nWeight.cast();
        let tmp = huffman.nWeight;
        huffman.parent[tmp] = -1;
        huffman.nHeap.prefix_plus_plus();
        let tmp = huffman.nHeap;
        huffman.heap[tmp] = huffman.nWeight.cast();
        huffman.nWeight.prefix_plus_plus();
        BzpHeapAdjustUp(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap.cast());
    }
}
