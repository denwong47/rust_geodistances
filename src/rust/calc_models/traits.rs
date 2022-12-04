use ndarray_numeric::{
    ArrayWithF64AngularMethods,
    ArrayWithF64LatLngMethods,
    F64Array1,
    F64LatLng,
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
    ) -> F64LatLng;
}

//  CheckDistance REQUIRES OffsetByVector
pub trait CheckDistance:OffsetByVector {
    fn within_distance(
        s:&F64LatLng,
        e:&dyn ArrayWithF64LatLngMethods,
        distance:f64,
    ) -> bool;
}
