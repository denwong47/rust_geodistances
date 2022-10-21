use pyo3::prelude::*;

use crate::data::structs;

impl ToPyObject for structs::LatLng {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        return (&self.lat, &self.lng).to_object(py)
    }
}

impl ToPyObject for structs::CoordinateList {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        return self.value().to_object(py)
    }
}

impl ToPyObject for structs::CalculationResult {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        return match self {
            Self::Geodistance(distance_option) => {
                distance_option.to_object(py)
            },
            Self::WithinDistance(value) => {
                value.to_object(py)
            },
            Self::Unpopulated => {
                None::<f64>.to_object(py)
            }
        }
    }
}

impl IntoPy<PyObject> for structs::IOCoordinateLists {
    fn into_py(self, py:Python) -> Py<PyAny> {
        return self.arrays().to_object(py)
    }
}

impl IntoPy<PyObject> for structs::IOResultArray {
    fn into_py(self, py:Python) -> Py<PyAny> {
        return self.array.to_object(py)
    }
}
