from __future__ import annotations
import tree_sitter_c as ts_c
from typing import Generator
from tree_sitter import Language, Parser, Tree, Node
import os
import json

from c_metadata.remove_comment import preprocess

c_language = Language(ts_c.language())
c_parser = Parser(c_language)


def get_metadata(files: dict[str, str]) -> dict[str, CFileMetadata]:
    result = {}
    for f in files:
        result[f] = CFileMetadata.from_code(f, files[f])
    return result

def has_function_declarator(node: Node):
    chind_cnt = node.child_count
    if node.type == "function_declarator":
        identifier_node = node.child(0)
        assert identifier_node.type == "identifier", "A"
        name = identifier_node.text.decode("utf-8")
        return True, name
    if chind_cnt == 0:
        return False, None
    for i in range(chind_cnt):
        res, name = has_function_declarator(node.child(i))
        if res:
            return True, name
    return False, None



class CFileMetadata:
    def __init__(self, name: str):
        self.name = name
        self.includes = []
        self.types = []
        self.macros = []
        self.macro_functions = []
        self.variable_declarations = []
        self.function_declarations = {}
        self.functions = {}

    def __str__(self):
        return str(self.__dict__())
    
    def __dict__(self):
        return {
            "includes": [x for x in self.includes],
            "macros": [x for x in self.macros],
            "macro_functions": [x for x in self.macro_functions],
            "types":  [x for x in self.types],
            "variable_declarations": [x for x in self.variable_declarations],
            "function_declarations": self.function_declarations,
            "functions": self.functions
        }
    
    @staticmethod
    def from_code(name:str, code: str) -> CFileMetadata:
        tree = c_parser.parse(bytes(code, "utf8"))
        data = CFileMetadata(name)
        data.parse_node(tree.root_node)
        return data 
    
    def parse_node(self, node: Node):
        chind_cnt = node.child_count
        snippet = node.text.decode("utf-8").strip()
        if node.type == "preproc_def":
            self.macros.append(snippet)
        elif node.type == "preproc_function_def":
            self.macro_functions.append(snippet)
        elif node.type == "preproc_include":
            self.includes.append(snippet)
        elif node.type in ["type_definition", "enum_specifier", "struct_specifier", "union_specifier"]: 
            if not snippet.endswith(";"):
                snippet += ";"
            self.types.append(snippet)
        elif node.type == "function_definition":
            res, name = has_function_declarator(node)
            assert res, "B"
            self.functions[name] = snippet
        elif node.type == "declaration":
            res, name = has_function_declarator(node)
            if not res:
                self.variable_declarations.append(snippet)
            else:
                self.function_declarations[name] = snippet

        else:
            chind_cnt = node.child_count
            if chind_cnt == 0:
                return
            else:
                for i in range(chind_cnt):
                    self.parse_node(node.child(i))
    
#     def recreate_code(self) -> str:
#         code = ""
#         for include in self.includes:
#             code += str(include) + "\n"
#         code += """
# #ifdef __cplusplus
# extern "C" {
# #endif
# """     
#         for macro in self.macros:
#             code += str(macro) + "\n"
#         for macro_function in self.macro_functions:
#             code += str(macro_function) + "\n"
#         for type in self.types:
#             code += str(type) + "\n"
#         for definition in self.definitions:
#             code += str(definition) + "\n"
#         for function in self.functions:
#             code += str(function) + "\n"
#         code += """
# #ifdef __cplusplus
# }
# #endif
# """
#         is_header = self.name.endswith(".h")
#         if is_header:
#             file_name = self.name.split("/")[-1].split(".")[0]
#             header_macro_name = file_name.replace("-", "_").upper() + "_H"
#             code = f"#ifndef {header_macro_name}\n#define {header_macro_name}\n\n{code}\n#endif"
#         return code
        
# def recreate_files_from_metadata(metadata: dict[str, CFileMetadata], location: str):
#     import shutil
#     shutil.rmtree(location, ignore_errors=True)
#     for file_name, file_metadata in metadata.items():
#         new_file_name = os.path.join(location, file_name)
#         os.makedirs(os.path.dirname(new_file_name), exist_ok=True)
#         with open(new_file_name, "w") as f:
#             f.write(file_metadata.recreate_code())


if __name__ == "__main__":
    proj_name = "bzp"
    data = preprocess(f"data/{proj_name}", ["include", "src"], f"preprocessed_data/{proj_name}")
    metadata = get_metadata(data)
    recreate_files_from_metadata(metadata, f"recreated_data/{proj_name}")
    with open("metadata.json", "w") as f:
        json.dump(metadata, f, default=lambda o: o.__dict__(), indent=4, ensure_ascii=False)