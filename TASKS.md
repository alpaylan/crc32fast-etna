# crc32fast — ETNA Tasks

Total tasks: 4

ETNA tasks are **mutation/property/witness triplets**. Each row below is one runnable task.

## Task Index

| Task | Variant | Framework | Property | Witness | Command |
|------|---------|-----------|----------|---------|---------|
| 001  | `combine_zero_length_identity_724ceb6_1` | proptest    | `property_combine_zero_length_identity` | `witness_combine_zero_length_identity_case_one_zero` | `cargo run --release --features etna --bin etna -- proptest CombineZeroLengthIdentity` |
| 002  | `combine_zero_length_identity_724ceb6_1` | quickcheck  | `property_combine_zero_length_identity` | `witness_combine_zero_length_identity_case_one_zero` | `cargo run --release --features etna --bin etna -- quickcheck CombineZeroLengthIdentity` |
| 003  | `combine_zero_length_identity_724ceb6_1` | crabcheck   | `property_combine_zero_length_identity` | `witness_combine_zero_length_identity_case_one_zero` | `cargo run --release --features etna --bin etna -- crabcheck CombineZeroLengthIdentity` |
| 004  | `combine_zero_length_identity_724ceb6_1` | hegel       | `property_combine_zero_length_identity` | `witness_combine_zero_length_identity_case_one_zero` | `cargo run --release --features etna --bin etna -- hegel CombineZeroLengthIdentity` |

## Witness catalog

Each witness is a deterministic concrete test. Base build: passes. Variant-active build: fails.

- `witness_combine_zero_length_identity_case_one_zero` — `crc1_init=0x00000000, crc2_init=0x00000001` → pre-724ceb6 returns `0x00000001` instead of `0x00000000`.
- `witness_combine_zero_length_identity_case_deadbeef_cafebabe` — `crc1_init=0xDEADBEEF, crc2_init=0xCAFEBABE` → pre-724ceb6 returns `0x14530451` (i.e. `crc1 ^ crc2`) instead of `0xDEADBEEF`.
