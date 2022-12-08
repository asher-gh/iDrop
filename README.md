# iDrop

Blazingly fast native cross-platform desktop application for predicting
microfluidic droplet sizes prediction utility application based on TensorFlow
and written in Rust.

# Compiling

At the current stage, the application is still in early development phase and
thus a packaged version is not yet made available. However, you can compile and
try out for yourself.

For PyO3 to work make sure to provide the exact version of python to use. A
shared library must be installed for that version. By default,
`pyo3-build-config` will try to use the virtualenv or system installed version.
This can be done by setting the `PYO3_PYTHON` environment variable to path of
the python interpretor.

```sh
cargo build --release
```

## Requisites

Make sure the following are setup correctly for your system:

- Python (dynamic library for python version)
- Tensorflow2
- Rust toolchain
