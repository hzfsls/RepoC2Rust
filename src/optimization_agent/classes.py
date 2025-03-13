import os
import sys

from rust_metadata.rust_project_creation import (
    RustProject,
    RustCode,
    RustProjectMetadata,
)
from rust_metadata.classes import RustFile
from tqdm import tqdm

import sys

created_project_dir = os.path.join(
    os.path.dirname(os.path.dirname(__file__)), "created_project"
)


class OptimizationAgent:
    def __init__(self, proj_name, metadata, optimize_func, override=False):
        self.name = proj_name
        self.metadata = metadata
        self.optimize_func = optimize_func
        self.override = override

    def try_build(self):
        proj = RustProject(self.name, self.metadata)
        success, error_msg = proj.build_project()
        if success:
            return True, ""
        else:
            return False, error_msg

    def try_optimize(self, code):
        original_code = code.rust_code
        status, error_msg = self.try_build()
        last_error_length = error_msg.count("\n")
        curr_error_length = error_msg.count("\n") + 1
        curr_code = original_code
        while True:
            candidates = self.optimize_func(curr_code)
            if len(candidates) == 0:
                break
            flag = False
            for c in candidates:
                code.rust_code = c
                status, error_msg = self.try_build()
                if error_msg.count("\n") < curr_error_length or (
                    error_msg.count("\n") == curr_error_length and self.override
                ):
                    curr_error_length = error_msg.count("\n")
                    curr_code = c
                    flag = True
                    break
            if curr_error_length < last_error_length or (
                error_msg.count("\n") == curr_error_length and self.override and flag
            ):
                last_error_length = curr_error_length
                curr_code = c
            else:
                break
        code.rust_code = curr_code
        status, error_msg = self.try_build()
        return curr_code, status, error_msg
