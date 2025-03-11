pub fn VOS_MD5PadBuff(mut context: Ptr<MD5_CTX>) -> bool {
    let mut needAnotherBuff: bool = (context.uiPos >= MD5_TEXT_IN_BUFFER_MAX!()).as_bool();

    context.aucBuffer[context.uiPos] = 0x80;
    context.uiPos.suffix_plus_plus();
    if needAnotherBuff {
        c_for!(; context.uiPos < MD5_BUFFER_SIZE!(); context.uiPos.suffix_plus_plus(); {
            context.aucBuffer[context.uiPos] = 0;
        });
    } else {
        c_for!(; context.uiPos < MD5_TEXT_IN_BUFFER_MAX!(); context.uiPos.suffix_plus_plus(); {
            context.aucBuffer[context.uiPos] = 0;
        });

        MD5_RECORD_MESSAGE_LEN!(context);
    }

    return needAnotherBuff.cast();
}
