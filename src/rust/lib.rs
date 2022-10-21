use pyo3::prelude::*;

mod data;
mod input_output;
mod geodistances;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn usizes(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn string(s: String) -> PyResult<String> {
    Ok(s.to_lowercase())
}

#[pyfunction]
fn vecu32(v: Vec<u32>) -> PyResult<u32> {
    Ok(v.iter().sum())
}

#[pyfunction]
fn vecvecf64(v: Vec<Vec<f64>>) -> PyResult<Vec<f64>> {
    Ok(v.iter().map(|x| x.iter().sum()).collect())
}

/// A Python module implemented in Rust.
#[pymodule]
fn lib_rust_geodistances(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(usizes, m)?)?;
    m.add_function(wrap_pyfunction!(string, m)?)?;
    m.add_function(wrap_pyfunction!(vecu32, m)?)?;
    m.add_function(wrap_pyfunction!(vecvecf64, m)?)?;
    Ok(())
}
