import os
import json

from tqdm import tqdm
from pebble import ProcessPool

from c_metadata.c_metadata import get_metadata
from c_metadata.preprocess import preprocess
from rust_metadata.rust_metadata import resolve_metadata
from rust_metadata.rust_project_creation import RustProject, RustProjectMetadata
from rust_metadata.classes import RustCode

from llm_gen.definition_translation import get_our_result_definition
from llm_gen.function_translation import get_our_result_function
from llm_gen.macro_translation import get_our_result_macro
from llm_gen.macro_function_translation import get_our_result_macro_function
from llm_gen.dummy_function_translation import get_our_result_dummy_function

from optimization_agent.classes import OptimizationAgent

import argparse

PARENT_DIR = os.path.dirname(os.path.dirname(__file__))

project_dir = ""
c_metadata_dir = ""
rust_metadata_dir = ""
project_template_dir = ""
created_project_dir = ""
global_cache_dir = ""


def implicit_casting_removal(code):
    ret = []
    sub = ".cast()"
    start = 0
    while True:
        start = code.find(sub, start)
        if start == -1:
            break
        new_code = code[:start] + code[start + len(sub):]
        ret.append(new_code)
        start += len(sub)
    return ret


def as_bool_removal(code):
    ret = []
    sub = ".as_bool()"
    start = 0
    while True:
        start = code.find(sub, start)
        if start == -1:
            break
        new_code = code[:start] + code[start + len(sub):]
        ret.append(new_code)
        start += len(sub)
    return ret


def struct_index_advancement(code):
    import re

    code_lines = code.split("\n")
    ret = []
    for i1, line in enumerate(code_lines):
        match = list(re.finditer(r"\[.*?\..*?\]", line))
        if len(match) > 0:
            new_code_lines = []
            left_spaces = len(line) - len(line.lstrip())
            new_line = ""
            curr_start = 0
            for idx, x in enumerate(match):
                start, end = x.start(), x.end()
                new_line += line[curr_start:start]
                new_line += f"[tmp{idx}]"
                curr_start = end
                word = line[start:end]
                index_word = word.split("[", 1)[1].split("]")[0]
                new_code_lines.append(
                    " " * left_spaces + f"let tmp{idx} = " + index_word + ";"
                )
            new_line += line[curr_start:]
            new_code_lines.append(new_line)
            ret.append(
                "\n".join(code_lines[:i1] +
                          new_code_lines + code_lines[i1 + 1:])
            )
    return ret


def definition_replace(code):
    if "#[derive(Default, Clone, Copy)]" in code:
        return [
            code.replace(
                "#[derive(Default, Clone, Copy)]",
                "#[derive(Default)]",
            )
        ]
    else:
        return []


##


class Cache:
    def __init__(self, cache_dir, name):
        self.path = os.path.join(cache_dir, name)
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


class RustProjectCompilationFailedError(Exception):
    pass


class CallLLMTimeoutError(Exception):
    pass


global_cache_dict = {
    "macro": None,
    "macro_function": None,
    "definition": None,
    "dummy_function": None,
    "function": None
}


def update_codes(type, codes: list[RustCode]):
    llm_functions = {
        "macro": get_our_result_macro,
        "macro_function": get_our_result_macro_function,
        "definition": get_our_result_definition,
        "dummy_function": get_our_result_dummy_function,
        "function": get_our_result_function,
    }
    get_our_result = llm_functions[type]
    cache = global_cache_dict[type]
    if len(codes) == 1:
        c = codes[0]
        c.rust_code = get_our_result(c.c_code, cache.cache)
        cache.update(c.c_code, c.rust_code)
    else:
        with ProcessPool(max_workers=20) as pool:
            futures = []
            for c in codes:
                future = pool.schedule(
                    get_our_result, args=[c.c_code, cache.cache], timeout=300
                )
                futures.append((c, future))
            for c, future in tqdm(futures):
                try:
                    rust_code = future.result()
                    c.rust_code = rust_code
                    cache.update(c.c_code, c.rust_code)
                except Exception as e:
                    raise CallLLMTimeoutError(e)


