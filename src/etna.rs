//! ETNA framework-neutral property functions for the `crc32fast` crate.
//!
//! Each `property_<name>` is a pure function over concrete, owned inputs
//! returning `PropertyResult`. Framework adapters in `src/bin/etna.rs` and
//! witness tests in `tests/etna_witnesses.rs` call these directly.

#![allow(missing_docs)]

use crate::Hasher;

#[derive(Debug)]
pub enum PropertyResult {
    Pass,
    Fail(String),
    Discard,
}

// ---------------------------------------------------------------------------
// combine_zero_length_identity_724ceb6_1
// ---------------------------------------------------------------------------

/// Invariant: `Hasher::combine(&other)` must leave the receiver's finalized
/// value unchanged whenever `other` represents zero bytes (`amount == 0`).
/// Combining with an empty chunk is the identity on the accumulated CRC.
///
/// Bug this catches:
/// - `combine_zero_length_identity_724ceb6_1`: pre-fix, `combine::combine`
///   omitted the `len2 == 0` early-return and fell through to `p ^ crc2`,
///   returning `crc1 ^ other_finalized_state` instead of `crc1`. The bug
///   only manifests when `other_finalized_state != 0`, which is reachable
///   via `Hasher::new_with_initial(non_zero)`.
pub fn property_combine_zero_length_identity(crc1_init: u32, crc2_init: u32) -> PropertyResult {
    let h = Hasher::new_with_initial(crc1_init);
    let before = h.clone().finalize();

    let other = Hasher::new_with_initial(crc2_init);
    if other.clone().finalize() != crc2_init {
        return PropertyResult::Discard;
    }

    let mut combined = h.clone();
    combined.combine(&other);
    let after = combined.finalize();

    if before != after {
        return PropertyResult::Fail(format!(
            "combine with zero-length other changed crc: {before:#010x} -> {after:#010x} (crc1_init={crc1_init:#010x}, crc2_init={crc2_init:#010x})"
        ));
    }
    PropertyResult::Pass
}
