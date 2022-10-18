use std::f64::consts::PI;

use crate::data::structs::LatLng;
use crate::geodistances::traits::{CalculateDistance, CheckDistance};

const CARTESIAN_DISTANCE_COEFFICIENT:f64 = 111.22983322959863;  // Assume 6373km radius

/// Cartesian calculation.
///
/// Treats as though the world is a cylinder, and return the distance between
/// two points as though they are on the equator.
///
/// Obviously this is not accurate in any sense, but useful to determine maximum
/// distance between two points using lower calculation costs.
pub struct Cartesian;
impl CalculateDistance for Cartesian {
    fn distance(
        s:&LatLng,
        e:&LatLng,
    )->Option<f64> {

        return Some(
            (
                (e.lat - s.lat).abs().powi(2)
                + (e.lng - s.lng).abs().powi(2)
            ).sqrt()
            * CARTESIAN_DISTANCE_COEFFICIENT
        )
    }
}
impl CheckDistance for Cartesian {
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
