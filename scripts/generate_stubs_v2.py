import ast
import importlib
import inspect
import subprocess
from pathlib import Path
from typing import Union

import click


class PyModule:
    def __init__(self, text, path):
        self.text = text
        self.path = path


class Component:
    def __init__(self, name, doc):
        self.name = name
        self.doc = doc

    def get_ast(self) -> Union[ast.stmt, list[list]]:
        pass


def blank():
    pass


class Function(Component):
    def __init__(self, obj):
        super().__init__(obj.__name__, obj.__doc__)
        try:
            self.signature = inspect.signature(obj)
        except:
            self.signature = inspect.signature(blank)

    def get_ast(self):
        arg = []
        for s in self.signature.parameters:
            if self.signature.parameters[s].default != Ellipsis:
                arg.append(ast.arg(s))
            else:
                arg.append(ast.arg(s))
        args = ast.arguments(
            posonlyargs=[],
            args=arg,
            defaults=[],
            kwonlyargs=[],
        )
        body = []
        if self.doc is not None:
            body.append(ast.Expr(value=ast.Constant(value=self.doc)))
        body.append(ast.Expr(value=ast.Constant(value=Ellipsis)))
        return ast.FunctionDef(
            self.name,
            args,
            decorator_list=[],
            lineno=0,
            body=body,
        )


class Cls(Component):
    def __init__(self, obj):
        super().__init__(obj.__name__, obj.__doc__)
        self.attributes = dir(obj)
        self.functions = [
            Function(getattr(obj, a))
            for a in self.attributes
            if (inspect.isroutine(getattr(obj, a))) or (obj in ["__new__"])
        ]
        self.variables = []
        for v in self.attributes:
            if type(getattr(obj, v)).__name__ == "getset_descriptor":
                self.variables.append(getattr(obj, v))

    def get_ast(self):
        ast_def = ast.ClassDef(
            self.name, keywords=[], bases=[], decorator_list=[]
        )  # TODO: Fix inheritance just in case
        ast_def.body = []
        if self.doc is not None:
            ast_def.body.append(ast.Expr(value=ast.Constant(value=self.doc)))
        for fn in self.functions:
            ast_def.body.append(fn.get_ast())
        for v in self.variables:
            ast_def.body.append(ast.Assign(targets=[ast.Name(id=v.__name__, ctx=ast.Store())],
                                           value=ast.Constant(value=Ellipsis),
                                           lineno=0))
        if len(ast_def.body) == 0:
            ast_def.body.append(ast.Expr(value=ast.Constant(value=Ellipsis)))
        return ast_def


class Module(Component):
    def __init__(self, obj):
        super().__init__(obj.__name__, obj.__doc__)
        self.children = get_attributes(obj)

    def __str__(self):
        return str(self.children)

    def get_ast(self) -> list[list]:
        module = ast.parse("")
        aux: list[list] = []
        for child in self.children:
            if type(child) == Module:
                aux.append([child.name, child.get_ast()])
                module.body.append(
                    ast.ImportFrom(module=self.name, names=[ast.alias(child.name)])
                )
        for child in self.children:
            if type(child) != Module:
                module.body.append(child.get_ast())
        aux.append(["__init__", module])
        return aux


def get_attributes(module) -> list[Component]:
    all_components = dir(module)
    real_components = []
    for component in all_components:
        real_component = getattr(module, component)
        # print(component, real_component, inspect.isroutine(real_component), isinstance(real_component, type),
        # hasattr(real_component, "__all__"), inspect.isclass(real_component))
        if component != "sys" and component != module.__name__:
            if inspect.ismodule(real_component):
                real_components.append(Module(real_component))
            elif inspect.isclass(real_component):
                real_components.append(Cls(real_component))
            elif inspect.isroutine(real_component):
                real_components.append(Function(real_component))
    return real_components


def format_with_black(code: str) -> str:
    result = subprocess.run(
        ["python", "-m", "black", "-t", "py38", "--pyi", "-"],
        input=code.encode(),
        capture_output=True,
    )
    result.check_returncode()
    return result.stdout.decode()


def write(out_dir: Path, files):
    out_dir.mkdir(exist_ok=True)
    for file in files:
        if isinstance(file[1], ast.Module):
            (out_dir / (file[0] + ".pyi")).open("w").write(ast.unparse(file[1]))
        else:
            write(out_dir / file[0], file[1])


@click.command()
@click.argument(
    "module_name",
    # help="Name of the Python module for which generate stubs"
)
@click.argument(
    "out",
    # help="Name of the Python stub file to write to"
)
@click.option("--black", is_flag=True, help="Formats the generated stubs using Black")
def main(module_name, out, black):
    ast_gen: list[list] = Module(importlib.import_module(module_name)).get_ast()
    if black:
        pass
        # stub_contents = [format_with_black(stub_content) for stub.text in stub_contents]
    write(Path(out), ast_gen)


if __name__ == "__main__":
    main(obj={})
    # m = Module(importlib.import_module("core")).get_ast()
    # print(m[4])
    # print(ast.dump(m[4][1], indent=4))
