![GitHub Workflow Status (branch)](https://img.shields.io/github/actions/workflow/status/zachcoleman/typed-lists/test.yml?branch=main)
![PyPI - Wheel](https://img.shields.io/pypi/wheel/typed-lists)
[![License](https://img.shields.io/badge/license-Apache2.0-green)](./LICENSE)

# typed-lists
`typed-lists` is a library for typed, parallelized lists for Python implemented in Rust. The project was develop using [maturin](https://maturin.rs). 

The project heavily depends on [`rayon`](https://github.com/rayon-rs/rayon) and PyO3 [`pyo3`](https://github.com/PyO3) and takes inspiration from `numpy`, Rust's `ndarray`, and many more packages.

## Installation
From PyPI:
```shell
pip install typed-lists 
```

Build from source:
```
maturin build -r -i=path/to/python
pip install .../typed-lists/target/wheels/<whl file name>.whl
```
_Note: requires Rust installation_

## Usage
See `./example.ipynb`.

## Running Tests
Tests are run with `pytest`.
