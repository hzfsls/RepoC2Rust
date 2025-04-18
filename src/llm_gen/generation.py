from openai import OpenAI
from pebble import ProcessPool
from httpx import Client
from tqdm import tqdm

from misc.exceptions import CallLLMTimeoutError

from rust_metadata.classes import RustCode

delim_repair_prompt = """\
Fix the compiler bugs in the following Rust code with provided compiler error messagesm, possibly because of mismatched parens.
Only fix lines that have unmatched parens bugs, don't modify any other code.

Source:
```rust
pub fn rb_tree_insert_case4(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    let mut next_node: Ptr<RBTreeNode> = Default::default();
    let mut side: RBTreeNodeSide = Default::default();
    
    side = rb_tree_node_side(node);
    if (side != rb_tree_node_side(node.parent) {
        next_node = node.parent;
        rb_tree_rotate(tree, node.parent, (1 - side));
    } else {
        next_node = node;
    }
    rb_tree_insert_case5(tree, next_node);
}
```

Error Message:
"error: mismatched closing delimiter: `}`   --> src/src/rb_tree_c.rs:154:8\n    |\n148 | pub fn rb_tree_insert_case4(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {\n    |                                                                               - closing delimiter possibly meant for this\n...\n154 |     if (side != rb_tree_node_side(node.parent).cast().as_bool() {\n    |        ^ unclosed delimiter\n...\n163 | }\n    | ^ mismatched closing delimiter\n\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn rb_tree_insert_case4(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    let mut next_node: Ptr<RBTreeNode> = Default::default();
    let mut side: RBTreeNodeSide = Default::default();
    
    side = rb_tree_node_side(node);
    if (side != rb_tree_node_side(node.parent)) {
        next_node = node.parent;
        rb_tree_rotate(tree, node.parent, (1 - side));
    } else {
        next_node = node;
    }
    rb_tree_insert_case5(tree, next_node);
}
```

Source:
```rust
pub fn rb_tree_insert(mut tree: Ptr<RBTree>, mut key: RBTreeKey, mut value: RBTreeValue) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode>;
    let mut rover: Ptr<Ptr<RBTreeNode>>;
    let mut parent: Ptr<RBTreeNode>;
    let mut side: RBTreeNodeSide;
    
    node = c_malloc!(c_sizeof!(RBTreeNode));
    
    if (node == NULL!()) {
        return NULL!();
    }
    node.key = key;
    node.value = value;
    node.color = RB_TREE_NODE_RED!();
    node.children[RB_TREE_NODE_LEFT!()] = NULL!();
    node.children[RB_TREE_NODE_RIGHT!()] = NULL!();
    
    parent = NULL!();
    rover = c_ref!(tree.root_node);
    
    while (*rover != NULL!()) {
        parent = *rover;
        if (tree.compare_func(value, (*rover).value) < 0 {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }
        
        rover = c_ref!((*rover).children[side]);
    }
    
    *rover = node;
    node.parent = parent;
    rb_tree_insert_case1(tree, node);
    tree.num_nodes.prefix_plus_plus();
    return node;
}
```

Error Message:
"error: mismatched closing delimiter: `}`\n   --> src/src/rb_tree_c.rs:194:12\n    |\n191 |     while (*rover != NULL!()).as_bool() {\n    |                                         - closing delimiter possibly meant for this\n...\n194 |         if (tree.compare_func(value.cast(), (*rover).value.cast()) < 0 {\n    |            ^ unclosed delimiter\n...\n201 |     }\n    |     ^ mismatched closing delimiter\n\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn rb_tree_insert(mut tree: Ptr<RBTree>, mut key: RBTreeKey, mut value: RBTreeValue) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode>;
    let mut rover: Ptr<Ptr<RBTreeNode>>;
    let mut parent: Ptr<RBTreeNode>;
    let mut side: RBTreeNodeSide;
    
    node = c_malloc!(c_sizeof!(RBTreeNode));
    
    if (node == NULL!()) {
        return NULL!();
    }
    node.key = key;
    node.value = value;
    node.color = RB_TREE_NODE_RED!();
    node.children[RB_TREE_NODE_LEFT!()] = NULL!();
    node.children[RB_TREE_NODE_RIGHT!()] = NULL!();
    
    parent = NULL!();
    rover = c_ref!(tree.root_node);
    
    while (*rover != NULL!()) {
        parent = *rover;
        if (tree.compare_func(value, (*rover).value) < 0) {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }
        
        rover = c_ref!((*rover).children[side]);
    }
    
    *rover = node;
    node.parent = parent;
    rb_tree_insert_case1(tree, node);
    tree.num_nodes.prefix_plus_plus();
    return node;
}
```
"""


