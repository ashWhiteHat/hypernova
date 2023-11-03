use crate::wire::Wire;

use std::ops::{Index, IndexMut};
use zkstd::common::{Add, Mul, PrimeField, Sub};
#[derive(Clone, Debug, Default)]
pub(crate) struct SparseMatrix<F: PrimeField>(pub(crate) Vec<Vec<Element<F>>>);

impl<F: PrimeField> SparseMatrix<F> {
    pub(crate) fn prod(
        &self,
        m: usize,
        x: &DenseVectors<F>,
        w: &DenseVectors<F>,
    ) -> DenseVectors<F> {
        let mut vectors = DenseVectors(vec![F::zero(); m]);
        for (index, elements) in self.0.iter().enumerate() {
            vectors[index] = elements.iter().fold(F::zero(), |sum, element| {
                let (wire, coeff) = element.get();
                let value = match wire {
                    Wire::Instance(i) => x[i],
                    Wire::Witness(i) => w[i],
                    Wire::One => F::one(),
                };
                sum + coeff * value
            })
        }
        vectors
    }
}

impl<F: PrimeField> Index<usize> for SparseMatrix<F> {
    type Output = Vec<Element<F>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<F: PrimeField> IndexMut<usize> for SparseMatrix<F> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Element<F: PrimeField>(pub(crate) Wire, pub(crate) F);

impl<F: PrimeField> Element<F> {
    pub(crate) fn get(&self) -> (Wire, F) {
        (self.0, self.1)
    }
}

impl<F: PrimeField> From<Wire> for Element<F> {
    fn from(value: Wire) -> Self {
        Self(value, F::one())
    }
}

impl<F: PrimeField> From<F> for Element<F> {
    fn from(value: F) -> Self {
        Self(Wire::one(), value)
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct DenseVectors<F: PrimeField>(pub(crate) Vec<F>);

impl<F: PrimeField> DenseVectors<F> {
    pub(crate) fn iter(&self) -> DenseVectorsIterator<F> {
        DenseVectorsIterator {
            dense_vectors: self.clone(),
            index: 0,
        }
    }
}

pub(crate) struct DenseVectorsIterator<F: PrimeField> {
    dense_vectors: DenseVectors<F>,
    index: usize,
}

impl<F: PrimeField> Iterator for DenseVectorsIterator<F> {
    type Item = F;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.dense_vectors.0.len() {
            let item = Some(self.dense_vectors[self.index]);
            self.index += 1;
            item
        } else {
            None
        }
    }
}

impl<F: PrimeField> Index<usize> for DenseVectors<F> {
    type Output = F;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<F: PrimeField> IndexMut<usize> for DenseVectors<F> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<F: PrimeField> Mul<F> for DenseVectors<F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self {
        Self(self.iter().map(|element| element * rhs).collect())
    }
}

/// Hadamard product
impl<F: PrimeField> Mul for DenseVectors<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len());

        Self(self.iter().zip(rhs.iter()).map(|(a, b)| a * b).collect())
    }
}

impl<F: PrimeField> Add for DenseVectors<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len());

        Self(self.iter().zip(rhs.iter()).map(|(a, b)| a + b).collect())
    }
}

impl<F: PrimeField> Sub for DenseVectors<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0.len(), rhs.0.len());

        Self(self.iter().zip(rhs.iter()).map(|(a, b)| a - b).collect())
    }
}
