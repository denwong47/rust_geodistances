use crate::data::structs::LatLng;

use crate::geodistances::traits::{CalculateDistance, CheckDistance};

const ELLIPSE_WGS84_A:f64 = 6378.137;
const ELLIPSE_WGS84_B:f64 = 6356.752314245;
const ELLIPSE_WGS84_F:f64 = 1./298.257223563;

const ITERATIONS:u16 = 1000;

pub struct Vincenty;
impl CalculateDistance for Vincenty {
    fn distance(
        s:&LatLng,
        e:&LatLng,
    )->Option<f64> {
        let eps:f64 = (2 as f64).powi(-52);

        return None
    }
}
impl CheckDistance for Vincenty {
    fn within_distance(
        s:&LatLng,
        e:&LatLng,
        distance:f64,
    )->bool {
        return match Self::distance(s, e){
            Some(measured) => measured <= distance,
            None => false,
        }
    }
}
