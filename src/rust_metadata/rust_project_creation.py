from __future__ import annotations
import os
import json
import shutil
import time
import subprocess

from classes import *
from rust_metadata import resolve_metadata

c_metadata_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(__file__))), "c_metadata")
rust_metadata_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(__file__))), "rust_metadata")
template_project_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(__file__))), "project_template")
created_project_dir = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(__file__))), "created_project")



def create_under_current_dir(dir_path: str, rpath: RustPath):
    if rpath.type == "folder":
        os.makedirs(f"{dir_path}/{rpath.name}", exist_ok=True)
        for k, v in rpath.children.items():
            create_under_current_dir(f"{dir_path}/{rpath.name}", v)
    elif rpath.type == "file":
        with open(f"{dir_path}/{rpath.name}", "w") as f:
            f.write("\n".join(rpath.declarations) + "\n\n")
            for k in rpath.definitions:
                f.write(k.rust_code + "\n\n")
            for k in rpath.macros:
                f.write(k.rust_code + "\n\n")
            for k in rpath.macro_functions:
                f.write(k.rust_code + "\n\n")
            for k in rpath.functions:
                f.write(k.dummy_code + "\n\n")

class RustProject:
    def __init__(self, name: str, metadata: RustProjectMetadata):
        self.dir_path = os.path.join(created_project_dir, f"{proj_name}_{int(time.time())}")
        self.metadata = metadata
        self.create_project()
    
    def create_project(self):       
        os.makedirs(self.dir_path, exist_ok=True)
        shutil.copytree(template_project_dir, self.dir_path, dirs_exist_ok=True)
        paths = self.metadata.paths
        for k, v in paths.items():
            create_under_current_dir(os.path.join(self.dir_path, "src"), v)

    def build_project(self):
        result = subprocess.run(["RUSTFLAGS=-Awarnings cargo build"], shell=True, cwd=self.dir_path, timeout=10, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if result.returncode == 0:
            return True, ""
        else:
            error_msg = result.stderr.decode("utf-8")
            return False, error_msg


if __name__ == "__main__":
    for proj_name in ["avl", "bzp", "cmptlz", "rapidlz", "md5", "sha256"]:
        with open(os.path.join(c_metadata_dir, proj_name, "files.json"), "r") as f:
            files_data = json.load(f)
        with open(os.path.join(c_metadata_dir, proj_name, "functions.json"), "r") as f:
            functions_data = json.load(f)
        metadata = resolve_metadata(files_data, functions_data)
        os.makedirs(os.path.join(rust_metadata_dir, proj_name), exist_ok=True)
        with open(os.path.join(rust_metadata_dir, proj_name, "metadata.json"), "w") as f:
            json.dump(metadata.__dict__(), f, indent=4)
        with open(os.path.join(rust_metadata_dir, proj_name, "metadata.json"), "r") as f:
            files_data = json.load(f)
        metadata = RustProjectMetadata.from_dict(files_data)
        proj = RustProject(proj_name, metadata)
        print(f"Create rust project `{proj_name}` at {proj.dir_path}")
        success, error_msg = proj.build_project()
        if success:
            print(f"{proj_name} build succeed!")
        else:
            print(f"{proj_name} build fail!")
            print(error_msg)

