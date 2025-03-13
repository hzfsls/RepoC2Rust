import os
import json

from c_metadata.c_metadata import get_metadata
from c_metadata.preprocess import preprocess
from rust_metadata.rust_metadata import resolve_metadata
from rust_metadata.rust_project_creation import RustProject, RustProjectMetadata
from rust_metadata.classes import RustCode
from tqdm import tqdm


from llm_gen.definition_translation import (
    get_our_results_definition,
    get_our_result_definition,
)
from llm_gen.function_translation import (
    get_our_results_function,
    get_our_result_function,
)
from llm_gen.macro_translation import get_our_results_macro, get_our_result_macro
from llm_gen.macro_function_translation import (
    get_our_results_macro_function,
    get_our_result_macro_function,
)
from llm_gen.dummy_function_translation import (
    get_our_results_dummy_function,
    get_our_result_dummy_function,
)

from optimization_agent.classes import OptimizationAgent


import argparse

project_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), "data")
c_metadata_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), "c_metadata")
rust_metadata_dir = os.path.join(
    os.path.dirname(os.path.dirname(__file__)), "rust_metadata"
)
project_template_dir = os.path.join(
    os.path.dirname(os.path.dirname(__file__)), "project_template"
)
created_project_dir = os.path.join(
    os.path.dirname(os.path.dirname(__file__)), "created_project"
)
global_cache_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), "cache")

## Optimization functions


def implicit_casting_removal(code):
    ret = []
    sub = ".cast()"
    start = 0
    while True:
        start = code.find(sub, start)
        if start == -1:
            break
        new_code = code[:start] + code[start + len(sub) :]
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
        new_code = code[:start] + code[start + len(sub) :]
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
                "\n".join(code_lines[:i1] + new_code_lines + code_lines[i1 + 1 :])
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


class RustProjectCompilationFailedError(Exception):
    pass


definition_cache = Cache("definition")
macro_cache = Cache("macro")
macro_function_cache = Cache("macro_function")
dummy_function_cache = Cache("dummy_function")
function_cache = Cache("function")


def update_definitions(codes: list[RustCode]):
    for c in codes:
        c.rust_code = get_our_result_definition(c.c_code, definition_cache.cache)
        definition_cache.update(c.c_code, c.rust_code)


def update_macros(codes: list[RustCode]):
    for c in codes:
        c.rust_code = get_our_result_macro(c.c_code, macro_cache.cache)
        macro_cache.update(c.c_code, c.rust_code)


def update_macro_functions(codes: list[RustCode]):
    for c in codes:
        c.rust_code = get_our_result_macro_function(
            c.c_code, macro_function_cache.cache
        )
        macro_function_cache.update(c.c_code, c.rust_code)


def update_functions(codes: list[RustCode]):
    for c in codes:
        c.rust_code = get_our_result_function(c.c_code, function_cache.cache)
        function_cache.update(c.c_code, c.rust_code)


def update_dummy_functions(codes: list[RustCode]):
    for c in codes:
        c.rust_code = get_our_result_dummy_function(
            c.c_code, dummy_function_cache.cache
        )
        dummy_function_cache.update(c.c_code, c.rust_code)


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
        print(f"Rust skeleton project {proj_name}(at {proj.dir_path}) build succeeded!")
    else:
        raise RustProjectCompilationFailedError(error_msg)
    return metadata


def code_filling(
    proj_name,
    metadata,
    type,
    fast=False,
    fast_end_idx=-1,
    optimizations=[],
    allow_error=False,
    output_information=False,
):
    update_hooks = {
        "macro": update_macros,
        "macro_function": update_macro_functions,
        "definition": update_definitions,
        "dummy_function": update_dummy_functions,
        "function": update_functions,
    }
    get_name = {
        "macro": "macro",
        "macro_function": "macro_function",
        "definition": "definition",
        "dummy_function": "function",
        "function": "function",
    }
    cache_name = {
        "macro": macro_cache,
        "macro_function": macro_function_cache,
        "definition": definition_cache,
        "dummy_function": dummy_function_cache,
        "function": function_cache,
    }
    print(f"Start {type} filling and compilation verification.")
    codes = metadata.get_all(get_name[type])
    func = update_hooks[type]
    cache = cache_name[type]
    output = []
    if not fast:
        for c in tqdm(codes):
            if allow_error:
                original_code = c.rust_code
            func([c])            
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
                            "Error at:" + curr_cache_path + "\n" + error_msg
                        )
                    else:
                        if output_information:
                            tuple = {
                                "c_code": c.c_code,
                                "rust_code": c.rust_code,
                                "error_msg": original_error_msg,
                            }
                            output.append(tuple)
                        print("Error at:" + curr_cache_path + "\n" + error_msg)
                        c.rust_code = original_code
                else:
                    c.rust_code = curr_code
                    cache.update(c.c_code, c.rust_code)
    else:
        if fast_end_idx == -1 or fast_end_idx >= len(codes):
            func(codes)
            proj = RustProject(proj_name, metadata, created_project_dir)
            success, error_msg = proj.build_project()
            if not success:
                if not allow_error:
                    raise RustProjectCompilationFailedError(error_msg)
                else:
                    print(error_msg)
        else:
            fast_filling_codes = codes[:fast_end_idx]
            func(fast_filling_codes)
            proj = RustProject(proj_name, metadata, created_project_dir)
            success, error_msg = proj.build_project()
            if not success:
                if not allow_error:
                    raise RustProjectCompilationFailedError(error_msg)
                else:
                    print(error_msg)
            remaining_codes = codes[fast_end_idx:]
            for c in tqdm(remaining_codes):
                func([c])
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
                                "Error at:" + curr_cache_path + "\n" + error_msg
                            )
                        else:
                            print("Error at:" + curr_cache_path + "\n" + error_msg)
                    else:
                        c.rust_code = curr_code
                        cache.update(c.c_code, c.rust_code)
    if output_information:
        return output


