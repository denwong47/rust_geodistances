/// A struct that can be sliced into subset of itself.
/// The slice returned will not be a reference, but a shallow copy.
pub trait Slicable {
    type SlicableType<const A:usize, const B:usize>;

    fn shape(&self) -> (usize, usize);

    fn sector<const U:usize, const V:usize>(
        &self,
        origin: (usize, usize),
    ) -> Self::SlicableType<U,V>;

    fn sector_replace<const U:usize, const V:usize>(
        &mut self,
        origin:(usize, usize),
        replace_with:Self::SlicableType<U, V>,
    );

    fn chunks(
        &self,
        count: usize,
    ) -> ((usize, usize), (usize, usize));
}
