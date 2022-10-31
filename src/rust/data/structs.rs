/// ==============================
///  Structs for data structures.
/// ==============================
///
///
use std::cmp;
use std::f64::consts::PI;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeTuple;
use serde::de::Error;
// use serde_json::{Result, Value};
// use serde_with::serde_as;
use serde_pickle;

use crate::data::traits::{Slicable};

use crate::input_output::pickle;

// #[derive(Debug, Deserialize, Copy, Clone)]
// pub struct Vector{
//     pub distance: f64,
//     pub bearing: f64,
// }

/// A point of latitude and longitude.
#[derive(Debug, Deserialize, Copy, Clone)]
pub struct LatLng{
    pub lat: f64,
    pub lng: f64,
}
impl LatLng {
    #[allow(dead_code)]
    pub fn new(lat:f64, lng:f64) -> Self {
        return Self {
            lat: (lat+270.) % 180. -90.,
            lng: (lng+540.) % 360. -180.,
        }
    }

    #[allow(dead_code)]
    pub fn new_from_rad(lat_r:f64, lng_r:f64) -> Self {
        return Self::new(
            lat_r / PI * 180.,
            lng_r / PI * 180.
        )
    }

    #[allow(dead_code)]
    pub fn as_rad(&self) -> (f64, f64) {
        return (
            self.lat / 180. * PI,
            self.lng / 180. * PI
        )
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Clone)]
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
                "Rust Backend: Incoming data is not well formed. Expected [ [ (f64, f64), ... ], [ (f64, f64), ... ] ], found {:?}",
                data
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
impl Slicable for IOCoordinateLists {

    /// Returns a tuple of usizes, stating how many elements are in each array.
    /// Useful for IOResultArray::new.
    #[allow(dead_code)]
    fn shape(&self) -> (usize, usize) {
        let [array1, array2] = self.arrays();
        return (array1.len(), array2.len())
    }

    /// Get a shallow copy slice of itself.
    fn slice(
        &self,
        origin: (usize, usize),
        size: (usize, usize),
    ) -> Self {

        let [array1, array2] = self.arrays();

        return Self([
            Some(CoordinateList(array1.value()[
                origin.0..cmp::min(
                    self.shape().0, origin.0+size.0
                )].to_vec())
            ),
            Some(CoordinateList(array2.value()[
                origin.1..cmp::min(
                    self.shape().1, origin.1+size.1
                )].to_vec())
            )
        ])
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn chunks(
        &self,
        count: usize,
    ) -> ((usize, usize), (usize, usize)) {
        // Not yet functioning
        return (
            (0, 0),
            self.shape(),
        )
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
#[derive(Debug, Deserialize, Copy, Clone)]
pub enum CalculationResult {
    Geodistance(Option<f64>),
    WithinDistance(bool),
    Location(Option<LatLng>),
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
            Self::Location(Some(latlng)) => {
                latlng.serialize(serializer)
            },
            Self::Location(None) => {
                serializer.serialize_none()
            }
            Self::Unpopulated => {
                serializer.serialize_none()
            }
        }
    }
}

/// A result array of variable size to
#[derive(Debug, Deserialize, Clone)]
pub struct IOResultArray{
    pub array: Vec<Vec<CalculationResult>>
}
impl IOResultArray {

    /// Creates a new, empty IOResultArray.
    /// All initial values will be set to Unpopulated;
    /// if serialized, these will be come Nulls.
    #[allow(dead_code)]
    pub fn new(shape:(usize, usize)) -> Self {
        return Self::full(shape, CalculationResult::Unpopulated);
    }

    pub fn full(shape:(usize, usize), value: CalculationResult) -> Self {
        fn make_row_closure(value: CalculationResult, size:usize) ->
        Box<dyn Fn(usize) -> Vec<CalculationResult>> {
            Box::new(move |_| (0..size).map(|_| value.clone()).collect())
        }

        let _vec = (0..shape.0).map(make_row_closure(value, shape.1)).collect();

        return Self{
            // RefCell?
            array:_vec
        }
    }

    /// A wrapper function for ::new(), to create a IOResultArray that suits the size
    /// of the input arrays.
    #[allow(dead_code)]
    pub fn like_input(input: &IOCoordinateLists) -> Self {
        return Self::new(input.shape());
    }

    #[allow(dead_code)]
    pub fn like_input_full(input: &IOCoordinateLists, value: CalculationResult) -> Self {
        return Self::full(input.shape(), value);
    }

