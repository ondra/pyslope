use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

const VERSION: &str = git_version::git_version!(args=["--tags", "--always", "--dirty"]);
#[pyfunction]
fn version() -> PyResult<String> {
    Ok(VERSION.to_string())
}

fn validate(xs: &Vec<f64>, ys: &Vec<f64>) -> PyResult<()> {
    if xs.len() < 2 {
        return Err(PyValueError::new_err("xs and ys must contain at least 2 values"));
    }
    if xs.len() != ys.len() {
        return Err(PyValueError::new_err("xs and ys do not have identical lengths"));
    }
    Ok(())
}

#[pyfunction]
fn mk(xs: Vec<f64>, ys: Vec<f64>) -> PyResult<(f64, f64)> {
    validate(&xs, &ys)?;
    let (p, slope) = slope::mk(&xs[..], &ys[..]);
    Ok((slope, p))
}

#[pyfunction]
fn linreg(xs: Vec<f64>, ys: Vec<f64>) -> PyResult<(f64, f64)> {
    validate(&xs, &ys)?;
    let (p, slope) = slope::linreg(&xs[..], &ys[..]);
    Ok((slope, p))
}

#[pyfunction]
fn mk_intercept(xs: Vec<f64>, ys: Vec<f64>) -> PyResult<(f64, f64, f64)> {
    validate(&xs, &ys)?;
    let (p, slope, intercept) = slope::mk_intercept(&xs[..], &ys[..]);
    Ok((intercept, slope, p))
}

#[pyfunction]
fn linreg_intercept(xs: Vec<f64>, ys: Vec<f64>) -> PyResult<(f64, f64, f64)> {
    validate(&xs, &ys)?;
    let (p, slope, intercept) = slope::linreg_intercept(&xs[..], &ys[..]);
    Ok((intercept, slope, p))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule(name = "slope")]
fn pyslope(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mk, m)?)?;
    m.add_function(wrap_pyfunction!(linreg, m)?)?;
    m.add_function(wrap_pyfunction!(mk_intercept, m)?)?;
    m.add_function(wrap_pyfunction!(linreg_intercept, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
