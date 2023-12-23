# Development

## Prerequisites

* [Rust](https://www.rust-lang.org/) (latest version if possible, the earliest supported release is at worst 1 year old)
## Build for Development

```shell
git clone https://github.com/arihant2math/weathercli.git
```

To build, run:

```shell
cargo build
```

To have a development-friendly experience, run the following commands

```shell
weather config DEVELOPMENT true
weather config DEBUG true
```

## Build Release Executable

Just run:

```shell
cargo build -r
```

## Scripts

Scripts automate daily tasks

### Update Docs Templates

This script downloads the latest artifacts from GitHub Actions and replaces the executables in docs_templates/ with the
new artifacts.
A GitHub PAT is needed for the script to work.

```shell
./dev-scripts/target/debug/dev-scripts update-docs [gh token here]
```

### Update Index Hashes

```shell
./dev-scripts/target/debug/dev-scripts update-index-hashes
```