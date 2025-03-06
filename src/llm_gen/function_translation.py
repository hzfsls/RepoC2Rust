from openai import OpenAI
from pebble import ProcessPool
from httpx import Client

import json

proxies = {
    'http': 'http://z00835967:99310Wen.@proxy.huawei.com:8080',
    'https': 'https://z00835967:99310Wen.@proxy.huawei.com:8080',
}

client = OpenAI(api_key="sk-ea5eeb6b740a435e9a068ec46c594c3f", base_url="https://api.deepseek.com/beta",
    http_client=Client(
    proxy=proxies['http'],
    verify=False  # 注意：禁用 SSL 验证可能有安全风险，请根据实际情况决定是否需要这样做
))

example_1 = """
Source:
```c
void VOS_MD5CalcEx(uint8_t *output, uint32_t outputLen, const uint8_t *input, uint32_t inputLen)
{
    MD5_CTX context;
    if (outputLen < MD5_DIGEST_LEN)
    {
        return;
    }
    VOS_MD5Init(&context);
    VOS_MD5Update(&context, (uint8_t *)(uintptr_t)input, inputLen);
    VOS_MD5FinalEx(output, outputLen, &context);
}
```

Translation:
```rust
pub fn VOS_MD5CalcEx(mut output: Ptr<u8>, mut outputLen: u32, mut input: Ptr<u8>, mut inputLen: u32) {
    let mut context: MD5_CTX = Default::default();
    if outputLen < MD5_DIGEST_LEN!() {
        return;
    }
    VOS_MD5Init(c_ref!(context).cast());
    VOS_MD5Update(c_ref!(context).cast(), input.cast::<Ptr<u8>>(), inputLen.cast());
    VOS_MD5FinalEx(output.cast(), outputLen.cast(), c_ref!(context).cast());
}
```
"""

example_2 = """
Source:
```c
void VosAvlRebalance(AVLBASE_NODE_S **ppstSubTree)
{
    int iMoment;
    iMoment = (*ppstSubTree)->sRHeight - (*ppstSubTree)->sLHeight;
    if (iMoment > 1)
    {
        if ((*ppstSubTree)->pstRight->sLHeight > (*ppstSubTree)->pstRight->sRHeight)
        {
            VosAvlRotateRight(&(*ppstSubTree)->pstRight);
        }
        VosAvlRotateLeft(ppstSubTree);
    }
    else if (iMoment < -1)
    {
        if ((*ppstSubTree)->pstLeft->sRHeight > (*ppstSubTree)->pstLeft->sLHeight)
        {
            VosAvlRotateLeft(&(*ppstSubTree)->pstLeft);
        }
        VosAvlRotateRight(ppstSubTree);
    }
    return;
}
```

Translation:
```rust
pub fn VosAvlRebalance(mut ppstSubTree: Ptr<Ptr<AVLBASE_NODE_S>>) {
    let mut iMoment: i32;
    iMoment = ((*ppstSubTree).sRHeight - (*ppstSubTree).sLHeight).cast();
    if iMoment > 1 {
        if (*ppstSubTree).pstRight.sLHeight > (*ppstSubTree).pstRight.sRHeight {
            VosAvlRotateRight(c_ref!((*ppstSubTree).pstRight).cast());
        }
        VosAvlRotateLeft(ppstSubTree.cast());
    } else if iMoment < -1 {
        if (*ppstSubTree).pstLeft.sRHeight > (*ppstSubTree).pstLeft.sLHeight {
            VosAvlRotateLeft(c_ref!((*ppstSubTree).pstLeft).cast());
        }
        VosAvlRotateRight(ppstSubTree.cast());
    }
    return;
}
```
"""

