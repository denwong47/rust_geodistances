/// ==============================
///  Structs for data structures.
/// ==============================
///

use serde::{Deserialize, Serialize};
// use serde_json::{Result, Value};
// use serde_with::serde_as;
use serde_pickle;

// #[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct LatLng(f64, f64);

#[derive(Debug, Serialize, Deserialize)]
pub struct CoordinateList(Vec<LatLng>);

#[derive(Debug, Serialize, Deserialize)]
pub struct IOCoordinateLists([CoordinateList; 2]);
impl IOCoordinateLists {

    /// Create a IOCoordinateLists object from a Python compatible pickle array of ubytes.
    pub fn from_pickle(data:&[u8]) -> Self {
        if let Ok(coordinate_lists) = serde_pickle::de::from_slice(
            data,
            serde_pickle::de::DeOptions::new()
        ) {
            return coordinate_lists;
        } else {
            panic!(
                "Rust Backend: Incoming data is not well formed. {}",
                "Expected [ List[ Tuple[ f64, f64 ] ], List[ Tuple[ f64, f64 ] ] ]."
            );
        }
    }

    /// Create a Python compatible pickle array of ubytes from a IOCoordinateLists object.
    ///
    /// This is not actually used currently, because the output of this program is in fact the
    /// distances, not distances.
    #[allow(dead_code)]
    pub fn to_pickle(&self) -> Vec<u8> {
        if let Ok(data) = serde_pickle::ser::to_vec(
            self,
            serde_pickle::ser::SerOptions::new()
        ) {
            return data;
        } else {
            panic!(
                "Rust Backend: Data created is not compatible for Python pickling:\n{:?}",
                self
            );
        }
    }
}
