/// ==============================
///  Structs for data structures.
/// ==============================
///

use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeTuple;
// use serde_json::{Result, Value};
// use serde_with::serde_as;
use serde_pickle;

use crate::input_output::pickle;

// #[serde_as]
#[derive(Debug, Deserialize)]
pub struct LatLng{
    lat: f64,
    lng: f64,
}
impl Serialize for LatLng {
    /// If we don't specify, this thing is going to serialize into a dict.
    /// Numpy is not going to be happy - lets make it a tuple.
    fn serialize<S>(
        &self,
        serializer: S
    ) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(&self.lat)?;
        tup.serialize_element(&self.lng)?;
        tup.end()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoordinateList(pub Vec<LatLng>);
impl CoordinateList {

    /// This is to get the inside value of the Tuple struct.
    pub fn value(&self) -> &[LatLng] {
        let Self(value) = self;
        return &value
    }

    pub fn len(&self) -> usize {
        return self.value().len()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IOCoordinateLists(pub [CoordinateList; 2]);
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
impl IOCoordinateLists {

    /// This is to get the inside value of the Tuple struct.
    pub fn arrays(&self) -> [&CoordinateList; 2] {
        let Self([array1, array2]) = self;
        return [&array1, &array2]
    }
    pub fn shape(&self) -> (usize, usize) {
        let [array1, array2] = self.arrays();
        return (array1.len(), array2.len())
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
    /// This Enum needs to know how to serialize itself.
    /// Its not difficult - it just needs to use the internal value instead.
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

/// A result array of variable size to
#[derive(Debug, Serialize, Deserialize)]
pub struct IOResultArray(Vec<Vec<CalculationResult>>);
impl IOResultArray {
    /// Creates a new, empty IOResultArray to check if
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
