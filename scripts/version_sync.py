import ast
import datetime
import tomllib

import tomli_w

from path_helper import weathercli_dir

corelib_cargo_toml = weathercli_dir / "Cargo.toml"
updater_cargo_toml = weathercli_dir / "updater" / "Cargo.toml"
cli_version = weathercli_dir / "cli" / "version.py"
now = datetime.datetime.now()
version_string = "{}.{}.{}".format(now.year, now.month, now.day)
t = tomllib.load(corelib_cargo_toml.open("rb"))
t["package"]["version"] = version_string
tomli_w.dump(t, corelib_cargo_toml.open("wb"))
t = tomllib.load(updater_cargo_toml.open("rb"))
t["package"]["version"] = version_string
tomli_w.dump(t, updater_cargo_toml.open("wb"))
a = ast.parse(cli_version.open().read())
a.body.pop()
a.body.append(
    ast.Assign(
        targets=[ast.Name(id="__version__", ctx=ast.Store())],
        value=ast.Constant(value=version_string),
        lineno=0,
    )
)
cli_version.open("w").write(ast.unparse(a))