def extract_c_metadata_from_project(proj_name, src_folders, macros, replacements):
    files = preprocess(
        os.path.join(project_dir, proj_name), src_folders, macros, replacements
    )
    metadata = get_metadata(files)
    declarations_location = {}
    for f in metadata:
        for func in metadata[f].functions:
            declarations_location[func] = f
        for global_var in metadata[f].global_variables:
            declarations_location[global_var] = f
        for type in metadata[f].types:
            if type != "":
                declarations_location[type] = f
    os.makedirs(os.path.join(c_metadata_dir, proj_name), exist_ok=True)
    print(f"C project `{proj_name}` resolve succeeded!")
    with open(os.path.join(c_metadata_dir, proj_name, "files.json"), "w") as f:
        f.write(
            json.dumps(
                metadata,
                default=lambda o: o.__dict__(),
                indent=4,
                ensure_ascii=False,
            )
        )
    with open(
        os.path.join(c_metadata_dir, proj_name, "declarations_location.json"),
        "w",
    ) as f:
        f.write(json.dumps(declarations_location, indent=4, ensure_ascii=False))
    print(
        f"C project `{proj_name}` metadata stored at {os.path.join(c_metadata_dir, proj_name)}"
    )


def c_metadata_to_rust_metadata(proj_name):
    with open(os.path.join(c_metadata_dir, proj_name, "files.json"), "r") as f:
        files_data = json.load(f)
    with open(
        os.path.join(c_metadata_dir, proj_name, "declarations_location.json"),
        "r",
    ) as f:
        declarations_data = json.load(f)
    metadata = resolve_metadata(files_data, declarations_data)
    os.makedirs(os.path.join(rust_metadata_dir, proj_name), exist_ok=True)
    with open(os.path.join(rust_metadata_dir, proj_name, "metadata.json"), "w") as f:
        json.dump(metadata.__dict__(), f, indent=4)

    with open(os.path.join(rust_metadata_dir, proj_name, "metadata.json"), "r") as f:
        files_data = json.load(f)
    metadata = RustProjectMetadata.from_dict(files_data)
    print(
        f"Rust project `{proj_name}` metadata stored at {os.path.join(c_metadata_dir, proj_name)}"
    )
    proj = RustProject(proj_name, metadata, created_project_dir)
    print(f"Create rust project `{proj_name}` at {proj.dir_path}")
    success, error_msg = proj.build_project()
    if success:
        print(
            f"Rust skeleton project {proj_name}(at {proj.dir_path}) build succeeded!")
    else:
        raise RustProjectCompilationFailedError(error_msg)
    return metadata


