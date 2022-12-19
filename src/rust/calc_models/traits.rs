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
    fn offset_from_point(
        s:&dyn LatLngArray,
        distance:T,
        bearing:T,
        settings: Option<&config::CalculationSettings>,
    ) -> F64LatLngArray;
}

//  CheckDistance REQUIRES OffsetByVector
/// Generic T here, could be scalar f64 or F64Array.
pub trait CheckDistance<T>:OffsetByVector<T> {
    fn within_distance_from_point(
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance:T,
        settings: Option<&config::CalculationSettings>,
    ) -> BoolArray1;

    fn within_distance(
        s:&dyn LatLngArray,
        e:&dyn LatLngArray,

        // Only supports f64 for 2-dimensions:
        // While this parameter allows for 1-dimensional arrays, which
        // it ACTUALLY works, but the length of the array needs
        // to match that of `e`, not `s` intuitively.
        // We can probably change this to make it work with some `rows_mut`
        // but currently this is in the backlog.
        distance: T,
        shape:(usize, usize),
        settings: Option<&config::CalculationSettings>,
    ) -> BoolArray2;
}
