use crate::ccs::Ccs;
use crate::matrix::{DenseVectors, Element, SparseMatrix};
use crate::wire::Wire;

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
    pub fn is_sat(&self) -> bool {
        let R1cs {
            m,
            n: _,
            l: _,
            a,
            b,
            c,
            x: _,
            w: _,
        } = self.clone();
        (0..m).all(|i| {
            let a_prod = self.dot_product(&a[i]);
            let b_prod = self.dot_product(&b[i]);
            let c_prod = self.dot_product(&c[i]);
            a_prod * b_prod == c_prod
        })
    }

    fn dot_product(&self, elements: &Vec<Element<F>>) -> F {
        elements.iter().fold(F::zero(), |sum, element| {
            let (wire, value) = element.get();
            let coeff = match wire {
                Wire::Witness(index) => self.w[index],
                Wire::Instance(index) => self.x[index],
                Wire::One => F::one(),
            };
            sum + coeff * value
        })
    }

    fn to_ccs(&self) -> Ccs<F, 3, 2> {
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
    use super::*;
    use jub_jub::Fr as Scalar;

    fn array_to_witnessess<F: PrimeField>(witnesses: Vec<u64>) -> Vec<F> {
        witnesses
            .iter()
            .skip(1)
            .map(|witness| F::from(*witness))
            .collect::<Vec<_>>()
    }

    fn dense_to_sparse<F: PrimeField>(value: Vec<Vec<u64>>, l: usize) -> SparseMatrix<F> {
        let sparse_matrix = value
            .iter()
            .map(|elements| {
                elements
                    .iter()
                    .enumerate()
                    .map(|(index, element)| {
                        if index == 0 {
                            Element(Wire::One, F::from(*element))
                        } else if index <= l {
                            let index = index - 1;
                            Element(Wire::instance(index), F::from(*element))
                        } else {
                            let index = index - 1 - l;
                            Element(Wire::witness(index), F::from(*element))
                        }
                    })
                    .filter(|element| element.1 != F::zero())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        SparseMatrix(sparse_matrix)
    }

    fn example_z_witness<F: PrimeField>(
        input: u64,
        l: usize,
    ) -> (DenseVectors<F>, DenseVectors<F>) {
        let z = array_to_witnessess(vec![
            1,
            input,
            input * input * input + input + 5,
            input * input,
            input * input * input,
            input * input * input + input,
        ]);
        let (public_inputs, witness) = z.split_at(l);
        (
            DenseVectors(public_inputs.to_vec()),
            DenseVectors(witness.to_vec()),
        )
    }

    fn example_r1cs<F: PrimeField>(input: u64) -> R1cs<F> {
        let m = 4;
        let n = 6;
        let l = 1;
        let a = dense_to_sparse(
            vec![
                vec![0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 0],
                vec![5, 0, 0, 0, 0, 1],
            ],
            l,
        );
        let b = dense_to_sparse(
            vec![
                vec![0, 1, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0, 0],
            ],
            l,
        );
        let c = dense_to_sparse(
            vec![
                vec![0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 1],
                vec![0, 0, 1, 0, 0, 0],
            ],
            l,
        );
        let (x, w) = example_z_witness(input, l);
        R1cs {
            m,
            n,
            l,
            a,
            b,
            c,
            x,
            w,
        }
    }

    #[test]
    fn r1cs_test() {
        for i in 1..10 {
            let r1cs = example_r1cs::<Scalar>(i);
            assert!(r1cs.is_sat())
        }
    }
}
