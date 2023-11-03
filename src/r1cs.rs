use crate::matrix::SparseMatrix;

use zkstd::common::TwistedEdwardsAffine;

#[derive(Clone, Debug)]
pub struct R1csStructure<C: TwistedEdwardsAffine> {
    /// matrix length
    pub(crate) m: usize,
    /// instance length
    pub(crate) l: usize,
    pub(crate) a: SparseMatrix<C::Scalar>,
    pub(crate) b: SparseMatrix<C::Scalar>,
    pub(crate) c: SparseMatrix<C::Scalar>,
}
