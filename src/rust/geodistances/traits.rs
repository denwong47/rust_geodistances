use crate::data::structs::LatLng;

pub trait CalculateDistance {
    fn distance(
        s:&LatLng,
        e:&LatLng,
    )->Option<f64>;
}

pub trait OffsetByVector:CalculateDistance {
    fn offset(
        s:&LatLng,
        distance:f64,
        bearing:f64,
    )->Option<LatLng>;
}

//  CheckDistance REQUIRES OffsetByVector
pub trait CheckDistance:OffsetByVector {
    fn within_distance(
        s:&LatLng,
        e:&LatLng,
        distance:f64,
    )->bool;
}
