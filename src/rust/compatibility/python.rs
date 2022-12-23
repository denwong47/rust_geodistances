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
    // ArrayFromDuplicatedRows,

    // F64Array1,
    // F64Array2,
};

use crate::calc_models::config;
use super::{enums, CalculationInterfaceInternal};

#[pymethods]
impl enums::CalculationMethod {
    #[pyo3(text_signature = "($self, s, e, *, settings)")]
    /// Great-circle distances from a point to an array of lat-long coordinates.
    ///
    /// Parameters
    /// ----------
    /// s: numpy.ndarray
    ///     Of dimension ``(2)``, e.g. ``numpy.array([51.5072, -0.1276])``.
    ///
    /// e: numpy.ndarray
    ///     Of dimension ``(n, 2)``.
    ///
    /// settings: CalculationSettings
    ///     Settings to be passed on to the calculation method.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray (dtype=numpy.float64)
    ///     An array of great-circle distances mapping each point in
    ///     `e` to `s`.
    ///
    /// Example
    /// -------
    /// Mapping distances between two arrays::
    ///
    ///     >>> import numpy as np; import random
    ///     >>> from rust_geodistances.lib_rust_geodistances import CalculationMethod, CalculationSettings
    ///     >>> from rust_geodistances import haversine
    ///
    ///     >>> sn = en = np.array(
    ///     ...     [
    ///     ...         (random.random()*180-90, random.random()*360-180)
    ///     ...         for _ in range(8)
    ///     ...     ]
    ///     ... )
    ///
    ///     >>> haversine.distance(sn, en, settings = CalculationSettings(workers=6))
    ///     array([[    0.        , 12908.50478944,  9875.9964588 ,  8626.94507491,
    ///              7212.0267238 ,  8036.96348564, 16613.63077234,  6734.12355525],
    ///            [12908.50478944,     0.        , 11875.45259788,  8204.64386099,
    ///             13598.27403101, 18342.14917607,  3828.7790652 , 17495.93671898],
    ///            [ 9875.9964588 , 11875.45259788,     0.        , 18313.98382482,
    ///              2771.77544953,  6625.97278549, 11796.96939483,  5828.42501612],
    ///            [ 8626.94507491,  8204.64386099, 18313.98382482,     0.        ,
    ///             15838.96432924, 13465.30033574,  9248.00225409, 13729.37952497],
    ///            [ 7212.0267238 , 13598.27403101,  2771.77544953, 15838.96432924,
    ///                 0.        ,  5333.41414913, 14498.75424006,  3897.67499337],
    ///            [ 8036.96348564, 18342.14917607,  6625.97278549, 13465.30033574,
    ///              5333.41414913,     0.        , 15378.86092402,  1775.65767549],
    ///            [16613.63077234,  3828.7790652 , 11796.96939483,  9248.00225409,
    ///             14498.75424006, 15378.86092402,     0.        , 16542.79712392],
    ///            [ 6734.12355525, 17495.93671898,  5828.42501612, 13729.37952497,
    ///              3897.67499337,  1775.65767549, 16542.79712392,     0.        ]])
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
    /// numpy.ndarray (dtype=numpy.float64)
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

    #[pyo3(text_signature = "($self, s, e, distance, bearing, *, settings)")]
    /// Offset an array of coordinates by a vector.
    fn offset(
        &self,
        s: &PyArray<f64, Ix2>,
        distance: f64,
        bearing: f64,
        settings: Option< &config::CalculationSettings>,
        py: Python<'_>,
    ) -> PyResult<PyObject> {
        let result = {
            CalculationInterfaceInternal::<f64>::_offset(
                self,
                &s.to_owned_array(),
                distance,
                bearing,
                settings,
            )
            .to_pyarray(py)
        };

        return Ok(result.into_py(py));
    }

    // TODO Check how arrays are implemented in offset again and see if this is possible at all
    // #[pyo3(text_signature = "($self, s, e, distance, bearing, *, settings)")]
    // /// Offset an array of coordinates by a vectors of equal length.
    // fn offset_group(
    //     &self,
    //     s: &PyArray<f64, Ix2>,
    //     distance: &PyArray<f64, Ix1>,
    //     bearing: &PyArray<f64, Ix1>,
    //     settings: Option< &config::CalculationSettings>,
    //     py: Python<'_>,
    // ) -> PyResult<PyObject> {
    //     let result = {
    //         CalculationInterfaceInternal::<&F64Array1>::_offset(
    //             self,
    //             &s.to_owned_array(),
    //             &F64Array2::from_duplicated_rows(distance.to_owned_array().view()),
    //             &F64Array2::from_duplicated_rows(bearing.to_owned_array().view()),
    //             settings,
    //         )
    //         .to_pyarray(py)
    //     };

