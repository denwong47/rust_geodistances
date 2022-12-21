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
    BoolArray1,
    BoolArray2,
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

// Marker types that include
pub trait LatLng : ArrayWithF64AngularMethods<Ix1> + Index<Ix, Output = f64> {}
pub trait LatLngArray : ArrayWithF64LatLngMethods + Index<Dim<[Ix; 2]>, Output = f64> {}

#[duplicate_item(
    __latlng_type__;
    [ F64LatLng ];
    [ F64LatLngView<'_> ];
    [ F64LatLngViewMut<'_> ];
)]
impl LatLng for __latlng_type__ {}

#[duplicate_item(
    __latlngarray_type__;
    [ F64LatLngArray ];
    [ F64LatLngArcArray ];
    // [ F64LatLngArrayView<'_> ];
    [ F64LatLngArrayViewMut<'_> ];
)]
impl LatLngArray for __latlngarray_type__ {}

pub trait CalculateDistance {
    fn distance_from_point_rad(
        s_lat_r:&f64,
        s_lng_r:&f64,
        e_lat_r:&F64ArrayView<'_, Ix1>,
        e_lng_r:&F64ArrayView<'_, Ix1>,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array1;

    fn distance_rad(
        s_lat_r:&F64ArrayView<'_, Ix1>,
        s_lng_r:&F64ArrayView<'_, Ix1>,
        e_lat_r:&F64ArrayView<'_, Ix1>,
        e_lng_r:&F64ArrayView<'_, Ix1>,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array2;

    fn distance_from_point(
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        settings: Option<&config::CalculationSettings>,
    ) -> F64Array1;

    fn distance(
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        shape:(usize, usize),
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
    /// bearing: f64 | Array1<f64>
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
    fn offset(
        s:&dyn LatLngArray,
        distance:T,
        bearing:T,
        settings: Option<&config::CalculationSettings>,
    ) -> F64LatLngArray;
}

/// Generic T here, could be scalar f64 or F64Array.
///
/// .. note::
///     CheckDistance REQUIRES OffsetByVector.
///
/// While this trait allows for T as 1-dimensional arrays, which
/// ACTUALLY works, but the length of the array needs
/// to match that of `e`, not `s` intuitively.
///
/// There is already the trait of `ArrayWithF64MappedOperators`
/// to resolve this; but this will require a second `impl` for
/// `T:F64Array1`, which is not a priority for now.
pub trait CheckDistance<T>:OffsetByVector<T> {
    fn within_distance_of_point(
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance:T,
        settings: Option<&config::CalculationSettings>,
    ) -> BoolArray1;

    fn within_distance(
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,
        distance: T,
        shape:(usize, usize),
        settings: Option<&config::CalculationSettings>,
    ) -> BoolArray2;
}
