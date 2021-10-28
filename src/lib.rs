use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

create_exception!(calculator, ZeroDivisionError, PyException);

/// Returns a float add of values in the float args sequence.
///
/// When the args is empty, return the 0.0.
/// When the args length is 1, return the first value.
#[pyfunction(args = "*")]
#[pyo3(text_signature = "(*args)")]
fn add(args: &PyTuple) -> f64 {
    let mut result: f64 = 0.0;
    for item in args.as_slice() {
        result += item.extract::<f64>().unwrap();
    }
    return result;
}

/// Returns a float subtract of values in the float args sequence.
///
/// When the args is empty, return the 0.0.
/// When the args length is 1, return the first value.
#[pyfunction(args = "*")]
#[pyo3(text_signature = "(*args)")]
fn subtract(args: &PyTuple) -> f64 {
    if args.len() == 0 {
        return 0.0;
    }

    let slice = args.as_slice();
    let mut result: f64 = slice[0].extract().unwrap();
    let mut index: usize = 1;
    while index < args.len() {
        result -= slice[index].extract::<f64>().unwrap();

        index += 1;
    }
    return result;
}

/// Returns a float multiply of values in the float args sequence.
///
/// When the args is empty, return the 0.0.
/// When the args length is 1, return the first value.
#[pyfunction(args = "*")]
#[pyo3(text_signature = "(*args)")]
fn multiply(args: &PyTuple) -> f64 {
    if args.len() == 0 {
        return 0.0;
    }

    let mut result: f64 = 1.0;
    for item in args.as_slice() {
        result *= item.extract::<f64>().unwrap();
    }
    return result;
}

/// Returns a float divide of values in the float args sequence.
///
/// When the args is empty, return the 0.0.
/// When the args length is 1, return the first value.
#[pyfunction(args = "*")]
#[pyo3(text_signature = "(*args)")]
fn divide(args: &PyTuple) -> Result<f64, PyErr> {
    if args.len() == 0 {
        return Ok(0.0);
    }

    let slice = args.as_slice();
    let mut result: f64 = slice[0].extract().unwrap();
    let mut index: usize = 1;
    while index < args.len() {
        let number = slice[index].extract::<f64>().unwrap();
        if number == 0.0 {
            return Err(ZeroDivisionError::new_err("Division by zero"));
        }
        result /= number;

        index += 1;
    }
    return Ok(result);
}

/// Calculator
///
/// This module provides a calculator written in Rust.
#[pymodule]
fn calculator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("ZeroDivisionError", _py.get_type::<ZeroDivisionError>())?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(subtract, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(divide, m)?)?;

    Ok(())
}
