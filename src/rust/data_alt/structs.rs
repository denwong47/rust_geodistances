/// ==============================
///  Structs for data structures.
/// ==============================
///
/// This is an alternative version which utilises `const` generics.
use std::{cmp, fmt};
use std::f64::consts::PI;

use crate::data::traits::{Slicable};

/// A point of latitude and longitude.
#[derive(Debug, Copy, Clone)]
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
impl fmt::Display for LatLng {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:06.2}{},{:06.2}{}",
            self.lat.abs(),
            match self.lat {
                v if v < 0. => "S",
                _ => "N",
            },
            self.lng.abs(),
            match self.lng {
                v if v < 0. => "W",
                _ => "E",
            },
        )
    }
}

/// An array of LatLng coordinates.
#[derive(Debug, Clone)]
pub struct CoordinateList<const L:usize>(pub [LatLng; L]);
impl<const L:usize> CoordinateList<L> {
    /// This is to get the inside value of the Tuple struct.
    pub fn value(&self) -> &[LatLng] {
        return &self.0
    }

    pub fn len(&self) -> usize {
        return L;
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
pub struct IOCoordinateLists<const L:usize>(pub [Option<[LatLng; L]>; 2]);
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
        return match self.1 {
            Some(_) => 2 as usize,
            None => 1 as usize,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CalculationResult{
    Geodistance(Option<f64>),
    WithinDistance(bool),
    Location(Option<LatLng>),
    Unpopulated,
}
impl fmt::Display for CalculationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>15}", match self {
            &CalculationResult::Geodistance(Some(distance)) => format!("{:8.2}km", distance),
            &CalculationResult::Geodistance(None) => "---".to_string(),
            &CalculationResult::WithinDistance(answer) => match answer {
                true => "Yes".to_string(),
                false => "No".to_string()
            },
            &CalculationResult::Location(Some(latlng)) => format!("{}", latlng),
            &CalculationResult::Location(None) => "---".to_string(),
            _ => "?".to_string(),
        })
    }
}

/// A result array of const size declared at definition time.
/// Example:
///     IOResultArray::<2,3>::full(CalculationResult::Geodistance(Some(10.)));
#[derive(Debug, Clone)]
pub struct IOResultArray<const A:usize, const B:usize>{
    pub array: [[CalculationResult; B]; A],
}
impl<const A:usize, const B:usize> IOResultArray<A,B>{
    pub fn new() -> Self {
        return Self::full(CalculationResult::Unpopulated)
    }

    pub fn full(value:CalculationResult) -> Self {
        return Self{
            array: [[value; B];A]
        }
    }

    pub fn mirror_fill(
        &mut self,
        diagonal_value: CalculationResult,
    ) {
        let max_dim = cmp::min(A, B);

        for row in 0..max_dim {
            self.array[row][row] = diagonal_value;

            for col in row+1..max_dim {
                self.array[row][col] = self.array[max_dim-1-row][max_dim-1-col]
            }
        }
    }

}
impl<const A:usize, const B:usize> Slicable for IOResultArray<A,B> {
    type SlicableType<const TA:usize, const TB:usize> = IOResultArray<A,B>;

    /// Return a tuple of (usize, usize) stating the shape of the underlying data.
    /// Assumes all secondary Vecs are the same size.
    #[allow(dead_code)]
    fn shape(&self) -> (usize, usize) {
        return (A, B);
    }

    /// Get a shallow copy sector of itself.
    fn sector<const U:usize, const V:usize>(
        &self,
        origin: (usize, usize),
    ) -> Self::SlicableType<U,V> {
        let (x, y) = origin;
        let (upper_x, upper_y) = (cmp::min(x+U, A), cmp::min(y+V, B));

        let mut sliced = Self::SlicableType::<U,V>::new();

        for row in x..upper_x {
            sliced.array[row-x][..upper_y-y].copy_from_slice(&self.array[row][y..upper_y])
        }

        return sliced
    }

    fn sector_replace<const U:usize, const V:usize>(
        &mut self,
        origin:(usize, usize),
        replace_with:Self::SlicableType<U, V>,
    ) {
        let (x, y) = origin;
        let (upper_x, upper_y) = (cmp::min(x+U, A), cmp::min(y+V, B));

        let array = self.array.as_mut_slice();

        for row in x..upper_x {
            array[row][y..upper_y].copy_from_slice(&replace_with.array[row-x][0..(upper_y-y)])
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