def code_filling(
    type,
    proj_name,
    metadata,
    fast=False,
    fast_end_idx=-1,
    optimizations=[],
    allow_error=False
):
    get_name = {
        "macro": "macro",
        "macro_function": "macro_function",
        "definition": "definition",
        "dummy_function": "function",
        "function": "function",
    }
    print(f"Start {type} filling and compilation verification.")
    codes = metadata.get_all(get_name[type])
    cache = global_cache_dict[type]
    output = []
    if not fast:
        for c in tqdm(codes):
            if allow_error:
                original_code = c.rust_code
            update_codes(type, [c])
            curr_cache_path = os.path.join(
                cache.path, cache.cache_index[c.c_code], "result.rs"
            )
            proj = RustProject(proj_name, metadata, created_project_dir)
            success, error_msg = proj.build_project()
            original_error_msg = error_msg
            if not success:
                curr_code, status, error_msg = c.rust_code, False, error_msg
                for o in optimizations:
                    curr_code, status, error_msg = o.try_optimize(c)
                if not status:
                    if not allow_error:
                        raise RustProjectCompilationFailedError(
                            error_msg + "\n" + "Error at:" + curr_cache_path
                        )
                    else:
                        tuple = {
                            "c_code": c.c_code,
                            "rust_code": c.rust_code,
                            "error_msg": original_error_msg,
                        }
                        output.append(tuple)
                        print(error_msg + "\n" + "Error at:" + curr_cache_path)
                        c.rust_code = original_code
                else:
                    c.rust_code = curr_code
                    cache.update(c.c_code, c.rust_code)
    else:
        if fast_end_idx == -1 or fast_end_idx >= len(codes):
            update_codes(type, codes)
            proj = RustProject(proj_name, metadata, created_project_dir)
            success, error_msg = proj.build_project()
            if not success:
                if not allow_error:
                    raise RustProjectCompilationFailedError(error_msg)
                else:
                    pass
                    # print(error_msg)
        else:
            fast_filling_codes = codes[:fast_end_idx]
            update_codes(type, fast_filling_codes)
            proj = RustProject(proj_name, metadata, created_project_dir)
            success, error_msg = proj.build_project()
            if not success:
                if not allow_error:
                    raise RustProjectCompilationFailedError(error_msg)
                else:
                    pass
                    # print(error_msg)
            remaining_codes = codes[fast_end_idx:]
            for c in tqdm(remaining_codes):
                update_codes(type, [c])
                curr_cache_path = os.path.join(
                    cache.path, cache.cache_index[c.c_code], "result.rs"
                )
                proj = RustProject(proj_name, metadata, created_project_dir)
                success, error_msg = proj.build_project()
                if not success:
                    for o in optimizations:
                        curr_code, status, error_msg = o.try_optimize(c)
                    if not status:
                        if not allow_error:
                            raise RustProjectCompilationFailedError(
                                error_msg + "\n" + "Error at:" + curr_cache_path
                            )
                        else:
                            pass
                            # print(error_msg + "\n" + "Error at:" + curr_cache_path)
                    else:
                        c.rust_code = curr_code
                        cache.update(c.c_code, c.rust_code)
    return output


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--project_name",        type=str,
                        default="c-algorithms",        help="Name of your project folder.")
    parser.add_argument("--project_dir",        type=str,        default="./data",
                        help="Parent directory of your project folder.")
    parser.add_argument("--project_src_folders",        type=str,
                        default="include, src",        help="Source file folder of your project.")
    parser.add_argument("--c_metadata_dir",        type=str,
                        default="./c_metadata",        help="Folder that stores c project metadata.")
    parser.add_argument("--macro_dict_path",        type=str,        default="./config/macro.json",
                        help="Json config file that declare the special macros in C projects to be replaced")
    parser.add_argument("--replacement_dict_path",        type=str,        default="./config/replacement.json",
                        help="Json config file that declare the special strings in C projects to be replaced")
    parser.add_argument("--rust_metadata_dir",        type=str,
                        default="./rust_metadata",        help="Folder that stores rust project metadata.")
    parser.add_argument("--project_template_dir",        type=str,        default="./project_template",
                        help="Folder that stores the template of Rust projects.")
    parser.add_argument("--created_project_dir",        type=str,
                        default="./created_project",        help="Folder that stores created Rust projects.")
    parser.add_argument("--cache_dir",        type=str,        default="./cache",
                        help="Folder that stores translation cache.")
    parser.add_argument("--generation_only",        action=argparse.BooleanOptionalAction,        default=False,
                        help="Parallel generation without checking.")

    parser.add_argument("--post_fixing",        action=argparse.BooleanOptionalAction,        default=True,
                        help="Try to fix code to pass the compilation.")

    parser.add_argument("--output_path",        type=str,
                        default="./output.json",        help="Output information path")

    parser.add_argument("--output_project_path",        type=str,
                        default="./final_project",        help="Output rust project path")

    args = parser.parse_args()
    project_dir = os.path.join(PARENT_DIR, args.project_dir)
    c_metadata_dir = os.path.join(PARENT_DIR, args.c_metadata_dir)
    rust_metadata_dir = os.path.join(PARENT_DIR, args.rust_metadata_dir)
    project_template_dir = os.path.join(PARENT_DIR, args.project_template_dir)
    created_project_dir = os.path.join(PARENT_DIR, args.created_project_dir)
    global_cache_dir = os.path.join(PARENT_DIR, args.cache_dir)

    global_cache_dict = {
        "macro": Cache(global_cache_dir, "macro"),
        "macro_function": Cache(global_cache_dir, "macro_function"),
        "definition": Cache(global_cache_dir, "definition"),
        "dummy_function": Cache(global_cache_dir, "dummy_function"),
        "function": Cache(global_cache_dir, "function"),
    }

    with open(os.path.join(PARENT_DIR, args.macro_dict_path), "r", encoding="utf-8",) as f:
        macro_dict = json.load(f)

    with open(os.path.join(PARENT_DIR, args.replacement_dict_path), "r", encoding="utf-8",) as f:
        replacement_dict = json.load(f)

    proj_name = args.project_name
    project_src_folders = [k.strip()
                           for k in args.project_src_folders.split(",")]

    extract_c_metadata_from_project(
        proj_name, project_src_folders, macro_dict, replacement_dict)

    metadata = c_metadata_to_rust_metadata(proj_name)

    if args.generation_only:
        print("Generating Macros:")
        code_filling("macro", proj_name, metadata, fast=True, allow_error=True)
        print("Generating Macro Functions:")
        code_filling("macro_function", proj_name,
                     metadata, fast=True, allow_error=True)
        print("Generating Definitions:")
        code_filling("definition", proj_name, metadata,
                     fast=True, allow_error=True)
        print("Generating Dummy Functions:")
        code_filling("dummy_function", proj_name,
                     metadata, fast=True, allow_error=True)
        print("Generating Functions:")
        code_filling("function", proj_name, metadata,
                     fast=True, allow_error=True)
    else:
        if not args.post_fixing:
            print("Generating Macros:")
            code_filling("macro", proj_name, metadata,
                         fast=False, allow_error=False)
            print("Generating Macro Functions:")
            code_filling("macro_function", proj_name, metadata,
                         fast=False, allow_error=False)
            print("Generating Definitions:")
            code_filling("definition", proj_name, metadata,
                         fast=False, allow_error=False)
            print("Generating Dummy Functions:")
            code_filling("dummy_function", proj_name, metadata,
                         fast=False, allow_error=False)
            print("Generating Functions:")
            output = code_filling("function", proj_name,
                                  metadata, fast=False, allow_error=True)
        else:
            print("Generating Macros:")
            code_filling("macro", proj_name, metadata,
                         fast=True, allow_error=False)
            print("Generating Macro Functions:")
            code_filling("macro_function", proj_name, metadata,
                         fast=True, allow_error=False)
            print("Generating Definitions:")
            code_filling("definition", proj_name, metadata, fast=True, optimizations=[
                OptimizationAgent(proj_name, metadata,
                                  definition_replace, override=False)
            ], allow_error=False)
            print("Generating Dummy Functions:")
            code_filling("dummy_function", proj_name, metadata,
                         fast=True, allow_error=False)
            print("Generating Functions:")
            output = code_filling("function", proj_name, metadata, fast=False, optimizations=[
                OptimizationAgent(proj_name, metadata,
                                  struct_index_advancement, override=False),
                OptimizationAgent(proj_name, metadata,
                                  implicit_casting_removal, override=True),
                OptimizationAgent(proj_name, metadata,
                                  as_bool_removal, override=True),
            ], allow_error=True)

        error_cnt = len(output)
        all_cnt = len(metadata.get_all("function"))
        report = {
            "All": all_cnt,
            "Compilation Pass": all_cnt - error_cnt,
            "Compilation Error": error_cnt,
            "Pass Rate": 1 - error_cnt / all_cnt,
            "Error Information": output
        }
        with open(os.path.join(PARENT_DIR, args.output_path), "w", encoding="utf-8",) as f:
            json.dump(report, f)
        output_project_dir = os.path.join(PARENT_DIR, args.output_project_path)
        proj = RustProject(proj_name, metadata,
                           output_project_dir, no_timestamp=True)