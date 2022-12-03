/// =================
///  Pickling traits
/// =================
///
pub trait PickleImport<T> {
    fn from_pickle(data:&[u8]) -> T;
}

pub trait PickleExport {
    fn to_pickle(&self) -> Vec<u8>;
}
