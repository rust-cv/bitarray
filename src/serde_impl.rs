use crate::BitArray;
use core::fmt;
use serde::{
    de::{Error, Expected, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

impl<const B: usize> Serialize for BitArray<B> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.bytes)
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

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut arr = [0u8; B];
        let mut ix = 0;
        // Continuously fill the array with more values.
        while let Some(value) = seq.next_element()? {
            if ix == B {
                return Err(Error::custom("bitarray: too many bytes in sequence"));
            }
            arr[ix] = value;
            ix += 1;
        }

        if ix != B {
            Err(Error::invalid_length(ix, &BitArrayExpectedBytes::<B>))
        } else {
            Ok(BitArray::new(arr))
        }
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
        write!(formatter, "{} bytes", B)
    }
}
