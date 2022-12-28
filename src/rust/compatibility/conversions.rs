/// Simplified type conversions unique to this package.
///
/// These are type conversions that do not belong to :mod:`ndarray_numeric` but
/// still integral to :mod:`rust_geodistances`.

use duplicate::duplicate_item;
use ndarray::{
    ArcArray2,
    Array2,
    ArrayView2,
    ArrayViewMut2,
    Axis,
    Ix2,
};
use pyo3::{
    ToPyObject,
};

use ndarray_numeric::{
    BoolArcArray2,
    BoolArray2,
    BoolArrayView,
    BoolArrayViewMut,
    ArrayWithBoolIterMethods,
};

pub trait Array2ToVecVec<A> {
    fn to_vec(&self) -> Vec<Vec<A>>;
}

/// Allows two-dimensional arrays to convert into a `Vec` of `Vec`s.
/// This conversion is not efficient and should be avoided if possible.
#[duplicate_item(
    __array2_type__                 __impl_generics__;
    [ Array2<A> ]                   [ A ];
    [ ArcArray2<A> ]                [ A ];
    [ &ArrayView2<'a, A> ]          [ 'a, A ];
    [ &ArrayViewMut2<'a, A> ]       [ 'a, A ];
)]
impl<__impl_generics__> Array2ToVecVec<A> for __array2_type__
where A: Clone+ToPyObject {
    fn to_vec(&self) -> Vec<Vec<A>> {
        return self.axis_iter(Axis(0))
                   .map(
                       | row | row.to_vec()
                   )
                   .collect()
    }
}


pub trait BoolArrayToVecIndex {
    fn to_vec_of_indices(&self) -> Vec<Vec<usize>>;
}

/// Allows two-dimensional `bool` arrays to convert into a `Vec` of `Vec<usize>`s.
/// This reduces the `into_py` conversion cost somewhat, but is still a `Vec` of `Vec`.
///
/// .. deprecated:: 0.2.2
///     This function is no longer required as indices functions now returns
///     Arrays instead of `Vec<Vec<usize>>`
#[duplicate_item(
    __array2_type__                 __impl_generics__;
    [ BoolArray2 ]                  [ ];
    [ BoolArcArray2 ]               [ ];
    [ &BoolArrayView<'a, Ix2> ]     [ 'a ];
    [ &BoolArrayViewMut<'a, Ix2> ]  [ 'a ];
)]
impl<__impl_generics__> BoolArrayToVecIndex for __array2_type__ {
    fn to_vec_of_indices(&self) -> Vec<Vec<usize>> {
        return self.axis_iter(Axis(0))
                   .map(
                       | row | row
                                .indices()
                                .to_vec()
                   )
                   .collect()
    }
}
