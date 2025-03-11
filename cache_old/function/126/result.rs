pub fn BzpHeapAdjustDown(mut heap: Ptr<i32>, mut weight: Ptr<i32>, mut nHeap: i32) {
    let mut pos: i32 = 1;
    let mut chpos: i32 = pos << 1;
    let mut tmpid: i32 = heap[pos].cast();
    let mut tmpv: i32 = weight[tmpid].cast();
    while chpos <= nHeap {
        if (chpos | 1) <= nHeap && weight[heap[chpos]] > weight[heap[chpos | 1]] {
            chpos |= 1;
        }
        if tmpv < weight[heap[chpos]] {
            break;
        }
        heap[pos] = heap[chpos].cast();
        pos = chpos.cast();
        chpos = pos << 1;
    }
    heap[pos] = tmpid.cast();
}
