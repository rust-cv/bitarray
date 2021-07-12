#![cfg(feature = "serde")]

use bitarray::BitArray;

#[test]
fn bincode_serde_json_cycle() {
    let old_bits = vec![BitArray::new([0, 1, 2, 3, 255])];
    let mut bdata = vec![];
    bincode::serialize_into(&mut bdata, &old_bits).expect("failed to serialize with bincode");
    let middle_bits: Vec<BitArray<5>> =
        bincode::deserialize_from(bdata.as_slice()).expect("failed to deserialize with bincode");
    let new_bits: Vec<BitArray<5>> = serde_json::from_str(
        &serde_json::to_string(&middle_bits).expect("failed to serialize with serde_json"),
    )
    .expect("failed to deserialize with serde_json");
    assert_eq!(old_bits, new_bits);
}
