use crate::data::structs::LatLng;

pub trait CalculateDistance {
    fn distance(
        s:&LatLng,
        e:&LatLng,
    )->Option<f64>;
}

pub trait CheckDistance:CalculateDistance {
    fn within_distance(
        s:&LatLng,
        e:&LatLng,
        distance:f64,
    )->bool;
}
