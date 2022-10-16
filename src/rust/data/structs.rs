/// ==============================
///  Structs for data structures.
/// ==============================
///

use serde::{Deserialize, Serialize, Serializer};
// use serde_json::{Result, Value};
// use serde_with::serde_as;
use serde_pickle;

use crate::input_output::pickle;

// #[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct LatLng(f64, f64);

#[derive(Debug, Serialize, Deserialize)]
pub struct CoordinateList(Vec<LatLng>);

#[derive(Debug, Serialize, Deserialize)]
pub struct IOCoordinateLists([CoordinateList; 2]);
impl pickle::traits::PickleImport<Self> for IOCoordinateLists {

    /// Create a IOCoordinateLists object from a Python compatible pickle array of ubytes.
    fn from_pickle(data:&[u8]) -> Self {
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
}
impl pickle::traits::PickleExport for IOCoordinateLists {
    /// Create a Python compatible pickle array of ubytes from a IOCoordinateLists object.
    ///
    /// This is not actually used currently, because the output of this program is in fact the
    /// distances, not distances.
    #[allow(dead_code)]
    fn to_pickle(&self) -> Vec<u8> {
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


#[derive(Debug, Deserialize)]
pub enum CalculationResult {
    Geodistance(Option<f64>),
    WithinDistance(bool),
    Unpopulated,
}
impl Serialize for CalculationResult {
    fn serialize<S>(
        &self,
        serializer: S
    ) -> Result<S::Ok, S::Error>
    where S: Serializer {
        return match self {
            Self::Geodistance(Some(distance_option)) => {
                serializer.serialize_f64(*distance_option)
            },
            Self::Geodistance(None) => {
                serializer.serialize_none()
            },
            Self::WithinDistance(value) => {
                serializer.serialize_bool(*value)
            },
            Self::Unpopulated => {
                serializer.serialize_none()
            }
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct IOResultArray(Vec<Vec<CalculationResult>>);
impl IOResultArray {
    pub fn new(shape:(usize, usize)) -> Self {
        fn make_row(size:usize) -> Vec<CalculationResult> {
            (0..size).map(|_| CalculationResult::Unpopulated).collect()
        }

        let _vec = (0..shape.0).map(|_| make_row(shape.1)).collect();

        Self(_vec)
    }
}
impl pickle::traits::PickleExport for IOResultArray {
    /// Create a Python compatible pickle array of ubytes from a IOCoordinateLists object.
    ///
    /// This is not actually used currently, because the output of this program is in fact the
    /// distances, not distances.
    #[allow(dead_code)]
    fn to_pickle(&self) -> Vec<u8> {
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
