/// Enum classes.
///
/// The main purpose of this module is the enum of `CalculationMethod`.
/// This Rust `enum`/Python `class` contains a member for each provided
/// calculation model (see `calc_models`), with each function `match`-ing
/// the enum member to call the correct element in `calc_models`.
///
/// While the Enum itself is decorated with PyO3, all methods in this
/// module should be Rust-native, taking `ndarray` and returning the same,
/// so that they can be used directly by Rust as well.

use duplicate::duplicate_item;

use std::cmp;
use std::sync::{Arc, Mutex};

use ndarray::{
    Array1,
    Axis,
    Ix1,
    s,
    Slice,
};
use pyo3::prelude::*;
use rayon::prelude::*;

use ndarray_numeric::{
    ArrayWithBoolIterMethods,
    ArrayWithF64PartialOrd,

    BoolArray1,
    BoolArray2,

    F64Array1,
    F64Array2,
    F64ArcArray1,
    F64ArrayView,
    F64ArrayViewMut,
    F64LatLngArray,

    SquareShapedArray,
};

use crate::calc_models::traits::{
    LatLng,
    LatLngArray,
    CalculateDistance,
    OffsetByVector,
};

use crate::calc_models::{
    Haversine,
    Vincenty,
};

// use super::conversions::{
//     BoolArrayToVecIndex,
// };

// Public import this to unify things.
// It's a Enum afterall.
pub use crate::calc_models::CalculationSettings;

#[pyclass(module="rust_geodistances")]
/// Pseudo-Enum class of all supported calculation models.
///
/// This enum contains members that each represents a calculation method
/// that can:
///
/// - calculate distances,
/// - check adjacency between points, and
/// - displace points by a vector.
///
/// .. versionchanged:: 0.2.0
///     Since all functions are removed from
///     :mod:`~rust_geodistances.lib_rust_geodistances` module, this Enum is now the
///     primary means of accessing the functionalities of the whole library.
///
/// .. note::
///     The members of this class can be accessed via the aliases:
///
///     - :attr:`rust_geodistances.haversine`
///     - :attr:`rust_geodistances.vincenty`
///
/// Methods of this class operates solely on `numpy` data types:
/// n-dimensional arrays and primitives such as `numpy.float64`. This includes
/// single ``(latitude, longitude)`` pairs as well - instead of :class:`tuple`,
/// methods expects a 1-dimensional :class:`numpy.ndarray` of shape ``(2)``.
/// Simply wrap the tuple up with :func:`numpy.array`:
///
///     >>> import numpy as np
///     >>> my_coors = np.array( (-43.362504,   77.926993) )
///     >>>
///     >>> from rust_geodistances import haversine
///     >>> haversine.distance_from_point( my_coors, some_2dim_array )
///
/// All 2-dimensional arrays used by these methods should have:
///
/// - column 0 being latitudes in degrees, and
/// - column 1 being longitudes in degrees.
///
/// To illustrate this::
///
///             [:,0]       [:,1]
///        (Latitude) (Longitude)
///    [0] -43.362504   77.926993
///    [1] -85.044442  123.079125
///    [2]   4.081147   85.444927
///               ...         ...
///    [n]  29.265579  -14.329487
pub enum CalculationMethod {
    /// Haversine Calculation Model
    ///
    /// Assumes the Earth as a perfect sphere.
    /// .. note::
    ///     Algorithm derived from
    ///     `Movable Type Scripts <https://www.movable-type.co.uk/scripts/latlong.html>`_
    HAVERSINE,

    // /// Vincenty Calculation Model
    // ///
    // /// Assumes the Earth as an ellpisoid,
    // /// .. note::
    // ///     Algorithm derived from
    // ///     `Movable Type Scripts <https://www.movable-type.co.uk/scripts/latlong-vincenty.html>`_
    VINCENTY,
}
impl Default for CalculationMethod {
    fn default() -> Self { Self::HAVERSINE }
}

