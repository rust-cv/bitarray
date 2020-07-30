use crate::BitArray;
use core::fmt;
use serde::{
    de::{Error, Expected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

impl<const B: usize> Serialize for BitArray<B> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bytes[..].serialize(serializer)
    }
}

impl<'de, const B: usize> Deserialize<'de> for BitArray<B> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(BitArrayVisitor::<B>)
    }
}

struct BitArrayVisitor<const B: usize>;

impl<'de, const B: usize> Visitor<'de> for BitArrayVisitor<B> {
    type Value = BitArray<B>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "[u8; {}]", B)
    }

    fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if bytes.len() != B {
            Err(Error::invalid_length(
                bytes.len(),
                &BitArrayExpectedBytes::<B>,
            ))
        } else {
            let mut bitarray = BitArray::<B>::zeros();
            bitarray.bytes.copy_from_slice(bytes);
            Ok(bitarray)
        }
    }
}

struct BitArrayExpectedBytes<const B: usize>;

impl<const B: usize> Expected for BitArrayExpectedBytes<B> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{} bytes was expected", B)
    }
}
