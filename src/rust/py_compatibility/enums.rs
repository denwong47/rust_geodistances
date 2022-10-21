use pyo3::prelude::*;

#[pyclass(module="rust_geodistances")]
pub enum CalculationMethod {
    HAVERSINE,
    VINCENTY,
    CARTESIAN,
}
