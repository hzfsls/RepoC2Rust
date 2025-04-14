pub fn VOS_MD5PadBuff(mut context: Ptr<MD5_CTX>) -> bool {
    let mut needAnotherBuff: bool = (context.uiPos >= MD5_TEXT_IN_BUFFER_MAX!());
    let tmp0 = context.uiPos;
    context.aucBuffer[tmp0] = 0x80;
    context.uiPos += 1;
    if needAnotherBuff {
        while (context.uiPos < MD5_BUFFER_SIZE!()) {
            let tmp0 = context.uiPos;
            context.aucBuffer[tmp0] = 0;
            context.uiPos += 1;
        }
    } else {
        while (context.uiPos < MD5_TEXT_IN_BUFFER_MAX!()) {
            let tmp0 = context.uiPos;
            context.aucBuffer[tmp0] = 0;
            context.uiPos += 1;
        }
        MD5_RECORD_MESSAGE_LEN!(context);
    }
    return needAnotherBuff;
}