repair_prompt = """\
Fix the compiler bugs in the following Rust code with provided compiler error messagesm.
Fix these bugs according to the compiler informations:
1. Type mismatch: use `cast::<T>` to cast to original type to the targeted type.
2. Wrong function as struct field calling: `my_struct.my_func(a, b)` should be corrected as `(my_struct.my_func)(a, b)`.
3. Constant/Macro confusion: `a > MY_MACRO` should be `a > MY_MACRO!()`, and `b > my_constant!()` should be `b > my_constant`
4. Other bugs, just repair the corresponding line with the reference of error messages. 

Source:
```rust
pub fn binomial_tree_unref(mut tree: Ptr<BinomialTree>) {
    let mut i: i32 = Default::default();

    if (tree == NULL!()) {
        return;
    }
    
    tree.refcount.suffix_minus_minus()
    
    if (tree.refcount == 0) {
        c_for!(i = 0; i < tree.order; i.prefix_plus_plus(); {
            binomial_tree_unref(tree.subtrees[i]);
        });
        c_free!(tree.subtrees);
        c_free!(tree);
    }
}
```

Error Message:
"error[E0308]: mismatched types\n  --> src/src/binomial_heap_c.rs:55:27\n   |\n55 |         c_for!(i = 0; i < tree.order; i.prefix_plus_plus(); {\n   |                       -   ^^^^^^^^^^ expected `i32`, found `u16`\n   |                       |\n   |                       expected because this is `i32`\n   |\nhelp: you can convert a `u16` to an `i32`\n   |\n55 |         c_for!(i = 0; i < tree.order.into(); i.prefix_plus_plus(); {\n   |                                     +++++++\n\nFor more information about this error, try `rustc --explain E0308`.\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn binomial_tree_unref(mut tree: Ptr<BinomialTree>) {
    let mut i: i32 = Default::default();

    if (tree == NULL!()) {
        return;
    }
    
    tree.refcount.suffix_minus_minus()
    
    if (tree.refcount == 0) {
        c_for!(i = 0; i < tree.order.cast::<i32>(); i.prefix_plus_plus(); {
            binomial_tree_unref(tree.subtrees[i]);
        });
        c_free!(tree.subtrees);
        c_free!(tree);
    }
}
```

Source:
```rust
pub fn rb_tree_insert(mut tree: Ptr<RBTree>, mut key: RBTreeKey, mut value: RBTreeValue) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode>;
    let mut rover: Ptr<Ptr<RBTreeNode>>;
    let mut parent: Ptr<RBTreeNode>;
    let mut side: RBTreeNodeSide;

    node = c_malloc!(c_sizeof!(RBTreeNode));

    if (node == NULL!()) {
        return NULL!();
    }

    node.key = key;
    node.value = value;
    node.color = RB_TREE_NODE_RED!();
    node.children[RB_TREE_NODE_LEFT!()] = NULL!();
    node.children[RB_TREE_NODE_RIGHT!()] = NULL!();

    parent = NULL!();
    rover = c_ref!(tree.root_node);

    while (*rover != NULL!()) {
        parent = *rover;

        if (tree.compare_func(value, (*rover).value) < 0) {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }

        rover = c_ref!((*rover).children[side]);
    }

    *rover = node;
    node.parent = parent;

    rb_tree_insert_case1(tree, node);

    tree.num_nodes.prefix_plus_plus();

    return node;
}
```

Error Message:
"error: mismatched closing delimiter: `}`\n   --> src/src/rb_tree_c.rs:194:12\n    |\n191 |     while (*rover != NULL!()).as_bool() {\n    |                                         - closing delimiter possibly meant for this\n...\n194 |         if (tree.compare_func(value.cast(), (*rover).value.cast()) < 0 {\n    |            ^ unclosed delimiter\n...\n201 |     }\n    |     ^ mismatched closing delimiter\n\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn rb_tree_insert(mut tree: Ptr<RBTree>, mut key: RBTreeKey, mut value: RBTreeValue) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode>;
    let mut rover: Ptr<Ptr<RBTreeNode>>;
    let mut parent: Ptr<RBTreeNode>;
    let mut side: RBTreeNodeSide;

    node = c_malloc!(c_sizeof!(RBTreeNode));

    if (node == NULL!()) {
        return NULL!();
    }

    node.key = key;
    node.value = value;
    node.color = RB_TREE_NODE_RED!();
    node.children[RB_TREE_NODE_LEFT!()] = NULL!();
    node.children[RB_TREE_NODE_RIGHT!()] = NULL!();

    parent = NULL!();
    rover = c_ref!(tree.root_node);

    while (*rover != NULL!()) {
        parent = *rover;

        if ((tree.compare_func)(value, (*rover).value) < 0) {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }

        rover = c_ref!((*rover).children[side]);
    }

    *rover = node;
    node.parent = parent;

    rb_tree_insert_case1(tree, node);

    tree.num_nodes.prefix_plus_plus();

    return node;
}
```

Source:
```rust
pub fn string_hash(mut string: Ptr<Void>) -> u32 {
    let mut result: u32 = 5381;
    let mut p: Ptr<u8> = string.cast::<Ptr<u8>>();

    while (*p != '\\0') {
        result = (result << 5) + result + (*p).cast::<u32>();
        p += 1;
    }

    return result;
}
```

Error Message:
"error[E0308]: mismatched types\n --> src/src/hash_string_c.rs:8:18\n  |\n8 |     while (*p != '\\0') {\n  |            --    ^^^^ expected `u8`, found `char`\n  |            |\n  |            expected because this is `u8`\n  |\nhelp: if you meant to write a byte literal, prefix with `b`\n  |\n8 |     while (*p != b'\\0') {\n  |                  ~~~~~\n\nFor more information about this error, try `rustc --explain E0308`.\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn string_hash(mut string: Ptr<Void>) -> u32 {
    let mut result: u32 = 5381;
    let mut p: Ptr<u8> = string.cast::<Ptr<u8>>();

    while (*p != b'\\0' as u8) {
        result = (result << 5) + result + (*p).cast::<u32>();
        p += 1;
    }

    return result;
}
```

Source:
```rust
pub fn trie_free_list_pop(mut list: Ptr<Ptr<TrieNode>>) -> Ptr<TrieNode> {
    let mut result: Ptr<TrieNode>;

    result = *list;
    *list = result.data;

    return result;
}
```

Error Message:
"error[E0308]: mismatched types\n  --> src/src/trie_c.rs:46:13\n   |\n46 |     *list = result.data;\n   |     -----   ^^^^^^^^^^^ expected `Ptr<_TrieNode>`, found `Ptr<u8>`\n   |     |\n   |     expected due to the type of this binding\n   |\n   = note: expected struct `memory::ptr::Ptr<_TrieNode>`\n              found struct `memory::ptr::Ptr<u8>`\nhelp: consider removing the tuple struct field `data`\n   |\n46 -     *list = result.data;\n46 +     *list = result;\n   |\n\nFor more information about this error, try `rustc --explain E0308`.\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn trie_free_list_pop(mut list: Ptr<Ptr<TrieNode>>) -> Ptr<TrieNode> {
    let mut result: Ptr<TrieNode>;

    result = *list;
    *list = result.data.cast::<Ptr<TrieNode>>();

    return result;
}
```

Source:
```rust
pub fn set_allocate_table(mut set: Ptr<Set>) -> i32 {
    if (set.prime_index < set_num_primes!()) {
        let tmp0 = set.prime_index;
        set.table_size = set_primes[tmp0];
    } else {
        set.table_size = (set.entries * 10);
    }
    set.table = c_calloc!(set.table_size, c_sizeof!(Ptr<SetEntry>));
    return (set.table != NULL!()).cast::<i32>();
}
```

Error Message:
"error: cannot find macro `set_num_primes` in this scope\n  --> src/src/set_c.rs:35:27\n   |\n35 |     if (set.prime_index < set_num_primes!()) {\n   |                           ^^^^^^^^^^^^^^\n   |\n   = note: `set_num_primes` is in scope, but it is a constant, not a macro\n\nerror: could not compile `my_proj` (lib) due to 1 previous error\n"

Fixed Code:
```rust
pub fn set_allocate_table(mut set: Ptr<Set>) -> i32 {
    if (set.prime_index < set_num_primes) {
        let tmp0 = set.prime_index;
        set.table_size = set_primes[tmp0];
    } else {
        set.table_size = (set.entries * 10);
    }
    set.table = c_calloc!(set.table_size, c_sizeof!(Ptr<SetEntry>));
    return (set.table != NULL!()).cast::<i32>();
}
```
"""