example_3 = """
Source:
```c
static void RapidlzCopyMatchFast(uint8_t *dst, uint8_t *match, uint16_t offset, uint32_t length)
{
    uint8_t *dstCurr = dst;
    uint8_t *matchPtr = match;
    if (offset >= RAPIDLZ_SIXTEEN_BYTE)
    {
        RapidlzCopyLiteralsFast(matchPtr, dstCurr, length);
        return;
    }
    for (int i = 0; i < RAPIDLZ_EIGHT_BYTE; ++i)
    {
        dstCurr[i] = matchPtr[i];
    }
    if (length <= RAPIDLZ_EIGHT_BYTE)
    {
        return;
    }
    uint8_t *dstEnd = dstCurr + length;
    if (offset < RAPIDLZ_EIGHT_BYTE)
    {
        matchPtr += g_overlapOffAddVal[offset];
        dstCurr += RAPIDLZ_EIGHT_BYTE;
    }
    do
    {
        RapidlzCopy8Byte(dstCurr, matchPtr);
        dstCurr += RAPIDLZ_EIGHT_BYTE;
        matchPtr += RAPIDLZ_EIGHT_BYTE;
    } while (dstCurr < dstEnd);
}
```

Translation:
```rust
pub fn RapidlzCopyMatchFast(mut dst: Ptr<u8>, mut r#match: Ptr<u8>, mut offset: u16, mut length: u32) {
    let mut dstCurr: Ptr<u8> = dst.cast();
    let mut matchPtr: Ptr<u8> = r#match.cast();
    if offset >= RAPIDLZ_SIXTEEN_BYTE!() {
        RapidlzCopyLiteralsFast(matchPtr.cast(), dstCurr.cast(), length.cast());
        return;
    }
    c_for!(let mut i: i32 = 0; i < RAPIDLZ_EIGHT_BYTE!(); i.prefix_plus_plus(); {
        dstCurr[i] = matchPtr[i].cast();
    });
    if length <= RAPIDLZ_EIGHT_BYTE!() {
        return;
    }
    let mut dstEnd: Ptr<u8> = (dstCurr + length).cast();
    if offset < RAPIDLZ_EIGHT_BYTE!() {
        matchPtr += (*g_overlapOffAddVal.lock())[offset];
        dstCurr += RAPIDLZ_EIGHT_BYTE!();
    }
    c_do!({
        RapidlzCopy8Byte(dstCurr.cast(), matchPtr.cast());
        dstCurr += RAPIDLZ_EIGHT_BYTE!();
        matchPtr += RAPIDLZ_EIGHT_BYTE!();
    } while dstCurr < dstEnd);
}
```
"""

example_4 = """
Source:
```c
static uint16_t RapidlzReadLE16Bit(const void *addr)
{
    if (RapidlzIsLE() != 0)
    {
        return *(const uint16_t *)addr;
    }
    uint8_t tmp1 = ((const uint8_t *)addr)[0];
    uint8_t tmp2 = ((const uint8_t *)addr)[1];
    return (uint16_t)(tmp1 + (tmp2 << 8));
}
```

Translation:
```rust
pub fn RapidlzReadLE16Bit(mut addr: Ptr<Void>) -> u16 {
    if RapidlzIsLE() != 0 {
        return (*addr.cast::<Ptr<u16>>()).cast();
    }
    let mut tmp1: u8 = ((addr.cast::<Ptr<u8>>())[0]).cast();
    let mut tmp2: u8 = ((addr.cast::<Ptr<u8>>())[1]).cast();
    return (tmp1 + (tmp2 << 8)).cast::<u16>();
}
```
"""

example_5 = """
Source:
```c
void *VOS_AVL_Find(AVL_TREE *pstTree, const void *pKey)
{
    AVL_NODE *pstNode;
    int iResult;
    if (pstTree == AVL_NULL_PTR)
    {
        return AVL_NULL_PTR;
    }
    pstNode = pstTree->pstRoot;
    while (pstNode != AVL_NULL_PTR)
    {
        iResult = pstTree->pfnCompare(pKey, pstNode->pKey);
        if (iResult > 0)
        {
            pstNode = pstNode->pstRight;
        }
        else if (iResult < 0)
        {
            pstNode = pstNode->pstLeft;
        }
        else
        {
            break;
        }
    }
    return ((pstNode != AVL_NULL_PTR) ? pstNode->pSelf : AVL_NULL_PTR);
}
```

Translation:
```rust
pub fn VOS_AVL_Find(mut pstTree: Ptr<AVL_TREE>, mut pKey: Ptr<Void>) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL_NODE> = Default::default();
    let mut iResult: i32 = Default::default();
    if pstTree == AVL_NULL_PTR!() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot.cast();
    while pstNode != AVL_NULL_PTR!() {
        iResult = (pstTree.pfnCompare)(pKey.cast(), pstNode.pKey.cast()).cast();
        if iResult > 0 {
            pstNode = pstNode.pstRight.cast();
        } else if iResult < 0 {
            pstNode = pstNode.pstLeft.cast();
        } else {
            break;
        }
    }
    return if pstNode != AVL_NULL_PTR!() { pstNode.pSelf.cast() } else { AVL_NULL_PTR!() };
}
```
"""