def macro_filling(
    proj_name,
    metadata,
    fast=False,
    fast_end_idx=-1,
    optimizations=[],
    allow_error=False,
    output_information=False,
):
    return code_filling(
        proj_name,
        metadata,
        "macro",
        fast,
        fast_end_idx,
        optimizations,
        allow_error,
        output_information,
    )


def macro_function_filling(
    proj_name,
    metadata,
    fast=False,
    fast_end_idx=-1,
    optimizations=[],
    allow_error=False,
    output_information=False,
):
    return code_filling(
        proj_name,
        metadata,
        "macro_function",
        fast,
        fast_end_idx,
        optimizations,
        allow_error,
        output_information,
    )


def definition_filling(
    proj_name,
    metadata,
    fast=False,
    fast_end_idx=-1,
    optimizations=[],
    allow_error=False,
    output_information=False,
):
    return code_filling(
        proj_name,
        metadata,
        "definition",
        fast,
        fast_end_idx,
        optimizations,
        allow_error,
        output_information,
    )


def dummy_function_filling(
    proj_name,
    metadata,
    fast=True,
    fast_end_idx=-1,
    optimizations=[],
    allow_error=False,
    output_information=False,
):
    return code_filling(
        proj_name,
        metadata,
        "dummy_function",
        fast,
        fast_end_idx,
        optimizations,
        allow_error,
        output_information,
    )


