use std::f64::consts::PI;

use crate::data::structs::LatLng;
use crate::geodistances::traits::{CalculateDistance, CheckDistance, OffsetByVector};
use crate::geodistances::config::RADIUS;

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
impl OffsetByVector for Cartesian {
    fn offset(
        s:&LatLng,
        distance:f64,
        bearing:f64,
    )->Option<LatLng> {
        let degree_per_km = 360./(2.*PI*RADIUS);

        let bearing_r = bearing / 180. * PI;

        let dx = degree_per_km * distance * bearing_r.sin();
        let dy = degree_per_km * distance * bearing_r.cos();

        return Some(LatLng::new(
            s.lat + dy,
            s.lng + dx
        ))
    }
}
