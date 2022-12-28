// Traits for calculation methods.
//
// These traits describes what a Calculation Method (e.g. Haversine)
// can do.

use std::ops::Index;

use duplicate::duplicate_item;

use ndarray::{
    Dim,
    Ix,
    Ix1,
    // Ix2,
    // NdIndex,
};

use ndarray_numeric::{
    ArrayWithF64AngularMethods,
    ArrayWithF64LatLngMethods,

    F64Array1,
    F64Array2,
    F64ArrayView,
    F64LatLng,
    F64LatLngView,
    F64LatLngViewMut,
    F64LatLngArray,
    F64LatLngArcArray,
    // F64LatLngArrayView,
    F64LatLngArrayViewMut,
};

use super::config;

// Marker Trait definitions for the common parameter types
pub trait LatLng : ArrayWithF64AngularMethods<Ix1> + Index<Ix, Output = f64> + Sync {}
pub trait LatLngArray : ArrayWithF64LatLngMethods + Index<Dim<[Ix; 2]>, Output = f64> + Sync {}

#[duplicate_item(
    __latlng_type__;
    [ F64LatLng ];
    [ F64LatLngView<'_> ];
    [ F64LatLngViewMut<'_> ];
)]
/// Mark relevant types with Marker Trait.
impl LatLng for __latlng_type__ {}

#[duplicate_item(
    __latlngarray_type__;
    [ F64LatLngArray ];
    [ F64LatLngArcArray ];
    // [ F64LatLngArrayView<'_> ];
    [ F64LatLngArrayViewMut<'_> ];
)]
/// Mark relevant types with Marker Trait.
/// Note that F64LatLngArrayView is excluded as some methods do not support it.
impl LatLngArray for __latlngarray_type__ {}

