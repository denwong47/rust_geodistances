use std::ops::Index;

use duplicate::duplicate_item;

use ndarray::{
    Array1,
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
    F64LatLng,
    F64LatLngView,
    F64LatLngViewMut,
    F64LatLngArray,
    F64LatLngArcArray,
    // F64LatLngArrayView,
    F64LatLngArrayViewMut,
};

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
    fn distance(
        s:&dyn LatLng,
        e:&dyn LatLngArray,
    ) -> F64Array1;
}

pub trait OffsetByVector:CalculateDistance {
    fn offset(
        s:&dyn LatLngArray,
        distance:f64,
        bearing:f64,
    ) -> F64LatLngArray;
}

//  CheckDistance REQUIRES OffsetByVector
pub trait CheckDistance:OffsetByVector {
    fn within_distance(
        s:&dyn LatLng,
        e:&dyn LatLngArray,
        distance:f64,
    ) -> Array1<bool>;
}
