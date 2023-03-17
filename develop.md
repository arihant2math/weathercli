# Development
## Prerequisites
* [Python](https://python.org) (3.9+)
* Rust
## Build for Development
```shell
git clone https://github.com/arihant2math/weathercli.git
```
First build corelib:
```shell
pip install maturin
maturin develop
```
Now install python dependencies:
```shell
pip install -r dev-requirements.txt
```
... and run with the command `python weather.py`
To have a development friendly experience run the following commands
```shell
python weather.py config DEVELOPMENT true
python weather.py config DEBUG true
```
## Build Release Executable
```shell
maturin build -r
```
Now install the wheel at `target/wheels/`, you can manually type the wheel name, or you can use one of these scripts.

Bash/ZSH:
```shell
for file in target/wheels/*.whl
do
  name=${file##*/}
  pip install "$file"
done
```
Powershell:
```shell
$files = Get-ChildItem "target/wheels/"
foreach ($f in $files) {
    pip install $f.FullName
}
```
Now install python dependencies:
```shell
pip install -r requirements.txt
pip install pyinstaller
```
And we can build the executable:
```shell
pyinstaller -F weather.py -i ./icon/icon.png
```
The executable will be in `build/`
## Scripts
Scripts automate daily tasks
### Generate Stubs
Many IDEs use stub files to detect functions and annotations, pyo3 does not automatically generate them, to generate them run the following command:
```shell
python ./scripts/generate_stubs_v2.py core ./venv/Lib/core
```
### Update Docs Templates
This script downloads the latest artifacts from GitHub Actions and replaces the executables in docs_templates/ with the new artifacts.
A GitHub PAT is needed for the script to work.
```shell
python scripts/update_docs_templates.py [gh token here]
```
### Update Index Hashes
Should be run after the docs templates are updated
```shell
python scripts/update_index_hashes.py
```
### Version Sync
Updates the date everywhere
```shell
python scripts/version_sync.py
```
### Docs
Run `make docs`, or
```shell
python scripts/dev_helper.py docs
```