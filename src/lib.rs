#![no_std]
#![allow(incomplete_features)]
#![feature(const_generics)]

use core::{
    fmt,
    hash::{Hash, Hasher},
    slice,
};

/// A constant sized array of bits. `B` defines the number of bytes.
/// This has an alignment of 64 to maximize the efficiency of SIMD operations.
/// It will automatically utilize SIMD at runtime where possible.
#[repr(align(8))]
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

    /// Compute the hamming weight of the `BitArray`.
    ///
    /// ```
    /// use bitarray::BitArray;
    /// let array = BitArray::new([0xAA; 65]);
    /// assert_eq!(array.weight(), 4 * 65);
    /// ```
    #[allow(clippy::cast_ptr_alignment)]
    pub fn weight(&self) -> usize {
        let simd_len = B >> 3;
        let simd_bytes = simd_len << 3;
        let simd_sum = unsafe {
            slice::from_raw_parts(self.bytes.as_ptr() as *const u64, simd_len)
                .iter()
                .copied()
                .map(|chunk| chunk.count_ones() as usize)
                .sum::<usize>()
        };
        let remaining_sum = self.bytes[simd_bytes..]
            .iter()
            .copied()
            .map(|b| b.count_ones() as usize)
            .sum::<usize>();
        simd_sum + remaining_sum
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
