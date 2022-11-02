use std::cmp;

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

/// A result array of variable size to
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

    pub fn shape(&self) -> (usize, usize) {
        return (A, B);
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
