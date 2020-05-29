#![no_std]
#![allow(incomplete_features)]
#![feature(const_generics)]

use core::{
    fmt,
    hash::{Hash, Hasher},
};

/// A byte array with an alignment of 64 to maximize the efficiency of SIMD operations.
#[repr(align(64))]
pub struct ConstBytes<const N: usize>([u8; N]);

/// A constant sized array of bits. `B` defines the number of bytes.
/// This has an alignment of 64 to maximize the efficiency of SIMD operations.
/// It will automatically utilize SIMD at runtime where possible.
#[repr(align(64))]
#[derive(Clone)]
pub struct BitArray<const B: usize> {
    bytes: [u8; B],
}

impl<const B: usize> BitArray<B> {
    /// Create a new `BitArray`.
    ///
    /// ```
    /// use bitarray::BitArray;
    /// let array = BitArray::new([0]);
    /// assert_eq!(*array.bytes(), [0]);
    /// ```
    pub fn new(bytes: [u8; B]) -> Self {
        Self { bytes }
    }

    /// Create a new `BitArray` with all zeros.
    ///
    /// ```
    /// use bitarray::BitArray;
    /// let array = BitArray::new([0]);
    /// assert_eq!(array, BitArray::zeros());
    /// ```
    pub fn zeros() -> Self {
        Self { bytes: [0; B] }
    }

    /// Retrieve the byte array of a `BitArray`.
    ///
    /// ```
    /// use bitarray::BitArray;
    /// let array = BitArray::new([1, 2]);
    /// assert_eq!(*array.bytes(), [1, 2]);
    /// ```
    pub fn bytes(&self) -> &[u8; B] {
        &self.bytes
    }
}

impl<const B: usize> PartialEq for BitArray<B> {
    fn eq(&self, other: &Self) -> bool {
        self.bytes
            .iter()
            .zip(other.bytes.iter())
            .all(|(&a, &b)| a == b)
    }
}

impl<const B: usize> Eq for BitArray<B> {}

impl<const B: usize> fmt::Debug for BitArray<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.bytes[..].fmt(f)
    }
}

impl<const B: usize> Hash for BitArray<B> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bytes[..].hash(state)
    }
}
