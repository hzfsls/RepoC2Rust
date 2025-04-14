pub fn CmptlzLogWrite(mut errorCode: usize, mut funcName: Ptr<u8>, mut line: u16, mut fmt: Ptr<u8>, mut alist: VaList) {
    let mut output: Array<u8, { LOG_BUF_SIZE!() }> = Default::default();
    let mut ret: i32 = Default::default();
    let mut len: usize = Default::default();
    let mut func: CmptlzLogFunc = *g_cmptlzLogFunc.lock();

    if (func == NULL!()).as_bool() {
        return;
    }

    ret = c_snprintf_s!(output, LOG_BUF_SIZE!(), LOG_BUF_SIZE!() - 1, cstr!("\n[Cmptlz-Log] Func={}, Line={}, Error={}\n"), funcName, line, errorCode);
    if (ret < 0).as_bool() {
        return;
    }
    len = ret.cast();

    ret = c_vsnprintf_s!(output.cast::<Ptr<u8>>() + len, LOG_BUF_SIZE!() - len, LOG_BUF_SIZE!() - len - 1, fmt, alist);
    if (ret < 0).as_bool() {
        return;
    }

    func(output.cast(), c_strlen!(output) + 1);
}
