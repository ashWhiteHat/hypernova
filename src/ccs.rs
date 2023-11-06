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
        let vectors: Vec<DenseVectors<F>> = (0..q)
            .map(|i| {
                // constant scalar of matrix
                let c = self.constants[i];
                // matrix indexes
                let s = self.multisets[i].clone();
                let j_vectors: Vec<DenseVectors<F>> = s
                    .iter()
                    .map(|j| {
                        let matrix = self.matrices[i].clone();
                        matrix.prod(self.m, &self.x, &self.w)
                    })
                    .collect();
                let mut acc = DenseVectors::identity(self.m);
                for vector in j_vectors {
                    acc = acc * vector
                }
                acc * c
            })
            .collect();
        let mut identity = DenseVectors(vec![F::zero(); self.m]);
        for vector in vectors {
            identity = identity + vector
        }
        identity.iter().all(|vector| vector == F::zero())
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
