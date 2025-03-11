import os
import json

from c_metadata.c_metadata import get_metadata
from c_metadata.preprocess import preprocess
from rust_metadata.rust_metadata import resolve_metadata
from rust_metadata.rust_project_creation import RustProject, RustProjectMetadata
from rust_metadata.classes import RustFile
from tqdm import tqdm


from llm_gen.definition_translation import get_our_results_definition, get_our_result_definition
from llm_gen.function_translation import get_our_results_function, get_our_result_function
from llm_gen.macro_translation import get_our_results_macro, get_our_result_macro
from llm_gen.macro_function_translation import get_our_results_macro_function, get_our_result_macro_function
from llm_gen.dummy_function_translation import get_our_results_dummy_function, get_our_result_dummy_function

project_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), "data")
c_metadata_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), "c_metadata")
rust_metadata_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), "rust_metadata")
project_template_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), "project_template")
created_project_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), "created_project")


global_cache_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), "cache")


class Cache:
    def __init__(self, name):
        self.path = os.path.join(global_cache_dir, name)
        self.cache_index = {}
        self.cache = {}
        if not os.path.exists(self.path):
            os.makedirs(self.path)
            with open(os.path.join(self.path, f"index.json"), "w") as f:
                json.dump({}, f)        
        else:
            with open(os.path.join(self.path, f"index.json"), "r") as f:
                self.cache_index = json.load(f)
            for k, path in self.cache_index.items():
                with open(os.path.join(self.path, path, "result.rs"), "r") as f:
                    self.cache[k] = f.read()
    
    def update(self, key, value):
        if key in self.cache_index:
            new_idx = self.cache_index[key]
        else:
            new_idx = str(len(self.cache))
            os.makedirs(os.path.join(self.path, str(new_idx)))
        self.cache_index[key] = new_idx
        self.cache[key] = value        
        with open(os.path.join(self.path, str(new_idx), "result.rs"), "w") as f:
            f.write(value)
        with open(os.path.join(self.path, f"index.json"), "w") as f:
            json.dump(self.cache_index, f)

definition_cache = Cache("definition")
macro_cache = Cache("macro")
macro_function_cache = Cache("macro_function")
dummy_function_cache = Cache("dummy_function")
function_cache = Cache("function")


def update_definitions(files: list[RustFile]):
    for f in tqdm(files):
        f.rust_code = get_our_result_definition(f.c_code, definition_cache.cache)
        definition_cache.update(f.c_code, f.rust_code)

def update_macros(files: list[RustFile]):
    for f in tqdm(files):
        f.rust_code = get_our_result_macro(f.c_code, macro_cache.cache)
        macro_cache.update(f.c_code, f.rust_code)

def update_macro_functions(files: list[RustFile]):
    for f in tqdm(files):
        f.rust_code = get_our_result_macro_function(f.c_code, macro_function_cache.cache)
        macro_function_cache.update(f.c_code, f.rust_code)

def update_functions(files: list[RustFile]):
    for f in tqdm(files):
        f.rust_code = get_our_result_function(f.c_code, function_cache.cache)
        function_cache.update(f.c_code, f.rust_code)

def update_dummy_functions(files: list[RustFile]):
    for f in tqdm(files):
        f.dummy_code = get_our_result_dummy_function(f.c_code, dummy_function_cache.cache)
        dummy_function_cache.update(f.c_code, f.dummy_code)

