//! Fault-localization integration tests for crc32fast.
//!
//! One `#[test]` per property in src/bin/etna-faultloc.rs's dispatch.
//! Each test runs `crabcheck::quickcheck_with_locate!` on the property,
//! prints the report, and emits a single `@@LOCATE@@ {<json>}` line on
//! stdout. Tests never panic — the driver classifies success/failure
//! from the JSON.

#![cfg(feature = "etna")]

use std::fmt;

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

fn property_combine_zero_length_identity_test(v: CombineZeroInput) -> Option<bool> {
    to_opt(property_combine_zero_length_identity(v.crc1_init, v.crc2_init))
}

fn emit_locate_json(r: &crabcheck::profiling::LocateResult) {
    use crabcheck::quickcheck::ResultStatus;
    let status = match &r.run.status {
        ResultStatus::Failed { .. } => "Failed",
        ResultStatus::Finished => "Finished",
        ResultStatus::GaveUp => "GaveUp",
        ResultStatus::TimedOut => "TimedOut",
        ResultStatus::Aborted { .. } => "Aborted",
    };
    let top = if let Some(s) = r.top() {
        serde_json::json!({
            "rank": s.rank,
            "file": s.region.file,
            "function": s.region.function,
            "start_line": s.region.start_line,
            "end_line": s.region.end_line,
            "ochiai": s.region.suspiciousness.ochiai,
            "delta": s.region.delta,
            "panic_overlap": s.panic_overlap,
            "confidence": format!("{}", s.confidence),
            "confidence_rule": s.confidence_rule,
        })
    } else {
        serde_json::Value::Null
    };
    let top_5: Vec<_> = r
        .suspects
        .iter()
        .take(5)
        .map(|s| {
            serde_json::json!({
                "rank": s.rank,
                "file": s.region.file,
                "function": s.region.function,
                "start_line": s.region.start_line,
                "end_line": s.region.end_line,
                "confidence": format!("{}", s.confidence),
                "confidence_rule": s.confidence_rule,
                "panic_overlap": s.panic_overlap,
            })
        })
        .collect();
    let diags: Vec<_> = r.diagnostics.iter().map(|d| d.tag()).collect();
    let out = serde_json::json!({
        "status": status,
        "passed": r.run.passed,
        "discarded": r.run.discarded,
        "n_panics": r.n_panics,
        "n_suspects": r.suspects.len(),
        "top": top,
        "top_5": top_5,
        "diagnostics": diags,
    });
    println!("@@LOCATE@@ {}", out);
}

#[test]
fn locate_combine_zero_length_identity() {
    let report =
        crabcheck::quickcheck_with_locate!(property_combine_zero_length_identity_test, "crc32fast");
    eprintln!("{report}");
    emit_locate_json(&report);
}
