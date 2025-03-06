from openai import OpenAI
from pebble import ProcessPool
from httpx import Client

import json

client = OpenAI(api_key="sk-ea5eeb6b740a435e9a068ec46c594c3f", base_url="https://api.deepseek.com/beta",
    http_client=Client(
    verify=False  # 注意：禁用 SSL 验证可能有安全风险，请根据实际情况决定是否需要这样做
))

macro_text = """
Translate the C Code to Rust. 
You need to translate the macro only.
Notice that: 
Remember when translating macros, add `pub(crate)` to the macro definition to make it visible.
When using another macros in C, you should use macros in Rust with the same name. For example, `#define MY_MACRO ANOTHER_MACRO(ANOTHER_MACRO2)` should be translated to `macro_rules! MY_MACRO { () => { ANOTHER_MACRO!(ANOTHER_MACRO2!()) } }\n pub(crate) use MY_MACRO;`. Notice that C original macros, like __FILE__ and __LINE__, should also be translated to __FILE__!() and __LINE__!() in Rust.
Any expression as a condition of `if` or `while` or ternary operator should be append with `.as_bool()` method in Rust. For example, `(MyFunc(1)) ? 1 : 0` should be translated to `if MyFunc(1).as_bool() { 1 } else { 0 }`.
When using the C string literals, use cstr! macro to translate it to Rust. For example, `#define MY_STR "Hello, World!"` should be translated to `macro_rules! MY_STR { () => { cstr!("Hello, World!") } }\n pub(crate) use MY_STR;`.
When translating a macro that represents NULL pointer, use NULL!() macro in Rust. For example, `#define MY_NULL 0` should be translated to `macro_rules! MY_NULL { () => { NULL!() } }\n pub(crate) use MY_NULL;`. If it represents true or false, just use the numbers in C, for example, `#define MY_TRUE 1` should be translated to `macro_rules! MY_TRUE { () => { 1 } }\n pub(crate) use MY_TRUE;`, and `#define MY_FALSE 0` should be translated to `macro_rules! MY_FALSE { () => { 0 } }\n pub(crate) use MY_FALSE;`.
When using global variables that start with `g_`, use the lock() method to get reference of the variable. For example, `int a = g_a;` should be translated to `let mut a = (*g_a.lock());`, and `g_a = 10;` should be translated to `(*g_a.lock()) = 10;`.
When find a C reference operator `&`, use c_ref! macro to translate it to Rust. For example, `int a = &b;` should be translated to `let mut a = c_ref!(b);`.
If the original number literal in C has type, like `10U`, just ignore it and translate to Rust number without types directly, i.e. `10`. For example, `#define MY_NUM 10U` should be translated to `macro_rules! MY_NUM { () => { 10 } }\n pub(crate) use MY_NUM;`. However, if it is explicitly casted, like `(size_t)10`, you should translate it to Rust with the cast, with original `isize` literal, i.e. `(10isize as usize)`. 

Source:
```c
#define MY_NUMA 10U
```

Translation:
```rust
macro_rules! MY_NUMA { () => { 10 } }
pub(crate) use MY_NUMA;
```

Source:
```c
#define MY_NUMB 10LL
```

Translation:
```rust
macro_rules! MY_NUMB { () => { 10 } }
pub(crate) use MY_NUMB;
```

Source:
```c
#define MY_FFFF 0xffffL
```

Translation:
```rust
macro_rules! MY_FFFF { () => { 0xffff } }
pub(crate) use MY_FFFF;
```

Source:
```c
#define MYHEX 0x30
```

Translation:
```rust
macro_rules! MYHEX { () => { 0x30 } }
pub(crate) use MYHEX;
```

Source:
```c
#define MY_MALLOC_8 malloc(8)
```

Translation:
```rust
macro_rules! MY_MALLOC_8 { () => { malloc(8) } }
pub(crate) use MY_MALLOC_8;
```

Source:
```c
#define MY_NULL 0
```

Translation:
```rust
macro_rules! MY_NULL { () => { NULL!() } }
pub(crate) use MY_NULL;
```

Source:
```c
#define MY_EOK 0
```

Translation:
```rust
macro_rules! MY_EOK { () => { 0 } }
pub(crate) use MY_EOK;
```

Source:
```c
#define MY_NEG (uint8_t)~0
```

Translation:
```rust
macro_rules! MY_NEG { () => { (!0isize) as u8 } }
pub(crate) use MY_NEG;
```

Source:
```c
#define MY_SELECTION (MY_CONDITION!()) ? MY_SELECTION_TRUE : MY_SELECTION_FALSE
```

Translation:
```rust
macro_rules! MY_SELECTION { () => { if MY_CONDITION!().as_bool() > 10 { MY_SELECTION_TRUE!() } else { MY_SELECTION_FALSE!() } } }
pub(crate) use MY_SELECTION;
```

Source:
```c
#define MY_U8_MINUS (uint8_t)-1
```

Translation:
```rust
macro_rules! MY_U8_MINUS { () => { (-1isize) as u8 } }
```

Source:
```c
#define MY_CALL_FUNC_WITH_CHAR My_calledFunc('a')
```

Translation:
```rust
macro_rules! MY_CALL_FUNC_WITH_CHAR { () => { My_calledFunc(b'a' as u8) } }
```

"""

