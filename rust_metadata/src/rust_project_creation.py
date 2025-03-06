from __future__ import annotations
import os
import json
import shutil
import time
import subprocess
from classes import *


template_project_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), "project_template")

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
        self.dir_path = f"created_projects/{proj_name}_{int(time.time())}"
        self.metadata = metadata
        self.create_project()
    
    def create_project(self):       
        os.makedirs(self.dir_path, exist_ok=True)
        shutil.copytree(template_project_dir, self.dir_path, dirs_exist_ok=True)
        paths = self.metadata.paths
        for k, v in paths.items():
            create_under_current_dir(os.path.join(self.dir_path, "src"), v)

    def build_project(self):
        print(self.dir_path)
        result = subprocess.run(["RUSTFLAGS=-Awarnings cargo build"], shell=True, cwd=self.dir_path, timeout=10, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if result.returncode == 0:
            return True, ""
        else:
            error_msg = result.stderr.decode("utf-8")
            return False, error_msg


if __name__ == "__main__":
    proj_name = "bzp"
    with open(f"rust_metadata/{proj_name}/metadata.json", "r") as f:
        files_data = json.load(f)

    metadata = RustProjectMetadata.from_dict(files_data)
    proj = RustProject(proj_name, metadata)
    success, error_msg = proj.build_project()
    if success:
        print("Build success!")
    else:
        print("Build failed!")
        print(error_msg)

