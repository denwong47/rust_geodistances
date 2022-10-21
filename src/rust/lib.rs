use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

mod data;
mod input_output;
mod geodistances;

mod py_compatibility;

use data::structs;

// use geodistances::traits::{CalculateDistance, CheckDistance};

const HAVERSINE:&str = "haversine";
const VINCENTY:&str  = "vincenty";
const CARTESIAN:&str = "cartesian";


#[pyfunction]
fn distance_map(
    input: structs::IOCoordinateLists,
    origin: Option<(usize, usize)>,
    size: Option<(usize, usize)>,
    method: Option<&py_compatibility::enums::CalculationMethod>,
) -> PyResult<structs::IOResultArray> {
    let _origin = origin.unwrap_or_else(|| (0,0));
    let _size   = size.unwrap_or_else(|| input.shape());

    let f = match method {
        Some(member) => match member {
            py_compatibility::enums::CalculationMethod::HAVERSINE   => {
                geodistances::distance_map_unthreaded::<geodistances::Haversine>
            }
            py_compatibility::enums::CalculationMethod::VINCENTY   => {
                geodistances::distance_map_unthreaded::<geodistances::Vincenty>
            }
            py_compatibility::enums::CalculationMethod::CARTESIAN   => {
                geodistances::distance_map_unthreaded::<geodistances::Cartesian>
            }
        }
        None => geodistances::distance_map_unthreaded::<geodistances::Haversine>,
    };

    return Ok(
        f (
            &input,
            _origin,
            _size,
        )
    )
}

/// A Python module implemented in Rust.
#[pymodule]
fn lib_rust_geodistances(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(distance_map, m)?)?;

    m.add_class::<py_compatibility::enums::CalculationMethod>()?;

    Ok(())
}