/// Trait for structs that are able to calculate geodistances.
///
/// Structs must implement methods for both radian and great-circle distances;
/// this is to allow unrepeated conversions from degree-radian conversions before
/// threading.
pub trait CalculateDistance {
    /// Radian distances from one pair of radian coordinates to an array of them.
    ///
    /// .. note::
    ///     Internal Function; exposed within Rust, but not intended for use with
    ///     Python interface directly.
    ///
    /// Parameters
    /// ----------
    /// s_lat_r: &f64
    ///     Reference to a `f64` Latitude value.
    ///
    /// s_lng_r: &f64
    ///     Reference to a `f64` Latitude value.
    ///
    /// e_lat_r: &ArrayView<'_, f64, Ix1>
    ///     Reference to an `ArrayView` of Latitude values.
    ///     For an owned array, use :func:`.slice`
    ///
    /// e_lng_r: &ArrayView<'_, f64, Ix1>
    ///     Reference to an `ArrayView` of Latitude values.
    ///
    /// settings: Option<&config::CalculationSettings>
    ///     Settings to be passed on to the calculation method.
    ///
    /// Returns
    /// -------
    /// Array1<f64>
    ///     An array of radian distances calculated from `s` to each point in `e`.
    fn distance_from_point_rad(
        s_lat_r:&f64,
        s_lng_r:&f64,
        e_lat_r:&F64ArrayView<'_, Ix1>,
        e_lng_r:&F64ArrayView<'_, Ix1>,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array1;

    /// Radian distances between two arrays of radian coordinates.
    ///
    /// .. note::
    ///     Internal Function; exposed within Rust, but not intended for use with
    ///     Python interface directly.
    ///
    /// Parameters
    /// ----------
    /// s_lat_r: &ArrayView<'_, f64, Ix1>
    ///     Reference to an `ArrayView` of Latitude values of `s`.
    ///     For an owned array, use :func:`.slice`
    ///
    /// s_lng_r: &ArrayView<'_, f64, Ix1>
    ///     Reference to an `ArrayView` of Latitude values of `s`.
    ///
    /// e_lat_r: &ArrayView<'_, f64, Ix1>
    ///     Reference to an `ArrayView` of Latitude values of `e`.
    ///     For an owned array, use :func:`.slice`
    ///
    /// e_lng_r: &ArrayView<'_, f64, Ix1>
    ///     Reference to an `ArrayView` of Latitude values of `e`.
    ///
    /// settings: Option<&config::CalculationSettings>
    ///     Settings to be passed on to the calculation method.
    ///
    /// Returns
    /// -------
    /// Array2<f64>
    ///     An array of radian distances mapping each point in  `s` to each point in `e`.
    fn distance_rad(
        s_lat_r:&F64ArrayView<'_, Ix1>,
        s_lng_r:&F64ArrayView<'_, Ix1>,
        e_lat_r:&F64ArrayView<'_, Ix1>,
        e_lng_r:&F64ArrayView<'_, Ix1>,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array2;

    /// Great-circle distances from one pair of lat-long coordinates to an array of them.
    ///
    /// This method does not in itself carries a unit; it simply multiply the calculated
    /// radian distances by the spherical/elliptical radiuses in `settings`. The
    /// returned values will always be the same unit as those used in `settings`.
    ///
    /// Parameters
    /// ----------
    /// s_lat_r: &f64
    ///     Reference to a `f64` Latitude value.
    ///
    /// s_lng_r: &f64
    ///     Reference to a `f64` Latitude value.
    ///
    /// e_lat_r: &ArrayView<'_, f64, Ix1>
    ///     Reference to an `ArrayView` of Latitude values.
    ///     For an owned array, use :func:`.slice`
    ///
    /// e_lng_r: &ArrayView<'_, f64, Ix1>
    ///     Reference to an `ArrayView` of Latitude values.
    ///
    /// settings: Option<&config::CalculationSettings>
    ///     Settings to be passed on to the calculation method.
    ///
    /// Returns
    /// -------
    /// Array1<f64>
    ///     An array of great-circle distances calculated from `s` to each point in `e`.
    fn distance_from_point(
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array1;

    /// Great-circle distances between two arrays of lat-long coordinates.
    ///
    /// Parameters
    /// ----------
    /// s_lat_r: &ArrayView<'_, f64, Ix1>
    ///     Reference to an `ArrayView` of Latitude values of `s`.
    ///     For an owned array, use :func:`.slice`
    ///
    /// s_lng_r: &ArrayView<'_, f64, Ix1>
    ///     Reference to an `ArrayView` of Latitude values of `s`.
    ///
    /// e_lat_r: &ArrayView<'_, f64, Ix1>
    ///     Reference to an `ArrayView` of Latitude values of `e`.
    ///     For an owned array, use :func:`.slice`
    ///
    /// e_lng_r: &ArrayView<'_, f64, Ix1>
    ///     Reference to an `ArrayView` of Latitude values of `e`.
    ///
    /// settings: Option<&config::CalculationSettings>
    ///     Settings to be passed on to the calculation method.
    ///
    /// Returns
    /// -------
    /// Array2<f64>
    ///     An array of great-circle distances mapping each point in  `s` to each point in `e`.
    fn distance(
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array2;
}

/// Generic T here, could be scalar f64 or F64Array.
pub trait OffsetByVector<T>:CalculateDistance {
    /// Get the resultant coordinates after being displaced by the given vector(s).
    ///
    /// Parameters
    /// ----------
    /// s: Array2<f64>| ArcArray2<f64> | ArrayView2<'a, f64>
    ///     Dimension `(n, 2)`. Array of coordinates to be displaced:
    ///     - `array.column(0)` being latitude in degrees,
    ///     - `array.column(1)` being longitude in degrees.
    ///
    /// distance: f64 | Array1<f64>
    ///     If `f64`, all coordinates in `s` will be displaced by the
    ///     same distance.
    ///
    ///     If `Array1<f64>`, then Dimension must be `(n)` where `n` is
    ///     the number of rows in `s`. Each pair of coordinates in `s`
    ///     will then be displaced by the distance matching
    ///     element-wise by row.
    ///
    ///     The unit of this value must be
    ///
    /// bearing: f64 | Array1<f64>
    ///     `f64` between 0.-360º.
    ///
    ///     If `f64`, all coordinates in `s` will be displaced towards
    ///     the same bearing.
    ///
    ///     If `Array1<f64>`, then Dimension must be `(n)` where `n` is
    ///     the number of rows in `s`. Each pair of coordinates in `s`
    ///     will then be displaced by the bearing matching
    ///     element-wise by row.
    ///
    /// settings: Option<&config::CalculationSettings>
    ///     Settings to be passed on to the calculation method.
    ///
    /// Returns
    /// -------
    /// Array2<f64>
    ///     Dimension `(n, 2)`, with `n` matching the number of rows in
    ///     `s`.
    ///     - `array.column(0)` being latitude in degrees,
    ///     - `array.column(1)` being longitude in degrees.
    fn displace(
        s:&dyn LatLngArray,
        distance:T,
        bearing:T,
        settings: Option<&config::CalculationSettings>,
    ) -> F64LatLngArray;
}