example_6 = """
Source:
```c
uint32_t BzpReadUInt24(InDeComdata *inData)
{
    uint8_t ch;
    uint32_t val = 0;
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    return val;
}
```

Translation:
```rust
pub fn BzpReadUInt24(mut inData: Ptr<InDeComdata>) -> u32 {
    let mut ch: u8 = Default::default();
    let mut val: u32 = 0;
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    return val;
}
```
"""

example_7 = """
Source:
```c
int32_t BzpGetDictionaryList(InDeComdata *inData)
{
    int32_t ninUse = 0;
    bool use16[16] = {0};
    bool inUse[BZP_ASCII_SIZE] = {0};
    for (int32_t i = 0; i < BZP_GROUPS_ASCII; i++)
    {
        use16[i] = BzpReadBits(BZP_BIT, inData);
    }
    for (int32_t i = 0; i < BZP_GROUPS_ASCII; i++)
    {
        if (use16[i])
        {
            for (int32_t j = 0; j < BZP_CHARS_PER_GROUP_ASCII; j++)
            {
                inUse[i * BZP_GROUPS_ASCII + j] = BzpReadBits(BZP_BIT, inData);
            }
        }
    }
    for (int32_t i = 0; i < BZP_ASCII_SIZE; i++)
    {
        if (inUse[i])
        {
            inData->list[ninUse++] = i;
        }
    }
    return ninUse;
}
```

Translation:
```rust
pub fn BzpGetDictionaryList(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ninUse: i32 = 0;
    let mut use16: Array<bool, 16> = arr![false; 16];
    let mut inUse: Array<bool, { BZP_ASCII_SIZE!() }> = arr![false; BZP_ASCII_SIZE!()];
    c_for!(let mut i: i32 = 0; i < BZP_GROUPS_ASCII!(); i.suffix_plus_plus(); {
        use16[i] = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
    });
    c_for!(let mut i: i32 = 0; i < BZP_GROUPS_ASCII!(); i.suffix_plus_plus(); {
        if use16[i] {
            c_for!(let mut j: i32 = 0; j < BZP_CHARS_PER_GROUP_ASCII!(); j.suffix_plus_plus(); {
                inUse[i * BZP_GROUPS_ASCII!() + j] = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
            });
        }
    });
    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!(); i.suffix_plus_plus(); {
        if inUse[i] {
            inData.list[ninUse.suffix_plus_plus()] = i.cast();
        }
    });
    return ninUse.cast();
}
```
"""

