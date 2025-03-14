from openai import OpenAI
from pebble import ProcessPool
from httpx import Client
from tqdm import tqdm

from misc.exceptions import CallLLMTimeoutError

from rust_metadata.classes import RustCode
# client = OpenAI(
#     api_key="sk-76da526dbd8b48c3954df9336a8a6592",
#     base_url="https://api.deepseek.com/beta",
#     http_client=Client(
#         verify=False  # 注意：禁用 SSL 验证可能有安全风险，请根据实际情况决定是否需要这样做
#     ),
# )

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
        return result

def merge_prompt(prompt, c_code):
    return prompt + f"Now translate the following Function:\n```c\n{c_code.strip()}\n```"

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
                    get_llm_generated_result, args=[client, c.c_code, prompt, cache.cache], timeout=300
                )
                futures.append((c, future))
            for c, future in tqdm(futures):
                try:
                    rust_code = future.result()
                    c.rust_code = rust_code
                    cache.update(c.c_code, c.rust_code)
                except Exception as e:
                    raise CallLLMTimeoutError(e)