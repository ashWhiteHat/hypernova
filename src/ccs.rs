use crate::matrix::{DenseVectors, Element, SparseMatrix};

use zkstd::common::PrimeField;

#[allow(non_upper_case_globals)]
#[derive(Clone, Debug)]
pub struct Ccs<F: PrimeField, const t: usize, const q: usize> {
    // 1. Structure S
    // matrix length
    pub(crate) m: usize,
    // matrices
    pub(crate) matrices: [SparseMatrix<F>; t],
    // multisets
    pub(crate) multisets: [SparseMatrix<F>; q],
    // constants
    pub(crate) constants: [F; q],

    // 2. Instance
    pub(crate) x: DenseVectors<F>,

    // 3. Witness
    pub(crate) w: DenseVectors<F>,
}