example_8 = """
Source:
```c
void CmptlzLogWrite(size_t errorCode, const char *funcName, unsigned short line, const char *fmt, ...)
{
    va_list alist;
    char output[LOG_BUF_SIZE];
    int ret;
    size_t len;
    CmptlzLogFunc func = g_cmptlzLogFunc;
    if (func == NULL)
    {
        return;
    }
    ret = snprintf_s(output, LOG_BUF_SIZE, LOG_BUF_SIZE - 1, "\\n[Cmptlz-Log] Func=%s, Line=%u, Error=0x%zx\\n", funcName,
                     line, errorCode);
    if (ret < 0)
    {
        return;
    }
    len = (size_t)ret;
    va_start(alist, fmt);
    ret = vsnprintf_s(output + len, LOG_BUF_SIZE - len, LOG_BUF_SIZE - len - 1, fmt, alist);
    va_end(alist);
    if (ret < 0)
    {
        return;
    }
    func(output, strlen(output) + 1);
}
```

Translation:
```rust
pub fn CmptlzLogWrite(mut errorCode: usize, mut funcName: Ptr<u8>, mut line: u16, mut fmt: Ptr<u8>, mut alist: VaList) {
    // alist already initialized at parameter list
    let mut output: Array<u8, { LOG_BUF_SIZE!() }> = Default::default();
    let mut ret: i32 = Default::default();
    let mut len: usize = Default::default();
    let mut func: CmptlzLogFunc = *g_cmptlzLogFunc.lock();
    if func == NULL!() {
        return;
    }
    ret = c_snprintf_s!(output, LOG_BUF_SIZE!(), LOG_BUF_SIZE!() - 1, cstr!("\\n[Cmptlz-Log] Func={}, Line={}, Error={}\\n"), funcName, line, errorCode);
    if ret < 0 {
        return;
    }
    len = ret.cast();
    // va_start not needed
    ret = c_vsnprintf_s!(output.cast::<Ptr<u8>>() + len, LOG_BUF_SIZE!() - len, LOG_BUF_SIZE!() - len - 1, fmt, alist);
    // va_end not needed
    if ret < 0 {
        return;
    }
    func(output.cast(), c_strlen!(output) + 1);
}
```
"""

example_9 = """
Source:
```c
int CmptLzDecSinglePacket(CmptLzDecCtx *decCtx, size_t dicPosLimit, const unsigned char *pSrcIn, size_t srcInLen,
                          size_t *psrcCostLen)
{
    int res;
    size_t lookAheadLen = 0;
    uint32_t newTempBufSize = decCtx->tempBufSize;
    unsigned char *oldTmpBuf = &(decCtx->tempBuf[decCtx->tempBufSize]);
    while (newTempBufSize < CMPTLZ_REQUIRED_INPUT_MAX && lookAheadLen < srcInLen)
    {
        decCtx->tempBuf[newTempBufSize++] = pSrcIn[lookAheadLen++];
    }
    const unsigned char *bufLimit = decCtx->tempBuf + newTempBufSize;
    res = CmptLzTryDecOnePacket(decCtx, &(decCtx->tempBuf[0]), &bufLimit);
    if (res == CMPTLZ_DEC_INPUT_EOF)
    {
        *psrcCostLen = lookAheadLen;
        decCtx->tempBufSize = newTempBufSize;
        return CMPTLZ_DEC_INPUT_EOF;
    }
    if (res == CMPT_ERROR_DATA)
    {
        return res;
    }
    decCtx->buf = &(decCtx->tempBuf[0]);
    res = CmptLzDecDirectProcess(decCtx, dicPosLimit, bufLimit);
    if ((res != CMPT_OK) || (bufLimit != decCtx->buf) || (bufLimit <= oldTmpBuf))
    {
        *psrcCostLen = 0;
        return CMPT_ERROR_DATA;
    }
    *psrcCostLen = (size_t)(bufLimit - oldTmpBuf);
    decCtx->tempBufSize = 0;
    return res;
}
```

Translation:
```rust
pub fn CmptLzDecSinglePacket(mut decCtx: Ptr<CmptLzDecCtx>, mut dicPosLimit: usize, mut pSrcIn: Ptr<u8>, mut srcInLen: usize, mut psrcCostLen: Ptr<usize>) -> i32 {
    let mut res: i32;
    let mut lookAheadLen: usize = 0;
    let mut newTempBufSize: u32 = decCtx.tempBufSize.cast();
    let mut oldTmpBuf: Ptr<u8> = (c_ref!(decCtx.tempBuf[0]) + decCtx.tempBufSize).cast();
    while newTempBufSize < CMPTLZ_REQUIRED_INPUT_MAX!() && lookAheadLen < srcInLen {
        decCtx.tempBuf[newTempBufSize] = pSrcIn[lookAheadLen].cast();
        newTempBufSize += 1;
        lookAheadLen += 1;
    }
    let mut bufLimit: Ptr<u8> = decCtx.tempBuf.cast::<Ptr<u8>>() + newTempBufSize;
    res = CmptLzTryDecOnePacket(decCtx.cast(), decCtx.tempBuf.cast(), c_ref!(bufLimit).cast()).cast();
    if res == CMPTLZ_DEC_INPUT_EOF!() {
        *psrcCostLen = lookAheadLen.cast();
        decCtx.tempBufSize = newTempBufSize.cast();
        return CMPTLZ_DEC_INPUT_EOF!();
    }
    if res == CMPT_ERROR_DATA!() {
        return res;
    }
    decCtx.buf = c_ref!(decCtx.tempBuf[0]).cast();
    res = CmptLzDecDirectProcess(decCtx.cast(), dicPosLimit.cast(), bufLimit.cast()).cast();
    if res != CMPT_OK!() || bufLimit != decCtx.buf || bufLimit <= oldTmpBuf {
        *psrcCostLen = 0;
        return CMPT_ERROR_DATA!();
    }
    *psrcCostLen = (bufLimit - oldTmpBuf).cast::<usize>();
    decCtx.tempBufSize = 0;
    return res.cast();
}
```
"""

