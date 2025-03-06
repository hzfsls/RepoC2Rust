import os
import json

from c_metadata.c_metadata import get_metadata
from c_metadata.preprocess import preprocess
from rust_metadata.rust_metadata import resolve_metadata
from rust_metadata.rust_project_creation import RustProject, RustProjectMetadata

from llm_gen.definition_translation import get_our_results_definition
from llm_gen.function_translation import get_our_results_function
from llm_gen.macro_translation import get_our_results_macro
from llm_gen.macro_function_translation import get_our_results_macro_function

project_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), ".""data")
c_metadata_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), "c_metadata")
rust_metadata_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), "rust_metadata")
project_template_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), "project_template")
created_project_dir = os.path.join(os.path.dirname(os.path.dirname(__file__)), "created_project")


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