if __name__ == "__main__":
    for project_name in ["bzp"]:
        files = preprocess(os.path.join(project_dir, project_name), ["include", "src"],
            macros = {"ALWAYS_INLINE": "inline", "ALWAYS_NO_INLINE": "", "STATIC": "static", "HIDDEN": "", "CMPTLZ_HIDDEN":"", "TARGET_ATTRIBUTE_AUTO":"",
                "RAPIDLZ_ALWAYS_INLINE": "inline", "CSTL_STATIC": "static", "DT_EXPORT": ""},
            replacements = {
                "__asm": "asm",
                "args...": "...",
                "##args": "__VA_ARGS__",
                "#if __cplusplus\nextern \"C\" {\n#endif": "extern \"C\" {\n",
                "#if __cplusplus\n}\n#endif": "}\n",
                "#if __cplusplus\n}\n\n#endif": "}\n",
            })
        metadata = get_metadata(files)
        functions = {}
        for f in metadata:
            for func in metadata[f].functions:
                if "main" not in func and "test" not in func and len(func) < 4096:
                    functions[func] = f
        os.makedirs(os.path.join(c_metadata_dir, project_name), exist_ok=True)
        print(f"Project `{project_name}` resolve succeeded!")
        with open(os.path.join(c_metadata_dir, project_name, "files.json"), "w") as f:
            f.write(json.dumps(metadata, default=lambda o: o.__dict__(), indent=4, ensure_ascii=False))
        with open(os.path.join(c_metadata_dir, project_name, "functions.json"), "w") as f:
            f.write(json.dumps(functions, indent=4, ensure_ascii=False))
        with open(os.path.join(c_metadata_dir, project_name, "files.json"), "r") as f:
            files_data = json.load(f)
        with open(os.path.join(c_metadata_dir, project_name, "functions.json"), "r") as f:
            functions_data = json.load(f)
        metadata = resolve_metadata(files_data, functions_data)
        os.makedirs(os.path.join(rust_metadata_dir, project_name), exist_ok=True)
        with open(os.path.join(rust_metadata_dir, project_name, "metadata.json"), "w") as f:
            json.dump(metadata.__dict__(), f, indent=4)
        with open(os.path.join(rust_metadata_dir, project_name, "metadata.json"), "r") as f:
            files_data = json.load(f)
        metadata = RustProjectMetadata.from_dict(files_data)

        proj = RustProject(project_name, metadata)
        print(f"Create rust project `{project_name}` at {proj.dir_path}")
        success, error_msg = proj.build_project()
        if success:
            print(f"{project_name} build succeed!")
        else:
            print(f"{project_name} build fail!")
            print(error_msg)
            exit(0)

        update_macros(proj.metadata.get_all("macro"))
        proj = RustProject(project_name, metadata)
        print(f"Create rust project `{project_name}` with updated macro at {proj.dir_path}")

        success, error_msg = proj.build_project()
        if success:
            print(f"{project_name} build succeed!")
        else:
            print(f"{project_name} build fail!")
            print(error_msg)
            exit(0)

        update_macro_functions(proj.metadata.get_all("macro_function"))
        proj = RustProject(project_name, metadata)
        print(f"Create rust project `{project_name}` with updated macro function at {proj.dir_path}")

        success, error_msg = proj.build_project()
        if success:
            print(f"{project_name} build succeed!")
        else:
            print(f"{project_name} build fail!")
            print(error_msg)
            exit(0)
    
        update_definitions(proj.metadata.get_all("definition"))
        proj = RustProject(project_name, metadata)
        print(f"Create rust project `{project_name}` with updated definition at {proj.dir_path}")

        success, error_msg = proj.build_project()
        if success:
            print(f"{project_name} build succeed!")
        else:
            print(f"{project_name} build fail!")
            print(error_msg)
            exit(0)

        
        update_dummy_functions(proj.metadata.get_all("function"))
        proj = RustProject(project_name, metadata)
        print(f"Create rust project `{project_name}` with updated dummy function at {proj.dir_path}")

        success, error_msg = proj.build_project()
        if success:
            print(f"{project_name} build succeed!")
        else:
            print(f"{project_name} build fail!")
            print(error_msg)
            exit(0)

        functions = proj.metadata.get_all("function")
        for function in functions:
            update_functions([function])
            proj = RustProject(project_name, metadata, "full")
            print(f"Create rust project `{project_name}` with updated function at {proj.dir_path}")

            success, error_msg = proj.build_project()
            if success:
                print(f"{project_name} build succeed!")
            else:
                function_cache_index = function_cache.cache_index[function.c_code]
                print(f"{project_name} build fail! function cache index: {function_cache_index}")
                print(error_msg)
                exit(0)