def function_filling(
    proj_name,
    metadata,
    fast=True,
    fast_end_idx=-1,
    optimizations=[],
    allow_error=False,
    output_information=False,
):
    return code_filling(
        proj_name,
        metadata,
        "function",
        fast,
        fast_end_idx,
        optimizations,
        allow_error,
        output_information,
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--project_name",
        type=str,
        default="c-algorithms",
        help="Name of your project folder.",
    )
    parser.add_argument(
        "--project_dir",
        type=str,
        default="./data",
        help="Parent directory of your project folder.",
    )
    parser.add_argument(
        "--project_src_folders",
        type=str,
        default="include, src",
        help="Source file folder of your project.",
    )
    parser.add_argument(
        "--c_metadata_dir",
        type=str,
        default="./c_metadata",
        help="Folder that stores c project metadata.",
    )
    parser.add_argument(
        "--macro_dict_path",
        type=str,
        default="./config/macro.json",
        help="Json config file that declare the special macros in C projects to be replaced",
    )

    parser.add_argument(
        "--replacement_dict_path",
        type=str,
        default="./config/replacement.json",
        help="Json config file that declare the special strings in C projects to be replaced",
    )

    parser.add_argument(
        "--rust_metadata_dir",
        type=str,
        default="./rust_metadata",
        help="Folder that stores rust project metadata.",
    )

    parser.add_argument(
        "--project_template_dir",
        type=str,
        default="./project_template",
        help="Folder that stores the template of Rust projects.",
    )

    parser.add_argument(
        "--created_project_dir",
        type=str,
        default="./created_project",
        help="Folder that stores created Rust projects.",
    )

    parser.add_argument(
        "--cache_dir",
        type=str,
        default="./cache",
        help="Folder that stores created Rust projects.",
    )

    parser.add_argument(
        "--fast_macro_filling",
        type=bool,
        default=False,
        help="Fill all LLM-generated macros at once.",
    )
    parser.add_argument(
        "--fast_macro_function_filling",
        type=bool,
        default=False,
        help="Fill all LLM-generated macro functions at once.",
    )
    parser.add_argument(
        "--fast_definition_filling",
        type=bool,
        default=False,
        help="Fill all LLM-generated definitions at once.",
    )
    parser.add_argument(
        "--fast_dummy_function_filling",
        type=bool,
        default=True,
        help="Fill all LLM-generated dummy functions at once.",
    )
    parser.add_argument(
        "--fast_function_filling",
        type=bool,
        default=False,
        help="Fill all LLM-generated functions at once.",
    )

    parser.add_argument(
        "--fast_macro_filling_end_idx",
        type=int,
        default=-1,
        help="Last index during fast macro filling.",
    )
    parser.add_argument(
        "--fast_macro_function_filling_end_idx",
        type=int,
        default=-1,
        help="Last index during fast macro function filling.",
    )
    parser.add_argument(
        "--fast_definition_filling_end_idx",
        type=int,
        default=-1,
        help="Last index during fast definition filling.",
    )
    parser.add_argument(
        "--fast_dummy_function_filling_end_idx",
        type=int,
        default=-1,
        help="Last index during fast dummy function filling.",
    )
    parser.add_argument(
        "--fast_function_filling_end_idx",
        type=int,
        default=-1,
        help="Last index during fast function filling.",
    )

    parser.add_argument(
        "--output_path",
        type=str,
        default="./output.json",
        help="Output information path",
    )

    parser.add_argument(
        "--output_project_path",
        type=str,
        default="./final_project",
        help="Output rust project path",
    )

    args = parser.parse_args()
    project_dir = os.path.join(
        os.path.dirname(os.path.dirname(__file__)), args.project_dir
    )
    c_metadata_dir = os.path.join(
        os.path.dirname(os.path.dirname(__file__)), args.c_metadata_dir
    )
    rust_metadata_dir = os.path.join(
        os.path.dirname(os.path.dirname(__file__)), args.rust_metadata_dir
    )
    project_template_dir = os.path.join(
        os.path.dirname(os.path.dirname(__file__)), args.project_template_dir
    )
    created_project_dir = os.path.join(
        os.path.dirname(os.path.dirname(__file__)), args.created_project_dir
    )
    global_cache_dir = os.path.join(
        os.path.dirname(os.path.dirname(__file__)), args.cache_dir
    )

    definition_cache = Cache("definition")
    macro_cache = Cache("macro")
    macro_function_cache = Cache("macro_function")
    dummy_function_cache = Cache("dummy_function")
    function_cache = Cache("function")

    with open(
        os.path.join(os.path.dirname(os.path.dirname(__file__)), args.macro_dict_path),
        "r",
        encoding="utf-8",
    ) as f:
        macro_dict = json.load(f)

    with open(
        os.path.join(
            os.path.dirname(os.path.dirname(__file__)), args.replacement_dict_path
        ),
        "r",
        encoding="utf-8",
    ) as f:
        replacement_dict = json.load(f)

    proj_name = args.project_name
    project_src_folders = [k.strip() for k in args.project_src_folders.split(",")]
    extract_c_metadata_from_project(
        proj_name, project_src_folders, macro_dict, replacement_dict
    )

    metadata = c_metadata_to_rust_metadata(proj_name)

    # update_macros(proj.metadata.get_all("macro"))
    # update_macro_functions(proj.metadata.get_all("macro_function"))
    # update_definitions(proj.metadata.get_all("definition"))
    # update_dummy_functions(proj.metadata.get_all("function"))
    # update_functions(proj.metadata.get_all("function"))

    print("Try Build After Updating Macros:")
    macro_filling(
        proj_name,
        metadata,
        fast=args.fast_macro_filling,
        fast_end_idx=args.fast_macro_filling_end_idx,
        optimizations=[],
        allow_error=False,
    )
    print("Try Build After Updating Macro Functions:")
    macro_function_filling(
        proj_name,
        metadata,
        fast=args.fast_macro_function_filling,
        fast_end_idx=args.fast_macro_function_filling_end_idx,
        optimizations=[],
        allow_error=False,
    )
    print("Try Build After Updating Definition:")
    definition_filling(
        proj_name,
        metadata,
        fast=args.fast_definition_filling,
        fast_end_idx=args.fast_definition_filling_end_idx,
        optimizations=[
            OptimizationAgent(proj_name, metadata, definition_replace, override=False)
        ],
        allow_error=False,
    )
    print("Try Build After Updating Dummy Functions:")
    dummy_function_filling(
        proj_name,
        metadata,
        fast=args.fast_dummy_function_filling,
        fast_end_idx=args.fast_dummy_function_filling_end_idx,
        optimizations=[],
        allow_error=False,
    )
    print("Try Build After Updating Functions:")
    output = function_filling(
        proj_name,
        metadata,
        fast=args.fast_function_filling,
        fast_end_idx=args.fast_function_filling_end_idx,
        optimizations=[
            OptimizationAgent(
                proj_name, metadata, struct_index_advancement, override=False
            ),
            OptimizationAgent(
                proj_name, metadata, implicit_casting_removal, override=True
            ),
            OptimizationAgent(proj_name, metadata, as_bool_removal, override=True),
        ],
        allow_error=True,
        output_information=True,
    )
    with open(
        os.path.join(os.path.dirname(os.path.dirname(__file__)), args.output_path),
        "w",
        encoding="utf-8",
    ) as f:
        json.dump(output, f)
    print("Error Num:" + str(len(output)))
    output_project_dir = os.path.join(
        os.path.dirname(os.path.dirname(__file__)), args.output_project_path
    )
    
    proj = RustProject(proj_name, metadata, output_project_dir, no_timestamp=True)
    
