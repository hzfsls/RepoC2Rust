from openai import OpenAI
from pebble import ProcessPool
from httpx import Client
from tqdm import tqdm
import json

client = OpenAI(api_key="sk-ea5eeb6b740a435e9a068ec46c594c3f", base_url="https://api.deepseek.com/beta",
    http_client=Client(
    verify=False  # 注意：禁用 SSL 验证可能有安全风险，请根据实际情况决定是否需要这样做
))

example_1 = """
Source:
```c
void VOS_MD5CalcEx(char *output, uint32_t outputLen, const uint8_t *input, uint32_t inputLen)
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
    unimplemented!();
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
    unimplemented!();
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
    unimplemented!();
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
    unimplemented!();
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
    unimplemented!();
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
    unimplemented!();
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
    unimplemented!();
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
    unimplemented!();
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
    unimplemented!();
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
    unimplemented!();
}
```
"""

dummy_function_text = """
Translate the C Code to Rust. 
You need to translate the function to a dummy function with unimplemented!() macro only.
Here are some rules you need to follow:
Type translation: 
    1. Basic types like int, char, unsigned char, uint32_t, etc. should be translated to Rust types: int -> i32, char -> u8, unsigned char -> u8, uint32_t -> u32, etc. 
    2. Pointers in C should be translated to Ptr<T> in Rust, and void* should be translated to Ptr<Void>. char and unsigned char should all be translated to u8, so char* should be translated to Ptr<u8>.
    3. If function has array parameters, translate it to a Ptr<T> type in Rust. For example, `void MyFunction(int a[5])` should be translated to `pub fn MyFunction(mut a: Ptr<i32>)`.
    4. FILE* in C should be translated to FilePtr type in Rust.
Now follow these examples for translation:
""" + example_1 + example_2 + example_3 + example_4 + example_5 + example_6 + example_7 + example_8 + example_9 + example_10

def dummy_function_prompt(c_code):
    return dummy_function_text + f"Now translate the following Function:\n```c\n{c_code.strip()}\n```"


results = {}

def get_our_result_dummy_function(value, cache):
    if value in cache:
        return cache[value]
    text = dummy_function_prompt(value)
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
    cache[value] = result
    return result

def get_our_results_dummy_function(data, cache):
    our_result = []
    results = {}
    with ProcessPool(10) as pool:
        process = {}
        for idx, value in enumerate(data):
            process[idx] = pool.schedule(get_our_result_function, 
                args=(value, cache))
        results = {}
        for idx, value in enumerate(tqdm(data)):
            results[idx] = process[idx].result()
        results = list(sorted(results.items(), key=lambda item: item[0]))
        for key, value in results:
            our_result.append(value)    
    return our_result