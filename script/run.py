import os
import json

from tqdm import tqdm

from c_metadata.c_metadata import extract_c_metadata_from_project
from rust_metadata.rust_project_creation import RustProject, c_metadata_to_rust_metadata

from llm_gen.generation import GenerationClient, update_codes, get_repair_candidates, get_delim_repair_candidates
from llm_gen.cache import LLMGenerationCache

from misc.exceptions import RustProjectCompilationFailedError

from optimization_agent.classes import OptimizationAgent, OptimizationAgentWithCompilerFeedback

import argparse

PARENT_DIR = os.path.dirname(os.path.dirname(__file__))

client = GenerationClient(api_key="sk-76da526dbd8b48c3954df9336a8a6592", base_url="https://api.deepseek.com/beta")

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

def fix_mismatched_delim(code, compiler_msg):
    if "unclosed delimiter" not in compiler_msg:
        return []
    else:
        return get_delim_repair_candidates(client, code, compiler_msg)

def llm_try_repair(code, compiler_msg):
    print(compiler_msg)
    if compiler_msg == "":
        return []
    return get_repair_candidates(client, code, compiler_msg)

# def field_function_paren(code):
#     import re

#     code_lines = code.split("\n")
#     ret = []
#     for i1, line in enumerate(code_lines):
#         match = list(re.finditer(r"[a-zA-Z0-9_\.]+\.[a-zA-Z0-9_]+\(", line))
#         if len(match) > 0:
#             new_code_lines = []
#             new_line = ""
#             curr_start = 0
#             for idx, x in enumerate(match):
#                 start, end = x.start(), x.end()
#                 new_line += line[curr_start:start]
#                 new_line += "("
#                 new_line += line[start:end-1]
#                 new_line += ")("
#                 curr_start = end
#             new_line += line[curr_start:]
#             new_code_lines.append(new_line)
#             ret.append(
#                 "\n".join(code_lines[:i1] +
#                           new_code_lines + code_lines[i1 + 1:])
#             )
#     return ret

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