class GenerationClient:
    def __init__(self, api_key, base_url="https://api.deepseek.com/beta", proxy=None):
        self.api_key = api_key
        self.base_url = base_url
        self.proxy = proxy
    
    def get_response(self, text):
        openai_client = OpenAI(
            api_key=self.api_key,
            base_url=self.base_url,
            http_client=Client(
                proxy=self.proxy,
                verify=False
            )
        )
        response = openai_client.chat.completions.create(
            model="deepseek-coder",
            messages=[
                {"role": "assistant", "content": text, "prefix": True},
                # {
                #     "role": "assistant",
                #     "content": "Sure, here is the rust translation:\n```rust\n",
                #     "prefix": True,
                # },
            ],
            stop=["```"],
            temperature=0,
            top_p=0.01,
            max_tokens=4096,
            stream=False,
        )
        result = response.choices[0].message.content
        return result

def merge_prompt(prompt, c_code):
    return prompt + f"""\
Source:
```c
{c_code.strip()}
```

Translation:
```rust
"""

def merge_repair_prompt(prompt, c_code, compiler_msg):
    return prompt + f"""\
Source:
```rust
{c_code.strip()}
```

Error Message:{compiler_msg}

Fixed Code:
```rust
"""

def get_delim_repair_candidates(client, code, compiler_msg):
    text = merge_repair_prompt(delim_repair_prompt, code, compiler_msg)
    response = client.get_response(text)
    return [response]


