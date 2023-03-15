import tomllib
import datetime
import tomli_w
import ast

now = datetime.datetime.now()
version_string = "{}.{}.{}".format(now.year, now.month, now.day)
t = tomllib.load(open("../Cargo.toml", "rb"))
t["package"]["version"] = version_string
tomli_w.dump(t, open("../Cargo.toml", "wb"))
t = tomllib.load(open("../updater/Cargo.toml", "rb"))
t["package"]["version"] = version_string
tomli_w.dump(t, open("../updater/Cargo.toml", "wb"))
a = ast.parse(open("../cli/version.py").read())
a.body.pop()
a.body.append(
    ast.Assign(
        targets=[ast.Name(id="__version__", ctx=ast.Store())],
        value=ast.Constant(value=version_string),
        lineno=0,
    )
)
open("../cli/version.py", "w").write(ast.unparse(a))
