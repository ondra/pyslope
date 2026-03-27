# pyslope

`pyslope` is a small Python extension module built with PyO3 on top of the
[`slope`](https://github.com/ondra/slope) Rust crate. It exposes ordinary least
squares regression and Theil-Sen plus Mann-Kendall trend estimation to Python.

## Build and import

Build the extension module with Cargo:

```bash
cargo build --release
```

The raw Cargo-built library is named `libpyslope.so` on Linux. Python imports this module as `slope`, so the shared library must be available under the importable module name for the import to work.

The most portable approach is to rename or copy the library to `slope` plus
Python's extension suffix (`.so` on Linux) and place it into the directory with your program or on `PYTHONPATH`:

## Python API

The compiled Python module is imported as `slope` and provides:

- `linreg(xs, ys) -> (slope, p_value)`
- `linreg_intercept(xs, ys) -> (intercept, slope, p_value)`
- `mk(xs, ys) -> (slope, p_value)`
- `mk_intercept(xs, ys) -> (intercept, slope, p_value)`
- `version() -> str`

`xs` and `ys` must have identical lengths and contain at least two values.

## Python example

```python
import slope

# Input series
xs = [0.0, 1.0, 2.0, 3.0, 4.0]
ys = [1.0, 1.8, 3.2, 3.9, 5.1]

slope_ols, p_ols = slope.linreg(xs, ys)
intercept_mk, slope_mk, p_mk = slope.mk_intercept(xs, ys)

print("OLS:", slope_ols, p_ols)
print("MK:", intercept_mk, slope_mk, p_mk)
```

Verified on this machine with:

- `xs = [0.0, 1.0, 2.0, 3.0, 4.0]`
- `ys = [1.0, 1.8, 3.2, 3.9, 5.1]`
- `slope.linreg(xs, ys) -> (1.0299999999999998, 0.0003337236600056137)`
- `slope.mk_intercept(xs, ys) -> (0.9500000000000002, 1.0374999999999999, 0.027486336110310372)`

Lower p-values indicate stronger evidence for a trend. Positive slopes indicate
an increasing series.
