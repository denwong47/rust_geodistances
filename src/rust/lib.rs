use pyo3::prelude::*;
use pyo3::exceptions::{PyValueError, PyRuntimeError};

mod data;
mod input_output;
mod geodistances;

mod py_compatibility;

mod config;

use data::structs;
use data::traits::{Slicable};

/// DEBUG FUNCTIONS
#[pyfunction]
fn debug_info() -> PyResult<config::DebugInformation> {
    return Ok(config::DebugInformation::new())
}


/// POINT OPERATIONS

#[pyfunction]
fn distance(
    start: structs::LatLng,
    dest:  structs::LatLng,
    method: Option<&py_compatibility::enums::CalculationMethod>,
) -> PyResult<structs::CalculationResult> {
    let f = match method {
        Some(member) => match member {
            py_compatibility::enums::CalculationMethod::HAVERSINE   => {
                geodistances::distance_between_two_points::<geodistances::Haversine>
            }
            py_compatibility::enums::CalculationMethod::VINCENTY   => {
                geodistances::distance_between_two_points::<geodistances::Vincenty>
            }
            py_compatibility::enums::CalculationMethod::CARTESIAN   => {
                geodistances::distance_between_two_points::<geodistances::Cartesian>
            }
        }
        None => geodistances::distance_between_two_points::<geodistances::Haversine>,
    };

    let result = f((start, dest));
    return match result {
        structs::CalculationResult::Geodistance(Some(_)) => Ok(result),
        structs::CalculationResult::Geodistance(None) => Err(PyValueError::new_err(
            format!("Cannot calculate distance between {:?} and {:?}.", start, dest)
        )),
        _ => Err(PyRuntimeError::new_err(
            format!("Result type error during distance calculation betwen {:?} and {:?}.",
                    start, dest)
        ))
    }
}


#[pyfunction]
fn offset(
    start: structs::LatLng,
    distance: f64,
    bearing: f64,
    method: Option<&py_compatibility::enums::CalculationMethod>,
)-> PyResult<structs::CalculationResult> {
    let f = match method {
        Some(member) => match member {
            py_compatibility::enums::CalculationMethod::HAVERSINE   => {
                geodistances::offset_by_vector_from_point::<geodistances::Haversine>
            }
            py_compatibility::enums::CalculationMethod::VINCENTY   => {
                geodistances::offset_by_vector_from_point::<geodistances::Vincenty>
            }
            py_compatibility::enums::CalculationMethod::CARTESIAN   => {
                geodistances::offset_by_vector_from_point::<geodistances::Cartesian>
            }
        }
        None => geodistances::offset_by_vector_from_point::<geodistances::Haversine>,
    };

    let result = f(start, distance, bearing);
    return match result {
        structs::CalculationResult::Location(Some(_)) => Ok(result),
        structs::CalculationResult::Location(None) => Err(PyValueError::new_err(
            format!("Cannot calculate offset from {:?} at {:?}º for {}km.", start, bearing, distance)
        )),
        _ => Err(PyRuntimeError::new_err(
            format!("Result type error offset calculation from {:?} at {:?}º for {}km.",
                    start, bearing, distance)
        ))
    }
}

/// =====================================================================
/// ARRAY OPERATIONS

#[pyfunction]
fn distance_map(
    input: structs::IOCoordinateLists,
    // origin: Option<(usize, usize)>,
    // size: Option<(usize, usize)>,
    method: Option<&py_compatibility::enums::CalculationMethod>,
) -> PyResult<structs::IOResultArray> {
    // let _origin = origin.unwrap_or_else(|| (0,0));
    // let _size   = size.unwrap_or_else(|| input.shape());

    let f = match method {
        Some(member) => match member {
            py_compatibility::enums::CalculationMethod::HAVERSINE   => {
                geodistances::distance_map::<geodistances::Haversine>
            }
            py_compatibility::enums::CalculationMethod::VINCENTY   => {
                geodistances::distance_map::<geodistances::Vincenty>
            }
            py_compatibility::enums::CalculationMethod::CARTESIAN   => {
                geodistances::distance_map::<geodistances::Cartesian>
            }
        }
        None => geodistances::distance_map::<geodistances::Haversine>,
    };

    return Ok(
        f(
            &input,
            // _origin,
            // _size,
            None,
        )
    )
}

#[pyfunction]
fn within_distance_map(
    input: structs::IOCoordinateLists,
    distance: f64,
    origin: Option<(usize, usize)>,
    size: Option<(usize, usize)>,
    method: Option<&py_compatibility::enums::CalculationMethod>,
) -> PyResult<structs::IOResultArray> {
    let _origin = origin.unwrap_or_else(|| (0,0));
    let _size   = size.unwrap_or_else(|| input.shape());

    if distance < 0. {
        return Err(
            PyValueError::new_err(
                format!(
                    "Distance supplied must be >0, yet {} found.",
                    distance
                )
            )
        )
    }

    let f = match method {
        Some(member) => match member {
            py_compatibility::enums::CalculationMethod::HAVERSINE   => {
                geodistances::within_distance_map_unthreaded::<geodistances::Haversine>
            }
            py_compatibility::enums::CalculationMethod::VINCENTY   => {
                geodistances::within_distance_map_unthreaded::<geodistances::Vincenty>
            }
            py_compatibility::enums::CalculationMethod::CARTESIAN   => {
                geodistances::within_distance_map_unthreaded::<geodistances::Cartesian>
            }
        }
        None => geodistances::within_distance_map_unthreaded::<geodistances::Haversine>,
    };

    return Ok(
        f(
            &input,
            _origin,
            _size,
            distance,
        )
    )
}

/// =====================================================================
/// A Python module implemented in Rust.
#[pymodule]
fn lib_rust_geodistances(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(debug_info, m)?)?;

    m.add_function(wrap_pyfunction!(distance, m)?)?;
    m.add_function(wrap_pyfunction!(offset, m)?)?;

    m.add_function(wrap_pyfunction!(distance_map, m)?)?;
    m.add_function(wrap_pyfunction!(within_distance_map, m)?)?;

    m.add_class::<py_compatibility::enums::CalculationMethod>()?;

    Ok(())
}
