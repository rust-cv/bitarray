#![feature(const_generics)]

pub struct ConstBytes<const N: usize>([u8; N]);

/// A constant sized array of bits. `N` defines the number of bits in the array.
pub struct BitArray<const N: usize> {
    bytes: ConstBytes<{(N + 7) / 8}>,
}

impl<const N: usize> BitArray<{N}> {
    fn new(bytes: ConstBytes<{(N + 7) / 8}>) -> Self {
        Self {
            bytes,
        }
    }
}

#[cfg(test)]
#[test]
fn test_zeros() {
    let array = BitArray::<5>::new(ConstBytes([0]));
    assert!(&array.storage.0 == &[0u8]);
}