example_10 = """
Source:
```c
BzpHuffmanDecode *BzpHuffmanDecodeInit(int32_t blockSize)
{
    if (BZP_INVALID_BLOCK_SIZE(blockSize))
    {
        return NULL;
    }
    BzpHuffmanDecode *huffman = (BzpHuffmanDecode *)malloc(sizeof(BzpHuffmanDecode));
    if (huffman == NULL)
    {
        return NULL;
    }
    int32_t spaceSize = BZP_BASE_BLOCK_SIZE * blockSize / BZP_ELEMS_NUM_IN_ONE_GROUP;
    huffman->select = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    if (huffman->select == NULL)
    {
        BzpHuffmanDecodeFinish(huffman);
    }
    (void)memset_s(huffman->base, sizeof(huffman->base), 0, sizeof(huffman->base));
    (void)memset_s(huffman->perm, sizeof(huffman->perm), 0, sizeof(huffman->perm));
    (void)memset_s(huffman->limit, sizeof(huffman->limit), 0, sizeof(huffman->limit));
    huffman->selectCnt = 0;
    huffman->deCodeNum = 0;
    return huffman;
}
```

Translation:
```rust
pub fn BzpHuffmanDecodeInit(mut blockSize: i32) -> Ptr<BzpHuffmanDecode> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize) {
        return NULL!();
    }
    let mut huffman: Ptr<BzpHuffmanDecode> = c_malloc!(c_sizeof!(BzpHuffmanDecode));
    if huffman == NULL!() {
        return NULL!();
    }
    let mut spaceSize: i32 = BZP_BASE_BLOCK_SIZE!() * blockSize / BZP_ELEMS_NUM_IN_ONE_GROUP!();
    huffman.select = c_malloc!(spaceSize * c_sizeof!(i32));
    if huffman.select == NULL!() {
        BzpHuffmanDecodeFinish(huffman.cast());
    }
    c_memset_s!(huffman.base, c_sizeofval!(huffman.base), 0, c_sizeofval!(huffman.base)).cast::<Void>();
    c_memset_s!(huffman.perm, c_sizeofval!(huffman.perm), 0, c_sizeofval!(huffman.perm)).cast::<Void>();
    c_memset_s!(huffman.limit, c_sizeofval!(huffman.limit), 0, c_sizeofval!(huffman.limit)).cast::<Void>();
    huffman.selectCnt = 0;
    huffman.deCodeNum = 0;
    return huffman.cast();
}
```
"""

