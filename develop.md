# Development

## Prerequisites

* [Rust](https://www.rust-lang.org/) (1.60+, the earliest supported release is at worst 1 year old)
* [Python](https://python.org) (3.9+, any supported python version is supported, optional and only used for some
  scripts)

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
[command coming soon] [gh token here]
```

### Update Index Hashes

Documentation coming soon
