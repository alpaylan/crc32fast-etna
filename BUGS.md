# crc32fast — Injected Bugs

ETNA workload for the Rust `crc32fast` crate. Each variant re-introduces
one historical bug-fix into a fresh patched branch and pairs it with a
framework-neutral property, four PBT adapters, and a deterministic
witness test.

Total mutations: 1

## Bug Index

| # | Variant | Name | Location | Injection | Fix Commit |
|---|---------|------|----------|-----------|------------|
| 1 | `combine_zero_length_identity_724ceb6_1` | `combine_zero_length_identity` | `src/combine.rs:24` | `marauders` | `724ceb6d7f0b24fd2ac2be3461bdcefdae619703` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `combine_zero_length_identity_724ceb6_1` | `CombineZeroLengthIdentity` | `witness_combine_zero_length_identity_case_one_zero`, `witness_combine_zero_length_identity_case_deadbeef_cafebabe` |

## Framework Coverage

| Property | proptest | quickcheck | crabcheck | hegel |
|----------|---------:|-----------:|----------:|------:|
| `CombineZeroLengthIdentity` | ✓ | ✓ | ✓ | ✓ |

## Bug Details

### 1. combine_zero_length_identity

- **Variant**: `combine_zero_length_identity_724ceb6_1`
- **Location**: `src/combine.rs:24`
- **Property**: `CombineZeroLengthIdentity`
- **Witness(es)**:
  - `witness_combine_zero_length_identity_case_one_zero`
  - `witness_combine_zero_length_identity_case_deadbeef_cafebabe`
- **Source**: cover special case in combine
  > `combine()` missed a zero-length short-circuit: with `len2 == 0` and a non-zero `crc2`, the main path XOR'd `crc2` into `crc1` and returned `crc1 ^ crc2` instead of `crc1`. The fix adds `if len2 == 0 { return crc1; }` at the top of the function.
- **Fix commit**: `724ceb6d7f0b24fd2ac2be3461bdcefdae619703` — cover special case in combine
- **Invariant violated**: combining a `Hasher` with a zero-length `other` (even one whose internal CRC state is non-zero) must leave the receiver's finalized CRC unchanged — `combine(crc1, crc2, 0) == crc1`.
- **How the mutation triggers**: deleting the `if len2 == 0 { return crc1; }` early-return in `combine::combine` leaves the subsequent `p ^ crc2` path in charge for zero-length inputs. With `p = crc1` and a non-zero `crc2`, the function returns `crc1 ^ crc2` instead of `crc1`. The witness reaches this via `Hasher::new_with_initial(0x1)` → `Hasher::combine`, which forwards the initialized-but-unused hasher's state as `crc2` with `amount == 0`.