/// Trait for the internal calculation methods.
///
/// This does not in fact needs to be a `trait`, as should only be implemented on
/// a single `struct`; this is simply to provide and option of subtraits intsead of
/// forcing compositions.
pub trait CalculationInterfaceInternal<T> {
    /// Calculate distances from a point.
    ///
    /// This is simply a switch between :meth:`_ser_distance_from_point` and
    /// :meth:`_par_distance_from_point`, depending on the length of `e`.
    ///
    /// This split is required because
    ///
    /// - :meth:`_distance` internally
    ///   uses :meth:`_ser_distance_from_point` on chunks to perform the calculations
    ///   in serial, but
    /// - :meth:`_distance_from_point` should benefit from parallelisation if array
    ///   `e` is too long.
    ///
    /// Hence :meth:`_distance_from_point` cannot be the same function as
    /// :meth:`_ser_distance_from_point`.
    ///
    /// The threshold to choose serial/parallel execution should be read from
    /// :attr:`CalculationSettings.max_serial_1d_array_len`.
    fn _distance_from_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        settings: Option<&CalculationSettings>,
    ) -> F64Array1;

    /// Low level Serial Execution to calculate distances from a point.
    ///
    /// For internal use in `par_iter` in :meth:`_par_distance_from_point` and
    /// :meth:`_distance`.
    fn _ser_distance_from_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        settings: Option<&CalculationSettings>,
    ) -> F64Array1;

    /// Low level Parallel Execution to calculate distances from a point.
    ///
    /// For internal use in :meth:`_distance_from_point` if the length of `e`
    /// exceeds :attr:`CalculationSettings.max_serial_1d_array_len`.
    fn _par_distance_from_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        settings: Option<&CalculationSettings>,
    ) -> F64Array1;

    /// Pairwise distances between two array of points.
    fn _distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        settings: Option<&CalculationSettings>,
    ) -> F64Array2;

    /// Pairwise distances among a single array of points.
    ///
    /// This function will be called by higher level functions if
    /// `s` is found to be *identical* to `e` (i.e. the same instance).
    ///
    /// Since distance calculations are commutative (i.e. f(a,b) == f(b,a))
    /// and the two arrays contain the same elements, we can effectively make half of
    /// the calculations, then mirror the results over. This efficiency gain is most
    /// pronounced in iterative models such as Vincenty.
    fn _distance_within_array(
        &self,
        s:&dyn LatLngArray,
        settings: Option<&CalculationSettings>,
    ) -> F64Array2;

    /// Displace an array by a vector.
    ///
    /// Returns an 2-dimensional array of latitude-longitude pairs.
    ///
    /// .. note::
    ///     In a future version, this method will be detached from
    ///     `CalculationInterfaceInternal` together with
    ///     :meth:`_within_distance_of_point` to become a separate trait,
    ///     as the generic ``<T>`` is not used by any other methods.
    ///
    ///     Hence, use of this method through the trait is not recommended.
    ///     Call this method via the struct that implements this trait instead.
    fn _displace(
        &self,
        s:&dyn LatLngArray,
        distance:T,
        bearing:T,
        settings: Option<&CalculationSettings>,
    ) -> F64LatLngArray;

    /// Checks if an array ``e`` of latitude-longitude pairs are within ``distance`` of ``s``.
    ///
    /// Returns a 1-dimensional array of `bool`.
    fn _within_distance_of_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance:T,
        settings: Option<&CalculationSettings>,
    ) -> BoolArray1;

    /// Pairwise mapping of two arrays to check if each pair are within ``distance`` of each other.
    ///
    /// Returns a 2-dimensional array of `bool`.
    fn _within_distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        distance:f64,
        settings: Option<&CalculationSettings>,
    ) -> BoolArray2;

    /// Pairwise mapping within a single array to check if each pair are within ``distance`` of each other.
    ///
    /// Returns a 2-dimensional array of `bool`.
    ///
    /// Since distance calculations are commutative (i.e. f(a,b) == f(b,a))
    /// and the two arrays contain the same elements, we can effectively make half of
    /// the calculations, then mirror the results over. This efficiency gain is most
    /// pronounced in iterative models such as Vincenty.
    fn _within_distance_among_array(
        &self,
        s:&dyn LatLngArray,
        distance: f64,
        settings: Option<&CalculationSettings>,
    ) -> BoolArray2;

    /// Indices of points in ``e`` within ``distance`` of ``s``.
    ///
    /// Returns a 1-dimensional array of `usizes`. Length is variable depending on the
    /// results.
    fn _indices_within_distance_of_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance: f64,
        settings: Option<&CalculationSettings>,
    ) -> Array1<usize>;

    /// Indices of points in ``e`` within ``distance`` of each point of ``s``.
    ///
    /// Returns a Vector of 1-dimensional arrays of `usizes`.
    ///
    /// .. note::
    ///     Note that the return value is a ``Vec``, not a 2-dimensional array.
    ///     This is because of each ``Array1<usize>>`` being variable in length,
    ///     resulting in a jagged array which is not currently supported.
    fn _indices_within_distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        distance: f64,
        settings: Option<&CalculationSettings>,
    ) -> Vec<Array1<usize>>;

}

