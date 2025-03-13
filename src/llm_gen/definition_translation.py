from openai import OpenAI
from pebble import ProcessPool
from httpx import Client
from tqdm import tqdm
import json

client = OpenAI(
    api_key="sk-76da526dbd8b48c3954df9336a8a6592",
    base_url="https://api.deepseek.com/beta",
    http_client=Client(
        verify=False  # 注意：禁用 SSL 验证可能有安全风险，请根据实际情况决定是否需要这样做
    ),
)

definition_text = """
Translate the C Code to Rust. 
You need to translate the definition only.
Notice that: 
You need to translate `void*` type to VoidPtr in Rust, and all char type to u8.
Array in C like int[10] should be translated to `Array` type in Rust: Array<i32, 10>, and you should use arr! macro to initialize the array, for example, `int a[5] = {1, 2, 3, 4, 5};` should be translated to `a: Array<i32, 5> = arr![1, 2, 3, 4, 5];`.
Enum Type in C should be translated to i32 and the enum values should be translated to macro_rules, and all translated macros in Rust should be uppercased.
Pointers in C should be translated to Ptr<T> in Rust.
Remember when translating macros, add `pub(crate)` to the macro definition to make it visible.
You should translate the global variables start with g_ with Global<T> type and global!() macro, for example, `static int g_a = 0;` should be translated to `pub static g_a: Global<i32> = global!(0);`. However, if it not not start with g_, just translate it to a constant.

Here are some examples:
Source:
```c
typedef void *MY_VALUE;
```
Translation:
```rust
pub type MyValue = VoidPtr;
```
Source:
```
#define MY_NULL 0
```
Translation:
```rust
macro_rules! MY_NULL { () => { NULL!() } }
pub(crate) use MY_NULL;
```

Source:
```c
typedef struct _MyStruct MyStruct;
```

Translation:
```rust
pub type MyStruct = _MyStruct;
```

Source:
```c
typedef enum
{
    MY_RED = 0,
    MY_GREEN,
    MY_BLUE
} MyEnum;
```

Translation:
```rust
pub type MyEnum = i32;
macro_rules! MY_RED { () => { 0 } }
pub(crate) use MY_RED;
macro_rules! MY_GREEN { () => { 1 } }
pub(crate) use MY_GREEN;
macro_rules! MY_BLUE { () => { 2 } }
pub(crate) use MY_BLUE;
```

Source:
```c
typedef int (*MyFunction)(int a, int b);
```

Translation:
```rust
pub type MyFunction = FuncPtr<fn(i32, i32) -> i32>;
```

Source:
```c
typedef void (*ANO_function)(const void* a, char* b);
```

Translation:
```rust
pub type ANO_function = FuncPtr<fn(VoidPtr, Ptr<u8>)>;
```

Source:
```c
static MyFunction g_MyCustomFunc = NULL;
```

Translation:
```rust
pub static g_MyCustomFunc: Global<MyFunction> = global!(NULL!());
```

When translating string literals in C, use cstr! macro

Source:
```c
const char* g_MyGlobalStr = "Hello, World!";
```

Translation:
```rust
pub static g_MyGlobalStr: Global<Ptr<u8>> = global!(cstr!("Hello, World!"));
```

Source:
```c
static MyFunction g_MyCustomFunc = NULL;
```


Source:
```c
int[] g_MyCustomArray = {1, 2, 3, 4, 5};
const int[] myCustomArray = {1, 2, 3, 4, 5};
```

Translation:
```rust
pub static g_MyCustomArray: Global<Array<i32, 5>> = global!(arr![1, 2, 3, 4, 5]);
pub const myCustomArray: Array<i32, 5>> = arr![1, 2, 3, 4, 5];
```

Source:
```c
static int[] g_A10 = {3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
static const int[] A10 = {3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
```

Translation:
```
pub static g_A10: Global<Array<i32, 10>> = global!(arr![3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
pub const A10: Array<i32, 10> = arr![3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
```

Source:
```c
static const int arr_counts = sizeof(arr) / sizeof(int);
```

Translation:
```
pub const arr_counts: i32 = arr.len() as i32;
```

Source:
```c
struct MySimpleStruct {
    int* arr;
    unsigned int length;
};
```

Translation:
```rust
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct MySimpleStruct {
    pub arr: Ptr<i32>,
    pub length: u32,
}
```

If a struct is defined with `typedef struct A { ... } A1;`, add an additional `pub type A1 = A;` in Rust Translation. 

Source:
```c
typedef struct _MyComplexStruct {
    MyStructEntry **vEntries;
    const char* vlength;
    MyStructValueFunc valueFunc;
    int values[64];
    FILE* file;
	MyStructNode *children[CHINDREN_SIZE];
    MyStructNode more_children[CHINDREN_SIZE * 5 + 1];
} MY_Com_Struct;
```

Translation:
```rust
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _MyComplexStruct {
    pub vEntries: Ptr<Ptr<MyStructEntry>>,
    pub vlength: Ptr<u8>,
    pub valueFunc: MyStructValueFunc,
    pub values: Array<i32, 64>,
    pub file: FilePtr,
    pub children: Array<Ptr<MyStructNode>, { CHINDREN_SIZE!() }>,
    pub more_children: Array<MyStructNode, { CHINDREN_SIZE!() * 5 + 1 }>,
}

pub type MY_Com_Struct = _MyComplexStruct;
```

Source:
```c
struct _MySimpleStruct {
    int arr[2];
    unsigned int length;
    MySimpleStruct* ss; 
};
```

Translation:
```rust
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _MySimpleStruct {
    pub arr: Array<i32, 2>,
    pub length: u32,
    pub ss: MySimpleStruct,
}
```
"""


def definition_prompt(c_code):
    return (
        definition_text
        + f"Now translate the following Definition:\n```c\n{c_code.strip()}\n```"
    )


results = {}


def get_our_result_definition(value, cache):
    if value in cache:
        return cache[value]
    text = definition_prompt(value)
    response = client.chat.completions.create(
        model="deepseek-coder",
        messages=[
            {"role": "user", "content": text},
            {
                "role": "assistant",
                "content": "Sure, here is the rust translation:\n```rust\n",
                "prefix": True,
            },
        ],
        stop=["```"],
        temperature=0,
        top_p=0.01,
        max_tokens=4096,
        stream=False,
    )
    result = response.choices[0].message.content
    cache[value] = result
    return result


def get_our_results_definition(data, cache):
    our_result = []
    results = {}
    with ProcessPool(10) as pool:
        process = {}
        for idx, value in enumerate(data):
            process[idx] = pool.schedule(get_our_result_definition, args=(value, cache))
        results = {}
        for idx, value in enumerate(tqdm(data)):
            results[idx] = process[idx].result()
        results = list(sorted(results.items(), key=lambda item: item[0]))
        for key, value in results:
            our_result.append(value)
    return our_result