function_text = """
Translate the C Code to Rust. 
You need to translate the function only.
Here are some rules you need to follow:
Type translation: 
    1. Basic types like int, char, unsigned char, uint32_t, etc. should be translated to Rust types: int -> i32, char -> u8, unsigned char -> u8, uint32_t -> u32, etc. 
    2. Pointers in C should be translated to Ptr<T> in Rust, and void* should be translated to Ptr<Void>. char and unsigned char should all be translated to u8, so char* should be translated to Ptr<u8>.
    3. Array in C like int[10] should be translated to `Array` type in Rust: Array<i32, 10>, and you should use arr! macro to initialize the array, for example, `int a[5] = {1, 2, 3, 4, 5};` should be translated to `a: Array<i32, 5> = arr![1, 2, 3, 4, 5];`. Notice that if function has array parameters, translate it to a Ptr<T> type in Rust. For example, `void MyFunction(int a[5])` should be translated to `pub fn MyFunction(mut a: Ptr<i32>)`.
    4. FILE* in C should be translated to FilePtr type in Rust. You can use c_fread!(), f_fwrite!(), c_fopen!(), c_fclose!(), etc. to operate the file.
Operators:
    1. Always use the same operators in C which is available in Rust, like `+`, `-`, `*`, `+=` and `-=`, do not use methods like `.add()` or `.offset()` for pointers. The C `->` operator should be translated to `.` in Rust. For example, `a->b` should be translated to `a.b`.
    2. The `++` and `--` operators are not available in Rust, use .suffix_plus_plus(), .suffix_minus_minus() for suffix increment and decrement, and prefix_plus_plus() and prefix_minus_minus() for prefix increment and decrement. For example, `a++` -> `a.suffix_plus_plus()`, `++a` -> `a.prefix_plus_plus()`. 
    3. The `&` operator has different meaning in Rust, use macro `c_ref!()` instead. For example, `(void*)(&b)` should be translated to `c_ref!(b).cast::<Ptr<Void>>();`.
    4. The `sizeof` operator should be translated to Rust macro `c_sizeof!` for types and `c_sizeofval!` for variables. For example, `sizeof(int)` should be translated to `c_sizeof!(int)`, and `sizeof(my_struct->a)` should be translated to `c_sizeofval!(my_struct.a)`.
Macros vs Functions:
    1. Macros in C should be translated to Rust macros with the same name, also uppercased. For example, `a = MY_MACRO;` should be translated to `a = MY_MACRO!();`, and `a = MY_MACRO(b);` should be translated to `a = MY_MACRO!(b);`. Macros in C should ONLY CONTAIN uppercase letter, digits and underscores, like `MY_MACRO_NUM_2`. Anything contains lowercase letter is a funtion. Also, if some variable with only uppercase letter and digits is indexed, like `K256[a]`, it is not a macro, but a global array. Thus, `int a = K256[1];` should be translated to `let mut a: i32 = K256[1];`, don't use `K256!(1)`.
    2. Non C-builtin Functions in C should be translated to Rust functions with the same name. For example, `a = MY_Func(b);` should be translated to `a = MY_Func(b.cast()).cast();`. C-builtin Functions, like `malloc`, `free`, `strcmp`, `memmove_s`, `memcpy_s`, etc., should be translated to Rust macros with the same name, like `c_malloc!()`, `c_free!()`, `c_strcmp!()`, `c_memmove_s!()`, `c_memcpy_s!()`, etc. For example, `int* a = (int*)malloc(10 * sizeof(int));` should be translated to `let mut a: Ptr<i32> = c_malloc!(10 * c_sizeof!(int));`. They are macros, so don't use .cast() when pass parameters to them or make assignments.
Type casting:
    1. C have implicit type casting and explicit type casting. In Rust, you should use `.cast::<T>()` method for explicit type casting, and use `.cast()` for implicit type casting. 
    2. For example of explicit type casting, `int a = (int)b;` should be translated to `let mut a: i32 = b.cast::<i32>();`, and `b = (int32_t*)((char*)p + 8);` should be translated to `b = (p.cast::<Ptr<u8>>() + 8).cast::<Ptr<i32>>();`. 
    3. For implicit type casting, it exists in three conditions: assignments, function parameters, and return values. For example, `MyFunction(a, b)` should be translated to `MyFunction(a.cast(), b.cast());`, `a = b` should be translated to `a = b.cast();`, and `return a` should be translated to `return a.cast();`. Notice that only pass parameter for functions and assignments need to use .cast() method, for macros, do not use .cast() method. For example, `return MY_MACRO(a, b);` should be translated to `return MY_MACRO!(a, b);`, and `int a = MY_MACRO(b);` should be translated to `let mut a: i32 = MY_MACRO!(b);`.
    4. However, if the passed expression is not a typed value, which means: number literal(like `0` or `0x100`), string literal(like cstr!("abcd")), non-operator macro return value(like `MY_MACRO(a)`) do not need to be casted. For example. `MyFunction(0, a, 1 + MY_NUM, &expr, "abcd", MY_MACRO(b))` should be translated to `MyFunction(0, a.cast(), 1 + MY_NUM!(), c_ref!(expr).cast(), cstr!("abcd").cast(), MY_MACRO!(b));`. c_ref!(), c_sizeof!() and c_sizeofval!() are operator macros, so they should be casted. For example, `MyFunc(sizeof(int), sizeof(st.v), &st)` should be translated to `MyFunc(c_sizeof!(int).cast(), c_sizeofval!(st.v).cast(), c_ref!(st).cast());`.
Logical options:
    1. `for` is not available in Rust, use `c_for!` macro instead. For example, `for (int i = 0; i < 10; i++) {}` should be translated to `c_for!(let mut i = 0; i < 10; i.suffix_plus_plus(); {} );`. Those three parameters in `c_for!` can be empty, like `for(; i < 10;) {}` should be translated to `c_for!(; i < 10; {});`.
    2. `for` with no condition is not available in Rust, use `loop` instead. For example, `for(;;) {}` should be translated to `loop {}`.
    3. `while` is available in Rust, use `while` for `while` loop. For example, `while (a < 10) {}` should be translated to `while a < 10 {}`.
    4. `do while` is not available in Rust, use `c_do!` macro instead. For example, `do { a++; } while (a < 10);` should be translated to `c_do!({ a.suffix_plus_plus(); } while a < 10);`.
    5. `switch` is not available in Rust, use `if` and `else if` instead. For example, `switch(a) { case 1: i |= 1; case 2: i |= 2; break; default: i |= 4; break; }` should be translated to `c_switch!(a, { 1 => { i |= 1; break; }, 2 => { i |= 2; break; }, _ => { i |= 4; break; }, });`.
    6. Ternary operator `? :` is not available in Rust, use `if else` instead. For example, `a = (b > 0) ? 1 : 0;` should be translated to `a = if b > 0 { 1 } else { 0 };`.
    7. Notice that if the condition expression is not a comparison expression('==', '!=', '<', '>', '<=' and '>='), like `a = (b) ? 1 : 0;`, you should use `as_bool()` method to convert it to a boolean value. For example, `a = (b) ? 1 : 0;` shoule be translated to `a = if b.as_bool() { 1 } else { 0 };`.
Others:
    1. Notice that using one field for indexing another field in a struct cause compilation error in Rust. For example, `my_struct.arr[my_struct.field] = 1;` should be translated to `let idx: usize = my_struct.field.cast(); my_struct.arr[idx] = 1;`.
    2. Don't use uninitialized variables in Rust, use Default::default() to initialize the variable. For example, `int a;` should be translated to `let mut a: i32 = Default::default();`.
Now follow these examples for translation:
""" + example_1 + example_2 + example_3 + example_4 + example_5 + example_6 + example_7 + example_8 + example_9 + example_10

def function_prompt(c_code):
    return function_text + f"Now translate the following Function:\n```c\n{c_code.strip()}\n```"


results = {}

def get_our_result_function(value):
    text = function_prompt(value)
    response = client.chat.completions.create(
        model="deepseek-coder",
        messages=[
            {"role": "user", "content": text},
            {"role": "assistant", "content": "Sure, here is the rust translation:\n```rust\n", "prefix": True},
        ],
        stop=["```"],
        temperature=0,
        top_p=0.01,
        max_tokens=4096,
        stream=False
    )
    result = response.choices[0].message.content
    return result

def get_our_results_function(data):
    our_result = []
    results = {}
    with ProcessPool(10) as pool:
        process = {}
        for idx, value in enumerate(data):
            process[idx] = pool.schedule(get_our_result_function, 
                args=(value,))
        results = {}
        for idx, value in enumerate(data):
            results[idx] = process[idx].result()
        results = list(sorted(results.items(), key=lambda item: item[0]))
        for key, value in results:
            our_result.append(value)    
    return our_result