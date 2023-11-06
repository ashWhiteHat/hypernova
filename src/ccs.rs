use crate::matrix::{DenseVectors, Entry, SparseMatrix};

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
        // compute sum of q vector
        let mut final_vector = DenseVectors(vec![F::zero(); self.m]);
        (0..q).for_each(|i| {
            let mut product = DenseVectors::identity(self.m);
            self.multisets[i].iter().for_each(|j| {
                // matrix vector multiplication
                let vector = self.matrices[*j].prod(self.m, &self.x, &self.w);
                // vector hadamard product
                product = product.clone() * vector;
            });
            // constant scalar of matrix
            let c = self.constants[i];
            let q_vector = product * c;
            final_vector = final_vector.clone() + q_vector;
        });
        final_vector.iter().all(|vector| vector == F::zero())
    }
}

#[cfg(test)]
mod tests {
    use crate::test::example_r1cs;
    use jub_jub::Fr as Scalar;

    #[test]
    fn ccs_test() {
        for i in 1..10 {
            let r1cs = example_r1cs::<Scalar>(i);
            assert!(r1cs.to_ccs().is_sat())
        }
    }
}
