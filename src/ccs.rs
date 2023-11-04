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
    pub(crate) multisets: [Vec<usize>; q],
    // constants
    pub(crate) constants: [F; q],

    // 2. Instance
    pub(crate) x: DenseVectors<F>,

    // 3. Witness
    pub(crate) w: DenseVectors<F>,
}

impl<F: PrimeField, const t: usize, const q: usize> Ccs<F, t, q> {
    pub(crate) fn is_sat(&self) -> bool {
        (0..q).all(|i| true)
    }
}
