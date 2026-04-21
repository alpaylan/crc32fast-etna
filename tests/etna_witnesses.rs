//! ETNA witness tests for the `crc32fast` crate.
//!
//! Each `witness_<name>_case_<tag>` calls one property function with the
//! exact canonical input used by the `etna` replay path
//! (`src/bin/etna.rs`). On base, every witness passes. On its paired
//! variant branch (or under `M_<variant>=active`), the witness fails.

#![cfg(feature = "etna")]

use crc32fast::etna::{property_combine_zero_length_identity, PropertyResult};

fn assert_pass(r: PropertyResult) {
    match r {
        PropertyResult::Pass | PropertyResult::Discard => {}
        PropertyResult::Fail(m) => panic!("property failed: {}", m),
    }
}

/// Triggers `combine_zero_length_identity_724ceb6_1`. Combining any
/// hasher with a zero-length `other` whose internal state is non-zero
/// must not change the receiver's finalized CRC; the buggy code
/// returned `crc1 ^ crc2`.
#[test]
fn witness_combine_zero_length_identity_case_one_zero() {
    assert_pass(property_combine_zero_length_identity(0x0000_0000, 0x0000_0001));
}

/// Second witness, independent input vectors — catches the same bug
/// with a different `(crc1, crc2)` pair to rule out a coincidence where
/// `crc1 ^ crc2 == crc1`.
#[test]
fn witness_combine_zero_length_identity_case_deadbeef_cafebabe() {
    assert_pass(property_combine_zero_length_identity(0xDEAD_BEEF, 0xCAFE_BABE));
}