#[duplicate_item(
    __vector_type__                 __impl_generics__;
    [ f64 ]                         [];
    [ &F64Array1 ]                  [];
    [ &F64ArcArray1 ]               [];
    [ &F64ArrayView<'a, Ix1> ]      [ 'a ];
    [ &F64ArrayViewMut<'a, Ix1> ]   [ 'a ];
)]
/// *See trait for method descriptions.*
impl<__impl_generics__> CalculationInterfaceInternal<__vector_type__> for CalculationMethod {
    fn _distance_from_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        settings: Option<&CalculationSettings>,
    ) -> F64Array1 {
        let max_serial_1d_array_len: usize = settings.unwrap_or(
            &CalculationSettings::default()
        ).max_serial_1d_array_len;

        // Split case if `e` array is short, then don't parallellise.
        let f = {
            if e.shape()[0] >= max_serial_1d_array_len {
                CalculationInterfaceInternal::<__vector_type__>::_par_distance_from_point
            } else {
                CalculationInterfaceInternal::<__vector_type__>::_ser_distance_from_point
            }
        };

        return f(
            self,
            s, e,
            settings,
        )
    }

    fn _ser_distance_from_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        settings: Option<&CalculationSettings>,
    ) -> F64Array1 {
        let f = match self {
            Self::HAVERSINE => Haversine::distance_from_point,
            Self::VINCENTY => Vincenty::distance_from_point,
        };

        return f(s, e, settings);
    }

    fn _par_distance_from_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        settings: Option<&CalculationSettings>,
    ) -> F64Array1 {
        let shape = e.shape()[0];
        let workers: usize = settings.unwrap_or(
            &CalculationSettings::default()
        ).workers;
        let chunk_size: usize = (shape as f32 / workers as f32).ceil() as usize;

        let mut d = F64Array1::zeros(shape);
        let d_ref = Arc::new(Mutex::new(d));

        (0..shape)
        .into_par_iter()
        .step_by(chunk_size)
        .map(
            | start | (start, cmp::min(start+chunk_size, shape))
        )
        .for_each(
            | (start, end) | {
                let src_slice = CalculationInterfaceInternal::<__vector_type__>::_ser_distance_from_point(
                    self,
                    s, &e.slice_axis(Axis(0), Slice::from(start..end)).to_owned(),
                    settings
                );

                '_mutex_block: {
                    let dest_ref_t = Arc::clone(&d_ref);
                    let mut d = dest_ref_t.lock().unwrap();

                    let mut to_slice = d.slice_mut(s![start..end]);
                    to_slice.assign(&src_slice.view());
                };
            }
        );

        d = Arc::try_unwrap(d_ref)
                .unwrap()
                .into_inner()
                .unwrap();

        return d;
    }

    fn _distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        settings: Option<&CalculationSettings>,
    ) -> F64Array2 {
        let f = match self {
            Self::HAVERSINE => Haversine::distance,
            Self::VINCENTY => Vincenty::distance,
        };

        if s.shape()[0] > e.shape()[0] {
            return f(s, e, settings);
        } else {
            let mut result = f(e, s, settings);
            result.swap_axes(0, 1);

            return result;
        }
    }

    fn _distance_within_array(
        &self,
        s:&dyn LatLngArray,
        settings: Option<&CalculationSettings>,
    ) -> F64Array2 {
        let f = match self {
            Self::HAVERSINE => Haversine::distance_from_point,
            Self::VINCENTY => Vincenty::distance_from_point,
        };

        let s_owned = s.to_owned();

        let workers: usize = settings.unwrap_or(
            &CalculationSettings::default()
        ).workers;

        return F64Array2::from_mapped_array2_fn(
            &s_owned.view(),
            | s, e | {
                // println!("sn={:?} en={:?}", &s, &e);
                f(&s, &e.to_owned(), settings)
            },
            workers,
            Some(true),
        );
    }

    fn _displace(
        &self,
        s:&dyn LatLngArray,
        distance:__vector_type__,
        bearing:__vector_type__,
        settings: Option<&CalculationSettings>,
    ) -> F64LatLngArray {
        let f = match self {
            Self::HAVERSINE => Haversine::displace,
            Self::VINCENTY => Vincenty::displace,
        };

        return f(s, distance, bearing, settings);
    }

    fn _within_distance_of_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance: __vector_type__,
        settings: Option<&CalculationSettings>,
    ) -> BoolArray1 {
        let distances = CalculationInterfaceInternal
                                        ::<__vector_type__>
                                        ::_distance_from_point(
                                            self,
                                            s, e,
                                            settings,
                                        );

        return (distances - distance).le(&0.);
    }

    fn _within_distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        distance: f64, // Restrict to f64 here
        settings: Option<&CalculationSettings>,
    ) -> BoolArray2 {
        let distances = CalculationInterfaceInternal
                                        ::<__vector_type__>
                                        ::_distance(
                                            self,
                                            s, e,
                                            settings,
                                        );

        return (distances - distance).le(&0.);
    }

    fn _within_distance_among_array(
        &self,
        s:&dyn LatLngArray,
        distance: f64,
        settings: Option<&CalculationSettings>,
    ) -> BoolArray2 {
        let distances = CalculationInterfaceInternal
                                        ::<__vector_type__>
                                        ::_distance_within_array(
                                            self,
                                            s,
                                            settings,
                                        );

        return (distances - distance).le(&0.);
    }

    fn _indices_within_distance(
        &self,
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        distance: f64,
        settings: Option<&CalculationSettings>,
    ) -> Vec<Array1<usize>> {
        return CalculationInterfaceInternal
               ::<__vector_type__>
               ::_within_distance(
                    self,
                    s, e,
                    distance,
                    settings,
                )
                .axis_iter(Axis(0))
                .map(
                    | row| {
                        row.indices()
                    }
                )
                .collect();
    }

    fn _indices_within_distance_of_point(
        &self,
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance: f64,
        settings: Option<&CalculationSettings>,
    ) -> Array1<usize> {
        return self._within_distance_of_point(
            s, e,
            distance,
            settings,
        )
        .indices();
    }

}
