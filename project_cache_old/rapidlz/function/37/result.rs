pub fn RapidlzVersionGet() -> Ptr<u8> {
    return (*g_rapidlzVersion.lock()).cast();
}