    //     return Ok(result.into_py(py));
    // }

    #[pyo3(text_signature = "($self, s, e, distance, *, settings)")]
    /// Check if array of lat-long coordinates is within great-circle distance of point.
    ///
    /// Parameters
    /// ----------
    /// s: numpy.ndarray
    ///     Of dimension ``(2)``, e.g. ``numpy.array([51.5072, -0.1276])``.
    ///
    /// e: numpy.ndarray
    ///     Of dimension ``(n, 2)``.
    ///
    /// distance: numpy.float64
    ///     Distance to check against.
    ///     The unit of this must be the same as that of:
    ///
    ///     - :attr:`CalculationSettings.spherical_radius` or
    ///     - :attr:`CalculationSettings.ellipse_a` and
    ///     - :attr:`CalculationSettings.ellipse_b` and
    ///     - :attr:`CalculationSettings.ellipse_f`
    ///
    ///     whichever used by in the :class:`CalculationMethod`.
    ///
    /// settings: CalculationSettings
    ///     Settings to be passed on to the calculation method.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray (dtype=bool)
    ///     Dimension `(n)`.
    ///     An array of great-circle distances mapping each point in
    ///     `e` to `s`.
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
    /// Check if each point from ``s`` is within ``distance`` of that of ``e``.
    ///
    /// Parameters
    /// ----------
    /// s: numpy.ndarray
    ///     Of dimension ``(n, 2)``.
    ///
    /// e: numpy.ndarray
    ///     Of dimension ``(n, 2)``.
    ///
    /// distance: numpy.float64
    ///     Distance to check against.
    ///     The unit of this must be the same as that of:
    ///
    ///     - :attr:`CalculationSettings.spherical_radius` or
    ///     - :attr:`CalculationSettings.ellipse_a` and
    ///     - :attr:`CalculationSettings.ellipse_b` and
    ///     - :attr:`CalculationSettings.ellipse_f`
    ///
    ///     whichever used by in the :class:`CalculationMethod`.
    ///
    /// settings: CalculationSettings
    ///     Settings to be passed on to the calculation method.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray (dtype=bool)
    ///     An array of great-circle distances mapping each point in
    ///     `s` to each point in `e`.
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
    /// Indices of points in ``e`` that are within great-circle ``distance`` of ``e``.
    ///
    /// Parameters
    /// ----------
    /// s: numpy.ndarray
    ///     Of dimension ``(2)``, e.g. ``numpy.array([51.5072, -0.1276])``.
    ///
    /// e: numpy.ndarray
    ///     Of dimension ``(n, 2)``.
    ///
    /// distance: numpy.float64
    ///     Distance to check against.
    ///     The unit of this must be the same as that of:
    ///
    ///     - :attr:`CalculationSettings.spherical_radius` or
    ///     - :attr:`CalculationSettings.ellipse_a` and
    ///     - :attr:`CalculationSettings.ellipse_b` and
    ///     - :attr:`CalculationSettings.ellipse_f`
    ///
    ///     whichever used by in the :class:`CalculationMethod`.
    ///
    /// settings: CalculationSettings
    ///     Settings to be passed on to the calculation method.
    ///
    /// Returns
    /// -------
    /// numpy.ndarray (dtype=numpy.uint64)
    ///     ``numpy.ndarray`` of ``dtype`` ``numpy.uint64`` containing the indices of
    ///     all points in ``e`` within ``distance`` of ``s[i]``.
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
    /// Indices of all points from ``s`` is within ``distance`` of that of ``e``.
    ///
    /// Parameters
    /// ----------
    /// s: numpy.ndarray
    ///     Of dimension ``(n, 2)``.
    ///
    /// e: numpy.ndarray
    ///     Of dimension ``(n, 2)``.
    ///
    /// distance: numpy.float64
    ///     Distance to check against.
    ///     The unit of this must be the same as that of:
    ///
    ///     - :attr:`CalculationSettings.spherical_radius` or
    ///     - :attr:`CalculationSettings.ellipse_a` and
    ///     - :attr:`CalculationSettings.ellipse_b` and
    ///     - :attr:`CalculationSettings.ellipse_f`
    ///
    ///     whichever used by in the :class:`CalculationMethod`.
    ///
    /// settings: CalculationSettings
    ///     Settings to be passed on to the calculation method.
    ///
    /// Returns
    /// -------
    /// Tuple[numpy.ndarray (dtype=numpy.uint64)]
    ///     Each ``returned[i]`` contains a ``numpy.ndarray`` of ``dtype``
    ///     ``numpy.uint64`` that are the indices of all points in ``e`` within
    ///     ``distance`` of ``s[i]``.
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