def get_repair_candidates(client, code, compiler_msg):
    text = merge_repair_prompt(repair_prompt, code, compiler_msg)
    response = client.get_response(text)
    return [response]

def get_llm_generated_result(client, code, prompt, cache = {}):
    if code in cache:
        return cache[code]
    text = merge_prompt(prompt, code)
    response = client.get_response(text)
    return response

def update_codes(type, client, prompts, codes: list[RustCode], caches = {}):
    cache = caches[type]
    prompt = prompts[type]
    # for c in tqdm(codes):
    #     c.rust_code = get_llm_generated_result(client, c.c_code, prompt, cache.cache)
    #     print(c.rust_code)
    #     cache.update(c.c_code, c.rust_code)
    if len(codes) == 1:
        c = codes[0]
        c.rust_code = get_llm_generated_result(client, c.c_code, prompt, cache.cache)
        cache.update(c.c_code, c.rust_code)
    else:
        with ProcessPool(max_workers=5) as pool:
            futures = []
            for c in codes:
                future = pool.schedule(
                    get_llm_generated_result, args=[client, c.c_code, prompt, cache.cache], timeout=600
                )
                futures.append((c, future))
            for c, future in tqdm(futures):
                try:
                    rust_code = future.result()
                    c.rust_code = rust_code
                    cache.update(c.c_code, c.rust_code)
                except Exception as e:
                    raise CallLLMTimeoutError(e)