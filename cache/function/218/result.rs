pub fn queue_free(mut queue: Ptr<Queue>) {
    while !queue_is_empty(queue.cast()) {
        queue_pop_head(queue.cast());
    }
    c_free!(queue);
}
