use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

pub use slope::mk;
pub use slope::linreg;

#[pyfunction]
#[pyo3(name = "mk")]
fn mk_py(xs: Vec<f64>, ys: Vec<f64>) -> PyResult<(f64, f64)> {
    if xs.len() != ys.len() {
        return Err(PyValueError::new_err("xs and ys do not have identical lengths"));
    }
    let (slope, p) = mk(&xs[..], &ys[..]);
    Ok((slope, p))
}

#[pyfunction]
#[pyo3(name = "linreg")]
fn linreg_py(xs: Vec<f64>, ys: Vec<f64>) -> PyResult<(f64, f64)> {
    if xs.len() != ys.len() {
        return Err(PyValueError::new_err("xs and ys do not have identical lengths"));
    }
    let (slope, p) = linreg(&xs[..], &ys[..]);
    Ok((slope, p))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule(name = "slope")]
fn pyslope(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mk_py, m)?)?;
    m.add_function(wrap_pyfunction!(linreg_py, m)?)?;
    Ok(())
}
