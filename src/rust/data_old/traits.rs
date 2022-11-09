/// A struct that can be sliced into subset of itself.
/// The slice returned will not be a reference, but a shallow copy.
pub trait Slicable {
    fn shape(&self) -> (usize, usize);

    fn slice(
        &self,
        origin: (usize, usize),
        size: (usize, usize),
    ) -> Self;

    fn chunks(
        &self,
        count: usize,
    ) -> ((usize, usize), (usize, usize));
}
