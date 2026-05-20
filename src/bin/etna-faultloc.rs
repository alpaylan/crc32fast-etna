// Crabcheck fault-localization runner for crc32fast.
//
// Mirrors workloads/Rust/aho-corasick/src/bin/etna-faultloc.rs and
// BST/RBT/STLC's faultloc: drives each property via
// `crabcheck::profiling::quickcheck`, which instruments every iteration
// with LLVM coverage and snapshots .profraw files around the failing
// seed for SBFL analysis.
//
// Self-contained on purpose — the existing `etna` runner in
// src/bin/etna.rs stays untouched.

use std::fmt;

use crabcheck::profiling::quickcheck;
use crabcheck::quickcheck::{Arbitrary, Mutate};
use crc32fast::etna::{property_combine_zero_length_identity, PropertyResult};
use rand_etna::Rng;

#[derive(Clone, Copy)]
struct CombineZeroInput {
    crc1_init: u32,
    crc2_init: u32,
}

impl fmt::Debug for CombineZeroInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "crc1_init={:#010x} crc2_init={:#010x}",
            self.crc1_init, self.crc2_init
        )
    }
}

impl<R: Rng> Arbitrary<R> for CombineZeroInput {
    fn generate(rng: &mut R, _n: usize) -> Self {
        CombineZeroInput {
            crc1_init: rng.random::<u32>(),
            crc2_init: rng.random::<u32>(),
        }
    }
}

// BST-style single-point perturbation: flip one random bit in one of the
// two u32 fields. Keeps the mutant close to the failing seed so the
// fault-localization signal stays strong.
impl<R: Rng> Mutate<R> for CombineZeroInput {
    fn mutate(&self, rng: &mut R, _n: usize) -> Self {
        let which = rng.random_range(0u8..2);
        let bit = rng.random_range(0u32..32);
        let mask = 1u32 << bit;
        match which {
            0 => CombineZeroInput {
                crc1_init: self.crc1_init ^ mask,
                crc2_init: self.crc2_init,
            },
            _ => CombineZeroInput {
                crc1_init: self.crc1_init,
                crc2_init: self.crc2_init ^ mask,
            },
        }
    }
}

fn to_opt(r: PropertyResult) -> Option<bool> {
    match r {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        eprintln!("Usage: {} <tool> <property> [tests]", args[0]);
        eprintln!("  tool:     crabcheck");
        eprintln!("  property: CombineZeroLengthIdentity");
        return;
    }
    let tool = args[1].as_str();
    let property = args[2].as_str();

    let result = match (tool, property) {
        ("crabcheck", "CombineZeroLengthIdentity") => {
            quickcheck(|v: CombineZeroInput| {
                to_opt(property_combine_zero_length_identity(v.crc1_init, v.crc2_init))
            })
        },
        _ => panic!("Unknown tool or property: {tool} {property}"),
    };

    println!("Result: {:?}", result);
}
