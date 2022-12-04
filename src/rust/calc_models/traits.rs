use ndarray::{
    Array1
};

use ndarray_numeric::{
    ArrayWithF64LatLngMethods,
    F64Array1,
    F64LatLng,
    F64LatLngArray,
};

pub trait CalculateDistance {
    fn distance(
        s:&F64LatLng,
        e:&dyn ArrayWithF64LatLngMethods,
    ) -> F64Array1;
}

pub trait OffsetByVector:CalculateDistance {
    fn offset(
        s:&dyn ArrayWithF64LatLngMethods,
        distance:f64,
        bearing:f64,
    ) -> F64LatLngArray;
}

//  CheckDistance REQUIRES OffsetByVector
pub trait CheckDistance:OffsetByVector {
    fn within_distance(
        s:&F64LatLng,
        e:&dyn ArrayWithF64LatLngMethods,
        distance:f64,
    ) -> Array1<bool>;
}