def code_filling(
    type,
    proj_name,
    metadata,
    prompts,
    fast=False,
    fast_end_idx=-1,
    optimizations=[],
    allow_error=False,
    caches={},
    created_project_dir = "./created_project",
    template_project_dir = "./template_project"
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
    cache = caches[type]
    output = []
    if not fast:
        for c in tqdm(codes):
            if allow_error:
                original_code = c.rust_code
            update_codes(type, client, prompts, [c], caches)
            curr_cache_path = os.path.join(
                cache.path, cache.cache_index[c.c_code], "result.rs"
            )
            proj = RustProject(proj_name, metadata, created_project_dir, template_project_dir)
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
                            "original_error_msg": original_error_msg,
                            "optimized_code": curr_code,
                            "error_msg": error_msg
                        }
                        output.append(tuple)
                        print(error_msg + "\n" + "Error at:" + curr_cache_path)
                        c.rust_code = original_code
                else:
                    c.rust_code = curr_code
                    cache.update(c.c_code, c.rust_code)
    else:
        if fast_end_idx == -1 or fast_end_idx >= len(codes):
            update_codes(type, client, prompts, codes, caches)
            proj = RustProject(proj_name, metadata, created_project_dir, template_project_dir)
            success, error_msg = proj.build_project()
            if not success:
                if not allow_error:
                    raise RustProjectCompilationFailedError(error_msg)
                else:
                    pass
                    # print(error_msg)
        else:
            fast_filling_codes = codes[:fast_end_idx]
            update_codes(type, client, prompts, fast_filling_codes, caches)
            proj = RustProject(proj_name, metadata, created_project_dir, template_project_dir)
            success, error_msg = proj.build_project()
            if not success:
                if not allow_error:
                    raise RustProjectCompilationFailedError(error_msg)
                else:
                    pass
                    # print(error_msg)
            remaining_codes = codes[fast_end_idx:]
            for c in tqdm(remaining_codes):
                update_codes(type, client, prompts, [c], caches)                
                curr_cache_path = os.path.join(
                    cache.path, cache.cache_index[c.c_code], "result.rs"
                )
                proj = RustProject(proj_name, metadata, created_project_dir, template_project_dir)
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
    parser.add_argument("--config_dir",        type=str,        default="./config", help="Folder that stores tool configurations.")  
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
    template_project_dir = os.path.join(PARENT_DIR, args.project_template_dir)
    created_project_dir = os.path.join(PARENT_DIR, args.created_project_dir)
    global_cache_dir = os.path.join(PARENT_DIR, args.cache_dir)

    cache_dict = {
        "macro": LLMGenerationCache(global_cache_dir, "macro"),
        "macro_function": LLMGenerationCache(global_cache_dir, "macro_function"),
        "definition": LLMGenerationCache(global_cache_dir, "definition"),
        "dummy_function": LLMGenerationCache(global_cache_dir, "dummy_function"),
        "function": LLMGenerationCache(global_cache_dir, "function"),
    }

    with open(os.path.join(PARENT_DIR, args.config_dir, "macro.json"), "r", encoding="utf-8") as f:
        macro_dict = json.load(f)

    with open(os.path.join(PARENT_DIR, args.config_dir, "replacement.json"), "r", encoding="utf-8") as f:
        replacement_dict = json.load(f)
    
    prompts = {}
    for type in ["macro", "macro_function", "definition", "dummy_function", "function"]:
        with open(os.path.join(PARENT_DIR, args.config_dir, "prompts", f"{type}.txt"), "r", encoding="utf-8") as f:
            prompts[type] = f.read().strip()
    
    proj_name = args.project_name
    project_src_folders = [k.strip() for k in args.project_src_folders.split(",")]

    extract_c_metadata_from_project(proj_name, project_dir=project_dir, c_metadata_dir=c_metadata_dir, src_folders=project_src_folders, macros=macro_dict, replacements=replacement_dict)

    metadata = c_metadata_to_rust_metadata(proj_name, c_metadata_dir=c_metadata_dir, rust_metadata_dir=rust_metadata_dir, created_project_dir=created_project_dir, template_project_dir=template_project_dir)

    if args.generation_only:
        print("Generating Macros:")
        code_filling("macro", proj_name, metadata, prompts, fast=True, allow_error=True, caches=cache_dict, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
        print("Generating Macro Functions:")
        code_filling("macro_function", proj_name,
                     metadata, prompts, fast=True, allow_error=True, caches=cache_dict, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
        print("Generating Definitions:")
        code_filling("definition", proj_name, metadata, prompts,
                     fast=True, allow_error=True, caches=cache_dict, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
        print("Generating Dummy Functions:")
        code_filling("dummy_function", proj_name,
                     metadata, prompts, fast=True, allow_error=True, caches=cache_dict, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
        print("Generating Functions:")
        code_filling("function", proj_name, metadata, prompts,
                     fast=True, allow_error=True, caches=cache_dict, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
    else:
        if not args.post_fixing:
            print("Generating Macros:")
            code_filling("macro", proj_name, metadata, prompts,
                         fast=False, allow_error=False, caches=cache_dict, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
            print("Generating Macro Functions:")
            code_filling("macro_function", proj_name, metadata, prompts,
                         fast=False, allow_error=False, caches=cache_dict, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
            print("Generating Definitions:")
            code_filling("definition", proj_name, metadata, prompts, fast=False, optimizations=[
                OptimizationAgent(proj_name, metadata,
                                  definition_replace, override=False, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
            ], allow_error=False, caches=cache_dict, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
            print("Generating Dummy Functions:")
            code_filling("dummy_function", proj_name, metadata, prompts,
                         fast=True, allow_error=False, caches=cache_dict, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
            print("Generating Functions:")
            output = code_filling("function", proj_name,
                                  metadata, prompts, fast=False, allow_error=True, caches=cache_dict, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
        else:
            print("Generating Macros:")
            code_filling("macro", proj_name, metadata, prompts,
                         fast=False, allow_error=False, caches=cache_dict, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
            print("Generating Macro Functions:")
            code_filling("macro_function", proj_name, metadata, prompts,
                         fast=False, allow_error=False, caches=cache_dict, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
            print("Generating Definitions:")
            code_filling("definition", proj_name, metadata, prompts, fast=False, optimizations=[
                OptimizationAgent(proj_name, metadata,
                                  definition_replace, override=False, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
            ], allow_error=False, caches=cache_dict, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
            print("Generating Dummy Functions:")
            code_filling("dummy_function", proj_name, metadata, prompts,
                         fast=True, allow_error=False, caches=cache_dict, created_project_dir=created_project_dir, template_project_dir=template_project_dir)
            print("Generating Functions:")
            output = code_filling("function", proj_name, metadata, prompts, fast=False, optimizations=[
                OptimizationAgentWithCompilerFeedback(proj_name, metadata,
                                  fix_mismatched_delim, override=False, created_project_dir=created_project_dir, template_project_dir=template_project_dir),
                OptimizationAgent(proj_name, metadata,
                                  struct_index_advancement, override=False, created_project_dir=created_project_dir, template_project_dir=template_project_dir),
                OptimizationAgent(proj_name, metadata,
                                  implicit_casting_removal, override=True, created_project_dir=created_project_dir, template_project_dir=template_project_dir),
                OptimizationAgent(proj_name, metadata,
                                  as_bool_removal, override=True, created_project_dir=created_project_dir, template_project_dir=template_project_dir),
                OptimizationAgentWithCompilerFeedback(proj_name, metadata,
                                  llm_try_repair, override=False, created_project_dir=created_project_dir, template_project_dir=template_project_dir),                                
            ], allow_error=True, caches=cache_dict, created_project_dir=created_project_dir, template_project_dir=template_project_dir)

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
                           output_project_dir, template_project_dir, no_timestamp=True)