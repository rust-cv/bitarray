#![feature(const_generics)]

/// A constant sized array of bits. `N` defines the number of bits in the array.
pub struct BitArray<const N: usize> {
    storage: [u8; {(N + 7) / 8}],
}

impl<const N: usize> BitArray<{N}> {
    fn zeros() -> Self {
        Self {
            storage: [0; {(N + 7) / 8}],
        }
    }
}

#[cfg(test)]
#[test]
fn test_zeros() {
    let array = BitArray::<5>::zeros();
    assert!(&array.storage == &[0u8]);
}
