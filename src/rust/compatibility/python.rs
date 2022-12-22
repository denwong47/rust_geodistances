// Python implementation to add PyMethods to CalculationMethod enum struct.
// but uses ndarrays for all parameters and returns.
use pyo3::prelude::*;
use pyo3::types::{
    PyTuple,
};

use numpy::ndarray::{
    Ix1,
    Ix2,
};
use numpy::{
    ToPyArray,
    PyArray,
};

use ndarray_numeric::{
    ArrayWithBoolIterMethods,
};

use crate::calc_models::config;
use super::{enums, CalculationInterfaceInternal};

#[pymethods]
impl enums::CalculationMethod {
    #[pyo3(text_signature = "($self, s, e, *, settings)")]
    fn distance_from_point(
        &self,
        s: &PyArray<f64, Ix1>,
        e: &PyArray<f64, Ix2>,
        settings: Option< &config::CalculationSettings>,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        let result = {
            CalculationInterfaceInternal::<f64>::_distance_from_point(
                self,
                &s.to_owned_array(), &e.to_owned_array(),
                settings,
            )
            .to_pyarray(py)
        };

        return Ok(result.into_py(py));
    }

    #[pyo3(text_signature = "($self, s, e, *, settings)")]
    /// Great-circle distances between two arrays of lat-long coordinates.
    ///
    /// Parameters
    /// ----------
    /// s: numpy.ndarray
    ///     Of dimension ``(n, 2)``.
    ///
    /// e: numpy.ndarray
    ///     Of dimension ``(n, 2)``.
    ///
    /// settings: CalculationSettings
    ///     Settings to be passed on to the calculation method.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     An array of great-circle distances mapping each point in
    ///     `s` to each point in `e`.
    fn distance(
        &self,
        s: &PyArray<f64, Ix2>,
        e: &PyArray<f64, Ix2>,
        settings: Option< &config::CalculationSettings>,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        let (s_native, e_native) = (&s.to_owned_array(), &e.to_owned_array());
        let shape = (s_native.shape()[0], e_native.shape()[0]);

        let result = {
            CalculationInterfaceInternal::<f64>::_distance(
                self,
                &s.to_owned_array(), &e.to_owned_array(),
                shape,
                settings,
            )
            .to_pyarray(py)
        };

        return Ok(result.into_py(py));
    }

    // fn offset(
    //     &self,
    //     s: &PyArray<f64, Ix2>,
    //     distance: T,
    //     bearing: T,
    //     settings: Option< &config::CalculationSettings>,
    //     py: Python<'_>,
    // ) -> PyResult<PyObject>;

    #[pyo3(text_signature = "($self, s, e, distance, *, settings)")]
    fn within_distance_from_point(
        &self,
        s: &PyArray<f64, Ix1>,
        e: &PyArray<f64, Ix2>,
        distance: f64,
        settings: Option<&config::CalculationSettings>,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        let result = {
            CalculationInterfaceInternal::<f64>::_within_distance_of_point(
                self,
                &s.to_owned_array(), &e.to_owned_array(),
                distance,
                settings,
            )
            .to_pyarray(py)
        };

        return Ok(result.into_py(py));
    }

    #[pyo3(text_signature = "($self, s, e, distance, *, settings)")]
    fn within_distance(
        &self,
        s: &PyArray<f64, Ix2>,
        e: &PyArray<f64, Ix2>,
        distance: f64,
        settings: Option< &config::CalculationSettings>,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        let (s_native, e_native) = (&s.to_owned_array(), &e.to_owned_array());
        let shape = (s_native.shape()[0], e_native.shape()[0]);

        let result = {
            CalculationInterfaceInternal::<f64>::_within_distance(
                self,
                &s.to_owned_array(), &e.to_owned_array(),
                distance,
                shape,
                settings,
            )
            .to_pyarray(py)
        };

        return Ok(result.into_py(py));
    }

    #[pyo3(text_signature = "($self, s, e, distance, *, settings)")]
    fn indices_within_distance_of_point(
        &self,
        s: &PyArray<f64, Ix1>,
        e: &PyArray<f64, Ix2>,
        distance: f64,
        settings: Option<&config::CalculationSettings>,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        let result = {
            CalculationInterfaceInternal::<f64>::_within_distance_of_point(
                self,
                &s.to_owned_array(), &e.to_owned_array(),
                distance,
                settings,
            )
            .indices()
            .to_pyarray(py)
        };

        return Ok(result.into_py(py));
    }

    #[pyo3(text_signature = "($self, s, e, distance, *, settings)")]
    fn indices_within_distance(
        &self,
        s: &PyArray<f64, Ix2>,
        e: &PyArray<f64, Ix2>,
        distance: f64,
        settings: Option<&config::CalculationSettings>,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        let (s_native, e_native) = (&s.to_owned_array(), &e.to_owned_array());
        let shape = (s_native.shape()[0], e_native.shape()[0]);

        let result = {
            PyTuple::new(
                py,
                CalculationInterfaceInternal::<f64>::_within_distance(
                    self,
                    s_native, e_native,
                    distance,
                    shape,
                    settings,
                )
                .rows()
                .into_iter()
                .map(
                    |row| row.indices().to_pyarray(py)
                )
            )
        };

        return Ok(result.into_py(py));
    }
}
