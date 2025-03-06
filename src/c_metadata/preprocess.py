import subprocess
import os
import json

import tree_sitter_c as ts_c
from typing import Generator
from tree_sitter import Language, Parser, Tree, Node

c_language = Language(ts_c.language())
c_parser = Parser(c_language)



def get_files(src_folder: str, dirs: list[str]) -> dict[str, str]:
    files = {}
    for d in dirs:
        for root, _, fs in os.walk(os.path.join(src_folder, d)):
            for f in fs:
                if f.endswith(".c") or f.endswith(".h"):
                    file_name = os.path.join(root, f)
                    file_content = open(file_name, errors='ignore').read()
                    new_file_name = os.path.relpath(file_name, src_folder)
                    files[new_file_name] = file_content
    return files

def remove_files_comments(files: dict[str, str]) -> dict[str, str]:
    results = {}
    for file_rel_path in files:
        tmp_file_path = os.path.join(".tmp", file_rel_path)
        os.makedirs(os.path.dirname(tmp_file_path), exist_ok=True)
        with open(tmp_file_path, "w") as f:
            f.write(files[file_rel_path].replace("\\\n", ""))
        result = subprocess.run(f"gcc -fpreprocessed -dD -E {tmp_file_path}", shell=True, capture_output=True)
        if result.returncode != 0:
            raise Exception(f"Error in removing {file_rel_path} comments")
        try:
            results[file_rel_path] = result.stdout.decode("utf-8")
        except Exception as e:
            raise Exception(f"Error in decoding {file_rel_path} output: {e}")
    return results

def clang_format_files(files: dict[str, str]) -> dict[str, str]:
    results = {}
    for file_rel_path in files:
        tmp_file_path = os.path.join(".tmp", file_rel_path)
        os.makedirs(os.path.dirname(tmp_file_path), exist_ok=True)
        with open(tmp_file_path, "w") as f:
            f.write(files[file_rel_path])
        result = subprocess.run(f"clang-format -style=Microsoft {tmp_file_path}", shell=True, capture_output=True)
        if result.returncode != 0:
            raise Exception(f"Error in removing {file_rel_path} comments")
        try:
            results[file_rel_path] = result.stdout.decode("utf-8")
        except Exception as e:
            raise Exception(f"Error in decoding {file_rel_path} output: {e}")
    return results

def replace_files_macros(files: dict[str, str], macros: dict[str, str] = {}, replacements: dict[str, str] = {}) -> dict[str, str]:
    results = {}
    macros = dict(sorted(macros.items(), key=lambda x: len(x[0]), reverse=True))
    for file_rel_path in files:
        content = files[file_rel_path]
        for k, v in replacements.items():
            content = content.replace(k, v)
        for macro, replace in macros.items():
            lines = content.split("\n")
            new_lines = []
            for line in lines:
                if not line.strip().startswith("#"):
                    new_lines.append(line.replace(macro, replace))
                else:
                    new_lines.append(line)
            content = "\n".join(new_lines)
        results[file_rel_path] = content
    return results

def if_parse_error(node: Node) -> bool:
    if node.type in ["gnu_asm_expression", "function_definition"]:
        return True
    if node.type == "ERROR":
        return False
    for i in range(node.child_count):
        child = node.child(i)
        if not if_parse_error(child):
            return False
    return True

def try_parse(files: dict[str, str]):
    for file_name, file_content in files.items():
        tree = c_parser.parse(bytes(file_content, "utf-8"))
        if not if_parse_error(tree.root_node):
            print(file_content)
            raise Exception(f"Error in parsing {file_name}")

def preprocess(src_folder: str, dirs: list[str], macros: dict[str, str] = {}, replacements: dict[str, str] = {}) -> dict[str, str]:
    files = get_files(src_folder, dirs)
    files = remove_files_comments(files)
    files = replace_files_macros(files, macros, replacements)
    files = clang_format_files(files)
    try_parse(files)
    return files

project_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(__file__))), "data")
c_metadata_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(__file__))), "c_metadata")

if __name__ == "__main__":
    for project_name in ["avl", "bzp", "cmptlz", "rapidlz", "md5", "sha256"]:
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
            f.write(json.dumps(functions, indent=4, ensure_ascii=False))\

            