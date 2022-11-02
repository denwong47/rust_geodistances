use std::cmp;

use crate::data::traits::{Slicable};

#[derive(Debug, Copy, Clone)]
pub struct LatLng{
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Copy, Clone)]
pub enum CalculationResult{
    Geodistance(Option<f64>),
    WithinDistance(bool),
    Location(Option<LatLng>),
    Unpopulated,
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

    pub fn splice<const U:usize, const V:usize>(
        &mut self,
        origin:(usize, usize),
        replace_with:IOResultArray<U, V>,
    ) {
        let (x, y) = origin;
        let (upper_x, upper_y) = (cmp::min(x+U, A), cmp::min(y+V, B));

        let array = self.array.as_mut_slice();

        for row in x..upper_x {
            array[row][y..upper_y].copy_from_slice(&replace_with.array[row-x][0..(upper_y-y)])
        }
    }

}
impl<const A:usize, const B:usize> Slicable for IOResultArray<A,B> {

    /// Return a tuple of (usize, usize) stating the shape of the underlying data.
    /// Assumes all secondary Vecs are the same size.
    #[allow(dead_code)]
    fn shape(&self) -> (usize, usize) {
        return (A, B);
    }

    /// Get a shallow copy slice of itself.
    fn slice<const U:usize, const V:usize>(
        &self,
        origin: (usize, usize),
    ) -> IOResultArray<U,V> {
        let (x, y) = origin;
        let (upper_x, upper_y) = (cmp::min(x+U, A), cmp::min(y+V, B));

        let mut sliced = IOResultArray::<U,V>::new();

        for row in x..upper_x {
            sliced.array[row-x][..upper_y-y].copy_from_slice(&self.array[row][y..upper_y])
        }

        return sliced
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
