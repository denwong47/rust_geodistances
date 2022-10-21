use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::data::structs;

impl<'source> FromPyObject<'source> for structs::LatLng {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        let coordinates = Vec::<f64>::extract(ob)?;

        if coordinates.len() == 2 {
            return Ok(Self::new(coordinates[0], coordinates[1]))
        } else {
            return Err(PyValueError::new_err(
                "Expected each latititude-longitude pairs as tuples of (np.float64, np.float64)."
            ))
        }
    }
}

impl<'source> FromPyObject<'source> for structs::CoordinateList {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        return Ok(Self(
            Vec::<structs::LatLng>::extract(ob)?
        ))
    }
}

impl<'source> FromPyObject<'source> for structs::IOCoordinateLists {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        let mut lists = Vec::<structs::CoordinateList>::extract(ob)?;

        return match lists.len() {
            2 => Ok(Self([Some(lists.swap_remove(0)), Some(lists.swap_remove(0))])),
            1 => Ok(Self([Some(lists.swap_remove(0)), None])),
            _ => Err(PyValueError::new_err(
                "One or two list/numpy arrays of (np.float64, np.float64) are expected.",
            ))
        }
    }
}
