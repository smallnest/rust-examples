// Idiom #68 Create a bitset
// Create an object x to store n bits (n being potentially large).

use bitset_core::BitSet;

fn main() {
    let mut bits = [0u32; 4];
    assert_eq!(bits.bit_len(), 4 * 32);

    bits.bit_init(true); // Set all bits to true
    assert!(bits.bit_all()); // All bits are set

    bits.bit_reset(13); // Reset the 13th bit
    assert!(bits.bit_any()); // At least some bits are set

    bits.bit_flip(42); // Flip the 42nd bit twice (no change)
    bits.bit_flip(42);

    bits.bit_cond(1, false); // Set the bit to runtime value

    assert_eq!(bits.bit_test(42), true);
    assert_eq!(bits.bit_test(13), false);
    assert_eq!(bits.bit_test(1), false);

    assert_eq!(bits.bit_count(), 4 * 32 - 2);
}
