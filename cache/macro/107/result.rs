macro_rules! BZP_BUFF_READ_EMPTY { () => { (bzpf.input.pos >= bzpf.input.nBuf).as_bool() } }
pub(crate) use BZP_BUFF_READ_EMPTY;
