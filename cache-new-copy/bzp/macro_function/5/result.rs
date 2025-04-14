macro_rules! BZP_BUFF_READ_EMPTY { ($bzpf:expr) => { (*$bzpf.input.lock()).pos >= (*$bzpf.input.lock()).nBuf } }
pub(crate) use BZP_BUFF_READ_EMPTY;
