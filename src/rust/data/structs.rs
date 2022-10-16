/// ==============================
///  Structs for data structures.
/// ==============================
///

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeTuple;
use serde::de::Error;
// use serde_json::{Result, Value};
// use serde_with::serde_as;
use serde_pickle;

use crate::input_output::pickle;

/// A point of latitude and longitude.
#[derive(Debug, Deserialize, Copy, Clone)]
pub struct LatLng{
    pub lat: f64,
    pub lng: f64,
}
impl LatLng {
    pub fn new(lat:f64, lng:f64) -> Self {
        return Self {
            lat: lat,
            lng: lng,
        }
    }
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

/// An array of LatLng coordinates.
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

/// Input Coordinate Lists.
///
/// Input can be one or two arrays.
/// One array implies that we are measuring distances among the same points,
/// and there are further optimisation that can be made.
///
/// If none, or 3 or more arrays are provided, Deserialization will fail.
#[derive(Debug)]
pub struct IOCoordinateLists(pub [Option<CoordinateList>; 2]);
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
                "Rust Backend: Incoming data is not well formed. Expected [ [ (f64, f64), ... ], [ (f64, f64), ... ] ]."
            );
        }
    }
}
impl IOCoordinateLists {

    /// This is to get the inside value of the Tuple struct.
    /// It will evaluate whether the second array exists; if not, it will return another
    /// reference to the first array.
    pub fn arrays(&self) -> [&CoordinateList; 2] {
        let Self([array1, array2]) = self;

        return match array2 {
            Some(list) => [&array1.as_ref().unwrap(), &list],
            None => [&array1.as_ref().unwrap(), &array1.as_ref().unwrap()],
        }

    }

    /// Returns a tuple of usizes, stating how many elements are in each array.
    /// Useful for IOResultArray::new.
    #[allow(dead_code)]
    pub fn shape(&self) -> (usize, usize) {
        let [array1, array2] = self.arrays();
        return (array1.len(), array2.len())
    }

    /// Input can be one or two arrays.
    /// One array implies that we are measuring distances among the same points,
    /// and there are further optimisation that can be made.
    /// This function detects if the second array exists, and return the absolute
    /// array count.
    #[allow(dead_code)]
    pub fn unique_array_count(&self) -> usize {
        let Self([_, array2]) = self;

        return match array2 {
            Some(_) => 2 as usize,
            None => 1 as usize,
        }
    }
}
impl Serialize for IOCoordinateLists {
    /// This Enum needs to know how to serialize itself.
    /// Its not difficult - it just needs to use the internal value instead.
    #[allow(dead_code)]
    fn serialize<S>(
        &self,
        serializer: S
    ) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut tup = serializer.serialize_tuple(2)?;

        let [array1, array2] = self.arrays();

        tup.serialize_element(array1)?;
        tup.serialize_element(array2)?;
        tup.end()
    }
}
impl<'de> Deserialize<'de> for IOCoordinateLists {

    /// We need to specify how to deserialize here, because there could be 1 or 2 arrays
    /// provided. We put everything in a Vec first, figure out which case it is, then
    /// deserialize accordingly.
    #[allow(dead_code)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
    {
        let mut lists = Vec::<CoordinateList>::deserialize(deserializer)?;

        return match lists.len() {
            2 => Ok(Self([Some(lists.swap_remove(0)), Some(lists.swap_remove(0))])),
            1 => Ok(Self([Some(lists.swap_remove(0)), None])),
            len => Err(D::Error::invalid_length(len, &"1 or 2"))
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

/// A result of calculation.
///
/// Depending on which mode is being called, this could be:
/// - Geodistance - an optional f64; if not present, it means it is impossible to be
///   within maximum distance.
/// - WithinDistance - bool; determines if something is within distance. Always
///   populated.
/// - Unpopulated - placeholder value until an array value is filled. Serialise to Null.
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

    /// Creates a new, empty IOResultArray.
    /// All initial values will be set to Unpopulated;
    /// if serialized, these will be come Nulls.
    pub fn new(shape:(usize, usize)) -> Self {
        fn make_row(size:usize) -> Vec<CalculationResult> {
            (0..size).map(|_| CalculationResult::Unpopulated).collect()
        }

        let _vec = (0..shape.0).map(|_| make_row(shape.1)).collect();

        Self(_vec)
    }

    /// A wrapper function for ::new(), to create a IOResultArray that suits the size
    /// of the input arrays.
    pub fn like_input(input: &IOCoordinateLists) -> Self {
        return Self::new(input.shape());
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
