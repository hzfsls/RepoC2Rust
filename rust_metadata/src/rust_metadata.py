from __future__ import annotations
import os
import json

from classes import *

def c_filename_to_rust_filename(name: str) -> str:
    return name.replace("-", "_").replace(".", "_") + ".rs"

def c_filename_to_rust_package_name(name: str) -> str:
    return "crate::" + name.replace("-", "_").replace(".", "_").replace("/", "::")

def resolve_directory(filenames: list[str]) -> dict[str, dict]:
    rust_filenames = [ f.split("/") for f in filenames ]
    rust_filenames = [ f for f in rust_filenames if len(f) > 1 ]
    directories = {}
    for f in rust_filenames:
        curr_dir = directories
        for name in f:
            if name not in curr_dir:
                curr_dir[name] = {}
            curr_dir = curr_dir[name]
    return directories



def directories_to_paths(name:str, directories: dict[str, dict]) -> RustPath:
    if len(directories) == 0:
        assert name.endswith(".rs"), name
        path = RustFile(name)
        path.declarations = ["use crate::translation_utils::*;"]
    else:
        path = RustFolder(name)
        path.children = { k: directories_to_paths(k, v) for k, v in directories.items() }
    return path

def directories_to_metadata(directories: dict[str, dict]) -> RustProjectMetadata:
    proj = RustProjectMetadata()
    proj.paths = { k: directories_to_paths(k, v) for k, v in directories.items() }
    name_dict = {}
    def recursive_name_dict(name:str, path: RustPath):
        if path.type == "file":
            name_dict[name] = path
        if path.type == "folder":
            for k, v in path.children.items():
                recursive_name_dict(k, v)
    for k, v in proj.paths.items():
        recursive_name_dict(k, v)
    return proj, name_dict


def add_mod_rs(folder: RustFolder):
    if "mod.rs" not in folder.children:
        mod_rs = RustFile("mod.rs")
        mods = []
        for k, v in folder.children.items():
            if v.type == "folder":
                mods.append(f"pub mod {k};")
            elif v.type == "file":
                mods.append(f"pub mod {k.split('.')[0]};")
        mod_rs.declarations += mods
        folder.children["mod.rs"] = mod_rs

def recursive_add_mod_rs(path: RustPath):
    if path.type == "folder":
        add_mod_rs(path)
        for k, v in path.children.items():
            recursive_add_mod_rs(v)
    elif path.type == "file":
        pass

def includes_to_declarations(includes: list[str], name_dict, c_name_dict) -> list[str]:
    declarations = []
    for i in includes:
        include_filename = ""
        if "<" in i:
            include_filename = i.split("<")[1].split(">")[0]
        elif '"' in i:
            include_filename = i.split('"')[1]
        else:
            raise Exception("Invalid include")        
        rust_include_filename = c_filename_to_rust_filename(include_filename)
        if rust_include_filename in name_dict:
            rust_path = name_dict[rust_include_filename]
            c_name = c_name_dict[include_filename]
            declarations.append(f"pub use {c_filename_to_rust_package_name(c_name)}::*;")
        else:
            print(f"Warning: {rust_include_filename} not found")
    return declarations

def resolve_metadata(files: dict[str, str], functions: dict[str, str]) -> dict[str, dict]:
    functions_use = { f: "pub use " + c_filename_to_rust_package_name(functions[f]) + "::" + f + ";" for f in functions }
    all_file_names = [ c_filename_to_rust_filename(f) for f in files ]
    c_name_dict = { f.split("/")[-1]: f for f in files }
    print(c_name_dict)
    directories = resolve_directory(all_file_names)
    metadata, name_dict = directories_to_metadata(directories)
    root_mods = ["pub(crate) mod translation_utils;"]
    for k, v in metadata.paths.items():
        if v.type == "file":
            assert k.endswith(".rs"), k
            file_name = k.split(".")[0]
            root_mods.append(f"pub(crate) mod {file_name};")
        elif v.type == "folder":
            root_mods.append(f"pub(crate) mod {k};")
    metadata.paths["lib.rs"] = RustFile("lib.rs")
    metadata.paths["lib.rs"].declarations = root_mods
    for path in metadata.paths.values():
        recursive_add_mod_rs(path)
    # 处理includes
    for path in files:
        target_path = name_dict[c_filename_to_rust_filename(path.split("/")[-1])]
        target_path.declarations += includes_to_declarations(files[path]["includes"], name_dict, c_name_dict)
    # 处理function_declarations
    for path in files:
        target_path = name_dict[c_filename_to_rust_filename(path.split("/")[-1])]
        target_path.declarations += [ functions_use[f] for f in files[path]["function_declarations"] ]
    # 处理macros
    for path in files:
        target_path = name_dict[c_filename_to_rust_filename(path.split("/")[-1])]
        target_path.macros += [ RustCode(m) for m in files[path]["macros"]]
    # 处理macro_functions
    for path in files:
        target_path = name_dict[c_filename_to_rust_filename(path.split("/")[-1])]
        target_path.macro_functions += [ RustCode(mf) for mf in files[path]["macro_functions"]]
    # 处理types
    for path in files:
        target_path = name_dict[c_filename_to_rust_filename(path.split("/")[-1])]
        target_path.definitions += [ RustCode(t) for t in files[path]["types"]]    
    # 处理variable_declarations
    for path in files:
        target_path = name_dict[c_filename_to_rust_filename(path.split("/")[-1])]
        target_path.definitions += [ RustCode(vd) for vd in files[path]["variable_declarations"]]
    # 处理functions
    for path in files:
        target_path = name_dict[c_filename_to_rust_filename(path.split("/")[-1])]
        for f, v in files[path]["functions"].items():
            code = RustCodeWithDummy(v)
            code.dummy_code = "pub fn " + f + "() { unimplemented!(); }"
            target_path.functions.append(code)
    return metadata

if __name__ == "__main__":
    proj_name = "bzp"
    with open(f"metadata/{proj_name}/files.json", "r") as f:
        files_data = json.load(f)
    with open(f"metadata/{proj_name}/functions.json", "r") as f:
        functions_data = json.load(f)
    metadata = resolve_metadata(files_data, functions_data)
    os.makedirs(f"rust_metadata/{proj_name}", exist_ok=True)
    with open(f"rust_metadata/{proj_name}/metadata.json", "w") as f:
        json.dump(metadata.__dict__(), f, indent=4)