# Source:
# ```c
# #define PTR_PLUSPLUS(dst) (dst)++
# ```

# Translation:
# ```rust
# macro_rules! PTR_PLUSPLUS { ($dst:expr) => { $dst.plus_plus() } }
# pub(crate) use PTR_PLUSPLUS;
# ```

# Source:
# ```c
# #define PTR_SUB(dst, src) ((dst) - (src))
# ```

# Translation:
# ```rust
# macro_rules! PTR_SUB { ($dst:expr, $src:expr) => { $dst - $src } }
# pub(crate) use PTR_SUB;
# ```

# Source:
# ```c
# #define PTR_COMPLEX(ptr1, var, ptr2, var2)      \
#     do                                          \
#     {                                           \
#         *(ptr1)++ = (int32_t)(var);             \
#         (ptr2) = (uint16_t *)((ptr1) + 8);      \
#         (var2) = (ptr2) - (ptr1)                \
#         (ptr1) = &(ptr)[(var2) * 2];            \
#     } while (0)
# ```

# Translation:
# ```rust
# macro_rules! PTR_COMPLEX { ($ptr1:expr, $var:expr, $ptr2:expr, $var2:expr) => {
#     *$ptr1.plus_plus() = $var.cast::<i32>();
#     $ptr2 = ($ptr1 + 8).cast::<Ptr<u16>>();
#     $var2 = $ptr2 - $ptr1;
#     $ptr1 = c_ref!(($ptr)[($var2 * 2)]);
# }}
# ```



# Source:
# ```c
# #define MY_VARGS_M(error_code, fmt, args...)                                                                           \
#     do                                                                                                                 \
#     {                                                                                                                  \
#         MyVargsFunc(error_code, fmt, ##args);                                                                           \
#     } while (0)
# ```

# Translation:
# ```rust
# // when translate a variadic macro, you need to use the following pattern
# macro_rules! MY_VARGS_M {
#     ($error_code:expr, $fmt:expr) => {
#         MyVargsFunc($error_code, $fmt, &[]);
#     }
#     ($error_code:expr, $fmt:expr, $($args:expr),*) => {
#         MyVargsFunc($error_code, $fmt, &[$(&$args), *]);
#     }
# }
# ```
# """

def macro_prompt(c_code):
    return macro_text + f"Now translate the following Macro:\n```c\n{c_code.strip()}\n```"


results = {}

def get_our_result_macro(value):
    text = macro_prompt(value)
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

def get_our_results_macro(data):
    our_result = []
    results = {}
    with ProcessPool(10) as pool:
        process = {}
        for idx, value in enumerate(data):
            process[idx] = pool.schedule(get_our_result_macro, 
                args=(value,))
        results = {}
        for idx, value in enumerate(data):
            results[idx] = process[idx].result()
        results = list(sorted(results.items(), key=lambda item: item[0]))
        for key, value in results:
            our_result.append(value)    
    return our_result