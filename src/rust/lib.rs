/// Main Library file.
///
/// This is the main rust file that provides the PyO3 bindings; it declares a
/// Python module `lib_rust_geodistances`, which contains all Rust interfaces
/// with Python.
///
/// This file also needs to declare all the `#[cfg(test)]` as submodules to
/// allow `cargo test` to pick them up; this is currently done inside the
/// `tests` submodule.

#[allow(unused_imports)]
use duplicate::duplicate_item;

use pyo3::prelude::*;

#[allow(unused_imports)]
use ndarray_numeric::{
    F64Array,
    F64Array1,
    F64LatLngArray,
    ArrayWithF64Methods,
};

pub mod compatibility;
pub mod calc_models;

mod tests;

/// Internal module (lib.rs) in Rust exposes the Rust endpoints to Python.
///
/// This module is written in Rust and compiled by ``pyo3`` + ``maturin`` with Python
/// bindings.
///
/// .. versionchanged:: 0.2.0
///     This module now no longer contains any functions; instead of calling:
///
///         >>> from rust_geodistances import distance, CalculationMethod
///         >>> distance(s, e, method=CalculationMethod.HAVERSINE)
///
///     Now you can simply call:
///
///         >>> from rust_geodistances import haversine
///         >>> haversine.distance(s, e)
///
///     :attr:`~rust_geodistances.haversine` is simply an alias for
///     :attr:`rust_geodistances.lib_rust_geodistances.CalculationMethod.HAVERSINE`.
///
/// Most if not all of the objects within this module are already exposed at the top
/// level by :mod:`rust_geodistances`, so importing this module is typically not
/// required.
///
/// This module is also accessible as :attr:`rust_geodistances.bin`.
#[pymodule]
fn lib_rust_geodistances(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<compatibility::enums::CalculationMethod>()?;
    // Or should we compatibility::enums::CalculationSettings??
    m.add_class::<calc_models::config::CalculationSettings>()?;

    Ok(())
}