    /// Replace the vector contents of [x..x+w, y..y+h] with the contents of `replace_with`.
    #[allow(dead_code)]
    pub fn splice(
        &mut self,
        origin:(usize, usize),
        mut replace_with:IOResultArray
    ) {
        let (x, y) = origin;
        let (a, b) = self.shape();
        let (w, h) = replace_with.shape();

        let upper_x = cmp::min(x+w, a);
        let upper_y = cmp::min(y+h, b);

        let array:&mut [Vec<CalculationResult>] = self.array.as_mut_slice();

        for row in (x..upper_x).rev() {
            let last_element = replace_with.array.pop();

            if let Some(slice) = last_element {
                array[row][y..upper_y].copy_from_slice(&slice[0..(upper_y-y)])
                // drop(array[row].splice(y..upper_y, slice[0..(upper_y-y)].clone()))
            }
        }
    }

    /// Take the bottom left half of the array and mirror it to the upper right
    #[allow(dead_code)]
    pub fn mirror_fill(
        &mut self,
        diagonal_value: CalculationResult,
    ) {
        let (a, b) = self.shape();
        let max_dim = cmp::min(a, b);

        for row in 0..max_dim {
            for col in row..max_dim {
                // If its on the diagonal, put the default value in
                // Otherwise, clone from the mirrored half.
                self.array[row][col] = if col == row {
                    diagonal_value
                } else {
                    self.array[col][row].clone()
                }
            }
        }
    }
}
impl Slicable for IOResultArray {

    /// Return a tuple of (usize, usize) stating the shape of the underlying data.
    /// Assumes all secondary Vecs are the same size.
    #[allow(dead_code)]
    fn shape(&self) -> (usize, usize) {
        let x:usize = self.array.len();
        let y:usize = self.array[0].len();

        return (x, y);
    }

    /// Get a shallow copy slice of itself.
    fn slice(
        &self,
        origin: (usize, usize),
        size: (usize, usize),
    ) -> Self {

        fn make_row_closure(
            array: &'_ [Vec<CalculationResult>],
            origin: usize,
            size: usize,
        ) -> Box<dyn Fn(usize) -> Vec<CalculationResult> + '_> {
            Box::new(move |row| (
                origin
                ..origin+size
            ).map(
                |col| array[row][col].clone()
            ).collect())
        }

        let _vec = (
            origin.0
            ..cmp::min(origin.0+size.0, self.shape().0)
        ).map(
            make_row_closure(
                self.array.as_slice(),
                origin.1,
                cmp::min(
                    size.1,
                    self.shape().1 - origin.1
                )
            )
        ).collect();

        return Self{
            array:_vec
        }
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn chunks(
        &self,
        count: usize,
    ) -> ((usize, usize), (usize, usize)) {
        // Not yet functioning
        return (
            (0, 0),
            self.shape(),
        )
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
impl Serialize for IOResultArray {
    /// This guy is a fielded struct, and thus will Serialize as a dict. We need to change that.
    #[allow(dead_code)]
    fn serialize<S>(
        &self,
        serializer: S
    ) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let mut tup = serializer.serialize_tuple(self.array.len())?;

        let array = &self.array;

        for element in array {
            tup.serialize_element(&element)?;
        }
        tup.end()
    }
}


#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Bounds{
    upper_lat_bound: f64,
    upper_lng_bound: f64,
    lower_lat_bound: f64,
    lower_lng_bound: f64
}
impl Bounds {
    #[allow(dead_code)]
    pub fn new(
        upper_lat_bound: f64,
        upper_lng_bound: f64,
        lower_lat_bound: f64,
        lower_lng_bound: f64
    ) -> Self {
        return Self {
            upper_lat_bound: upper_lat_bound,
            upper_lng_bound: upper_lng_bound,
            lower_lat_bound: lower_lat_bound,
            lower_lng_bound: lower_lng_bound
        }
    }

    #[allow(dead_code)]
    pub fn as_tuple(&self) -> (f64, f64, f64, f64) {
        return (
            self.upper_lat_bound,
            self.upper_lng_bound,
            self.lower_lat_bound,
            self.lower_lng_bound
        )
    }

    #[allow(dead_code)]
    pub fn contains(&self, latlng:&LatLng) -> bool {
        let lat_in_bounds =
            latlng.lat <= self.upper_lat_bound
            && latlng.lat >= self.lower_lat_bound
        ;

        let lng_in_bounds = {
            if self.lower_lng_bound < self.upper_lng_bound {
                latlng.lng <= self.upper_lng_bound
                && latlng.lng >= self.lower_lng_bound
            } else {
                // The bound is looping around the international date line
                latlng.lng >= self.upper_lng_bound
                || latlng.lng <= self.lower_lng_bound
            }
        };

        return lat_in_bounds && lng_in_bounds;
    }
}
