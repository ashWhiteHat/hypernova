use crate::ccs::Ccs;
use crate::matrix::{DenseVectors, SparseMatrix};

use zkstd::common::PrimeField;

#[derive(Clone, Debug, Default)]
pub struct R1cs<F: PrimeField> {
    // 1. Structure S
    // matrix column size
    pub(crate) m: usize,
    // matrix row size
    pub(crate) n: usize,
    // instance length
    pub(crate) l: usize,
    // a, b and c matrices
    pub(crate) a: SparseMatrix<F>,
    pub(crate) b: SparseMatrix<F>,
    pub(crate) c: SparseMatrix<F>,

    // 2. Instance
    pub(crate) x: DenseVectors<F>,

    // 3. Witness
    pub(crate) w: DenseVectors<F>,
}

impl<F: PrimeField> R1cs<F> {
    ///  check (A · Z) ◦ (B · Z) = C · Z
    pub fn is_sat(&self) -> bool {
        let R1cs {
            m,
            n: _,
            l: _,
            a,
            b,
            c,
            x,
            w,
        } = self.clone();
        // A · Z
        let az = a.prod(m, &x, &w);
        // B · Z
        let bz = b.prod(m, &x, &w);
        // C · Z
        let cz = c.prod(m, &x, &w);
        // (A · Z) ◦ (B · Z)
        let azbz = az * bz;

        azbz.iter()
            .zip(cz.iter())
            .all(|(left, right)| left == right)
    }

    pub fn to_ccs(&self) -> Ccs<F, 3, 2> {
        let R1cs {
            m,
            n: _,
            l: _,
            a,
            b,
            c,
            x,
            w,
        } = self.clone();
        let matrices = [a, b, c];
        let multisets = [vec![0, 1], vec![2]];
        let constants = [F::one(), -F::one()];

        Ccs {
            m,
            matrices,
            multisets,
            constants,
            x,
            w,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::example_r1cs;
    use jub_jub::Fr as Scalar;

    #[test]
    fn r1cs_test() {
        for i in 1..10 {
            let r1cs = example_r1cs::<Scalar>(i);
            assert!(r1cs.is_sat())
        }
    }
}
