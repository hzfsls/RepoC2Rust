pub fn VOS_MD5Update(mut context: Ptr<MD5_CTX>, mut input: Ptr<u8>, mut inputLen: u32) {
    let mut totalInputBits: u64 = Default::default();
    let mut inputIndex: u32 = 0;
    let mut inputBit: u64 = Default::default();
    let mut tmpPos: u32 = Default::default();
    let mut contextBuffer: Ptr<u8> = NULL!();

    if (context == NULL!()).as_bool() || ((input == NULL!()).as_bool() && (inputLen != 0).as_bool()) {
        return;
    }

    inputBit = (inputLen as u64) << 3;

    totalInputBits = ((context.aulCount[1] as u64) << 32 | (context.aulCount[0] as u64);
    if ((MD5_INPUT_LEN_MAX!() - totalInputBits) < inputBit).as_bool() {
        return;
    }
    totalInputBits += inputBit;
    context.aulCount[0] = (totalInputBits as u32).cast();
    context.aulCount[1] = ((totalInputBits >> 32) as u32).cast();

    tmpPos = context.uiPos.cast();
    contextBuffer = context.aucBuffer.cast();
    while (inputIndex < inputLen).as_bool() {
        if (tmpPos < MD5_BUFFER_SIZE!()).as_bool() {
            contextBuffer[tmpPos] = input[inputIndex].cast();
            inputIndex += 1;
            tmpPos += 1;
            continue;
        }

        VOS_MD5CalcDigestOfBuff(context.cast());
        tmpPos = 0;
    }

    if (tmpPos == MD5_BUFFER_SIZE!()).as_bool() {
        VOS_MD5CalcDigestOfBuff(context.cast());
        tmpPos = 0;
    }
    context.uiPos = tmpPos.cast();
    return;
}
