//! Is `squash_noise()` deterministic for a fixed input?
//!
//! Yes. That is the answer these arms establish, and the reason to keep them.
//!
//! Motivation: on a three-operator stack, operators agreeing byte-for-byte on a
//! computed ciphertext were observed to disagree on the SNS digest for it --
//! roughly one add in four, always the same two values, with identical compute
//! digests on every handle. That is the shape of a squash with two reachable
//! outcomes, so these tests asked the question directly and in one process:
//!
//! if the same input squashed twice in ONE process yielded two answers, the
//! non-determinism would be inside tfhe-rs (or the compressed encoding) and the
//! next step a minimal repro for the tfhe team; if one process were
//! self-consistent, the difference would be environmental and the repro would
//! have to compare processes rather than iterations.
//!
//! Every arm came back deterministic -- 150/150 identical digests, across
//! iterations, processes, `RAYON_NUM_THREADS` 1/2/4/8, decompression inside the
//! loop, two independent ServerKey clones, sns-worker's own
//! `rayon::broadcast` + `par_iter_mut` policy, and on production-shaped
//! roundtripped input. Noiseless (trivial) and noisy operands alike.
//!
//! RESOLVED, and not in tfhe: the test host was running two sns-workers
//! against each operator database -- the stack's CPU-only container and a
//! leftover CUDA host worker from `gpu-consensus-workers.sh`. Both poll with
//! `FOR UPDATE SKIP LOCKED`, so each row was squashed by whichever process won
//! it, and the two backends produce different (both correct) ct128 bytes for
//! the same ct64. The two recurring values were one per backend; the "rate" was
//! the race. Proved by intervention in both directions -- CPU-only recompute
//! converged all 24 divergent handles, and a GPU-only recompute of one handle
//! reproduced the other value on demand. Recorded as B-1 (closed, not a bug)
//! and L-6 in the Consensus Defect Log; the harness now refuses a stack with
//! two workers on one queue.
//!
//! They are kept as the standing evidence that the squash itself is
//! deterministic, so the next time digests disagree this question does not have
//! to be re-opened from scratch. They need an extracted keyset and are a
//! diagnostic rather than a gate. Run with
//!
//!   cargo test -p sns-worker --release squash_determinism -- --ignored --nocapture

use crate::squash_noise::SquashNoiseCiphertext;
use fhevm_engine_common::types::SupportedFheCiphertexts;
use sha3::{Digest, Keccak256};
use std::collections::BTreeMap;
use tfhe::prelude::{CiphertextList, FheTryTrivialEncrypt};
use tfhe::xof_key_set::CompressedXofKeySet;

/// Overridable, because ruling out a low per-call divergence rate needs many
/// samples: twelve in-process repeats finding none is not evidence of
/// determinism if the rate is a few percent. (The rate that prompted this was
/// not per-call at all -- it was which of two workers won each row -- but the
/// knob is what let the sample size grow enough to say so.)
fn iterations() -> usize {
    std::env::var("SQUASH_ITERATIONS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(12)
}

fn digest_of(bytes: &[u8]) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

/// Squashes one ciphertext `ITERATIONS` times, returning digest -> count.
///
/// Compression is enabled to match the deployed configuration: every operator
/// runs sns-worker with `--enable-compression`, so the bytes that get hashed
/// into `ciphertext_digest.ciphertext128` are the compressed ones.
fn squash_repeatedly(ct: &SupportedFheCiphertexts, label: &str) -> BTreeMap<String, usize> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    let total = iterations();
    for iteration in 0..total {
        let serialized = ct.squash_noise_and_serialize(true).unwrap_or_else(|err| {
            panic!("{label}: squash failed on iteration {iteration}: {err:?}")
        });
        *counts.entry(digest_of(&serialized)).or_insert(0) += 1;
    }
    println!(
        "[squash-determinism] {label}: {} distinct result(s) over {total} squashes of the SAME ciphertext",
        counts.len()
    );
    for (digest, count) in &counts {
        println!("[squash-determinism]   {}  x{count}", &digest[..24]);
    }
    counts
}

#[test]
#[ignore = "loads a 444 MB keyset fixture; diagnostic, not a gate"]
fn squash_determinism_for_a_fixed_ciphertext() {
    let keyset_bytes =
        std::fs::read("../fhevm-keys/xof-keyset").expect("read the xof-keyset fixture");
    let keyset: CompressedXofKeySet =
        fhevm_engine_common::utils::safe_deserialize_key(&keyset_bytes)
            .expect("deserialize CompressedXofKeySet");
    let (compact_public_key, server_key) = keyset
        .decompress()
        .expect("decompress the xof keyset")
        .into_raw_parts();
    let server_key_for_workers = server_key.clone();
    tfhe::set_server_key(server_key);

    // Encrypt the fixture's own operands under the compact public key, the
    // same path production inputs take.
    let mut builder = tfhe::CompactCiphertextList::builder(&compact_public_key);
    builder.push(7_u64);
    builder.push(5_u64);
    let expanded = builder
        .build()
        .expand()
        .expect("expand the compact ciphertext list");
    let lhs: tfhe::FheUint64 = expanded.get(0).expect("get(0)").expect("element 0");
    let rhs: tfhe::FheUint64 = expanded.get(1).expect("get(1)").expect("element 1");

    // Arm 1 — the noisy case: exactly what AliasFixture.combineFromStorage
    // produces, an add over two operands. This is the shape that diverges in
    // production.
    let sum = &lhs + &rhs;
    let noisy = SupportedFheCiphertexts::FheUint64(sum);
    let noisy_counts = squash_repeatedly(&noisy, "add(7,5) — noisy");

    // Arm 2 — the control: a trivially encrypted value, which production
    // never saw diverge (80/80 identical).
    let trivial = SupportedFheCiphertexts::FheUint64(
        tfhe::FheUint64::try_encrypt_trivial(12_u64).expect("trivial encrypt"),
    );
    let trivial_counts = squash_repeatedly(&trivial, "trivial(12) — noiseless");

    // Arm 3 — the production shape. sns-worker never squashes a freshly
    // computed ciphertext: it squashes the decompressed form of the stored
    // compressed ct64. The roundtrip is documented as bit-inexact for noisy
    // ciphertexts, so this input can differ from arm 1's in exactly the way
    // that matters.
    let sum_again = SupportedFheCiphertexts::FheUint64(&lhs + &rhs);
    let ct_type = sum_again.type_num();
    let compressed = sum_again
        .compress()
        .expect("compress the add result as the DB stores it");
    let roundtripped = SupportedFheCiphertexts::decompress_no_memcheck(ct_type, &compressed)
        .expect("decompress the way sns-worker does");
    let roundtrip_counts = squash_repeatedly(
        &roundtripped,
        "decompress(compress(add)) — production shape",
    );

    // Arm 5 — decompress INSIDE the loop, which is what arms 1-3 failed to do.
    //
    // Each operator independently decompresses the same stored bytes and then
    // squashes. Arm 3 decompressed once and squashed twelve times, so it
    // proved squash determinism and said nothing about decompression. If
    // `decompress_no_memcheck` is itself non-deterministic, every operator
    // feeds the squash a slightly different ciphertext while the STORED bytes
    // stay byte-identical -- which is exactly the production observation:
    // identical ct64 (same md5, same length, same key id), divergent ct128.
    //
    // This repository already documents the roundtrip as bit-inexact for noisy
    // ciphertexts; arms 1 and 3 showed it also changes the squash output
    // (aa4b38b2 fresh vs e682a37e roundtripped). The open question is whether
    // it changes it the SAME way every time.
    let mut per_decompress: BTreeMap<String, usize> = BTreeMap::new();
    let total = iterations();
    for iteration in 0..total {
        let fresh = SupportedFheCiphertexts::decompress_no_memcheck(ct_type, &compressed)
            .unwrap_or_else(|err| panic!("decompress failed on iteration {iteration}: {err:?}"));
        let bytes = fresh
            .squash_noise_and_serialize(true)
            .expect("squash the freshly decompressed ciphertext");
        *per_decompress.entry(digest_of(&bytes)).or_insert(0) += 1;
    }
    println!(
        "[squash-determinism] decompress-then-squash, decompressing each time: {} distinct result(s)",
        per_decompress.len()
    );
    for (digest, count) in &per_decompress {
        println!("[squash-determinism]   {}  x{count}", &digest[..24]);
    }
    if per_decompress.len() > 1 {
        println!(
            "[squash-determinism] DECOMPRESSION IS THE NON-DETERMINISTIC STEP. The stored bytes are \
             identical on every operator and the squash is deterministic, so this is what makes \
             their ct128 differ. A minimal repro for the tfhe team is: compress once, then \
             decompress the same bytes twice and compare."
        );
    }

    // Arm 6 — the concurrent path, which is how production actually squashes.
    //
    // sns-worker's RayonParallel policy seeds each worker's thread-local key
    // with `rayon::broadcast` and then runs `batch.par_iter_mut()`
    // (executor.rs:660-676). Every arm so far squashed one ciphertext at a
    // time on the calling thread, so none of them exercised that. If two
    // workers sharing a cloned server key can produce different bytes for the
    // same input, this is where it shows.
    //
    // A batch of distinct ciphertexts is squashed serially, then through the
    // production concurrent shape, and the two are compared per index. The
    // parallel arm runs several times because a race need not fire on the
    // first attempt.
    {
        use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

        const BATCH: u64 = 8;
        let batch: Vec<SupportedFheCiphertexts> = (0..BATCH)
            .map(|i| {
                let mut b = tfhe::CompactCiphertextList::builder(&compact_public_key);
                b.push(i + 1);
                b.push(i + 2);
                let e = b.build().expand().expect("expand batch entry");
                let a: tfhe::FheUint64 = e.get(0).expect("get(0)").expect("elem 0");
                let c: tfhe::FheUint64 = e.get(1).expect("get(1)").expect("elem 1");
                // Roundtrip each one, so the batch is the production shape.
                let sum = SupportedFheCiphertexts::FheUint64(&a + &c);
                let t = sum.type_num();
                let bytes = sum.compress().expect("compress batch entry");
                SupportedFheCiphertexts::decompress_no_memcheck(t, &bytes)
                    .expect("decompress batch entry")
            })
            .collect();

        let serial: Vec<String> = batch
            .iter()
            .map(|ct| digest_of(&ct.squash_noise_and_serialize(true).expect("serial squash")))
            .collect();
        println!("[squash-determinism] serial batch of {BATCH} squashed");

        let mut mismatches = 0usize;
        for round in 0..4 {
            // Exactly the production preamble: seed every worker's
            // thread-local key, then squash the batch in parallel.
            rayon::broadcast(|_| tfhe::set_server_key(server_key_for_workers.clone()));
            let parallel: Vec<String> = batch
                .par_iter()
                .map(|ct| {
                    digest_of(
                        &ct.squash_noise_and_serialize(true)
                            .expect("parallel squash"),
                    )
                })
                .collect();
            for (index, (a, b)) in serial.iter().zip(parallel.iter()).enumerate() {
                if a != b {
                    mismatches += 1;
                    println!(
                        "[squash-determinism] MISMATCH round={round} index={index}\n  serial   {a}\n  parallel {b}"
                    );
                }
            }
            println!(
                "[squash-determinism] parallel round {round}: {} of {BATCH} match serial",
                serial
                    .iter()
                    .zip(parallel.iter())
                    .filter(|(a, b)| a == b)
                    .count()
            );
        }
        println!(
            "[squash-determinism] concurrent arm: {mismatches} mismatch(es) across 4 rounds of {BATCH}"
        );
        if mismatches > 0 {
            println!(
                "[squash-determinism] THE CONCURRENT PATH DIVERGES FROM THE SERIAL ONE for identical \
                 inputs. That is the production configuration, and it explains B-1 without any \
                 difference in input, key, format or build."
            );
        }
    }

    // Arm 4 — thread-count sensitivity, across PROCESSES.
    //
    // It cannot be done with an in-process rayon pool: `set_server_key` is
    // thread-local, so a custom pool's workers have no key and the squash
    // panics. Arms 1-3 work because the high-level `squash_noise()` reads the
    // key once on the calling thread and passes it explicitly into the
    // parallel interior -- which is also why the GLOBAL pool size reaches that
    // interior, and why separate processes are the right instrument. They are
    // also what production actually has: three containers, three pools.
    //
    // The input has to be byte-identical across those processes, and a fresh
    // compact encryption is randomized, so the ciphertext is written once and
    // reloaded. Set SQUASH_FIXTURE to a path:
    //
    //   for n in 1 2 4 8; do
    //     RAYON_NUM_THREADS=$n SQUASH_FIXTURE=/tmp/ct.bin \
    //       cargo test -p sns-worker --release squash_determinism -- --ignored --nocapture
    //   done
    //
    // First run writes the fixture, later runs reuse it; differing digests
    // across n then mean the squash depends on pool size.
    if let Ok(path) = std::env::var("SQUASH_FIXTURE") {
        let threads = rayon::current_num_threads();
        let ct = if std::path::Path::new(&path).exists() {
            let bytes = std::fs::read(&path).expect("read the squash fixture");
            let ct = SupportedFheCiphertexts::decompress_no_memcheck(ct_type, &bytes)
                .expect("decompress the squash fixture");
            println!(
                "[squash-determinism] reused fixture {path} ({} bytes)",
                bytes.len()
            );
            ct
        } else {
            std::fs::write(&path, &compressed).expect("write the squash fixture");
            println!(
                "[squash-determinism] wrote fixture {path} ({} bytes)",
                compressed.len()
            );
            SupportedFheCiphertexts::decompress_no_memcheck(ct_type, &compressed)
                .expect("decompress the fixture just written")
        };
        let bytes = ct
            .squash_noise_and_serialize(true)
            .expect("squash the fixture");
        println!(
            "[squash-determinism] FIXTURE rayon_threads={threads} digest={}",
            digest_of(&bytes)
        );
    }

    println!(
        "\n[squash-determinism] VERDICT: noisy={} distinct, trivial={} distinct, roundtrip={} distinct",
        noisy_counts.len(),
        trivial_counts.len(),
        roundtrip_counts.len()
    );

    if noisy_counts.len() > 1 {
        println!(
            "[squash-determinism] squash_noise is NON-DETERMINISTIC within a single process for a \
             fixed input. Nothing in this repository chooses the differing bytes; the next step is \
             a minimal repro against tfhe alone."
        );
    } else {
        println!(
            "[squash-determinism] one process is self-consistent, so the production divergence is \
             NOT per-call randomness. The repro must compare separate processes or hosts."
        );
    }

    // Deliberately not asserted. The purpose is to learn which of the two
    // explanations holds; failing the run would only hide the answer in a
    // panic message, and either outcome is informative.
}

/// Squashes a ciphertext taken straight out of a production database, under
/// that stack's own keyset, and reports the digest.
///
/// This is the experiment that names the culprit. Three operators recorded
/// three ct128 digests for one handle — two agreeing, one not — while their
/// inputs, key material, formats, binary and host were all shown identical.
/// Recomputing the value here says which of the recorded digests is the one a
/// correct squash produces, and therefore which operator's row is *wrong*
/// rather than merely different. That converts an unexplained disagreement
/// into a question about a single row: when was it written, and by what.
///
///   PROD_KEYSET=/path/xof_prod.bin  (the `compressed_xof_keyset` column)
///   PROD_CT=/path/divergent_ct.bin  (the `ciphertexts.ciphertext` bytes)
///   PROD_CT_TYPE=5                  (FheUint64)
///   EXPECT_A=<digest seen on the majority>
///   EXPECT_B=<digest seen on the odd operator>
#[test]
#[ignore = "needs a keyset and ciphertext extracted from a live database"]
fn squash_a_production_ciphertext() {
    let keyset_path = std::env::var("PROD_KEYSET").expect("PROD_KEYSET");
    let ct_path = std::env::var("PROD_CT").expect("PROD_CT");
    let ct_type: i16 = std::env::var("PROD_CT_TYPE")
        .unwrap_or_else(|_| "5".into())
        .parse()
        .expect("PROD_CT_TYPE");

    let keyset_bytes = std::fs::read(&keyset_path).expect("read the production keyset");
    println!(
        "[prod-squash] keyset {} ({} bytes)",
        keyset_path,
        keyset_bytes.len()
    );
    let keyset: CompressedXofKeySet =
        fhevm_engine_common::utils::safe_deserialize_key(&keyset_bytes)
            .expect("deserialize the production CompressedXofKeySet");
    let (_pk, server_key) = keyset
        .decompress()
        .expect("decompress the production keyset")
        .into_raw_parts();
    tfhe::set_server_key(server_key);

    let compressed = std::fs::read(&ct_path).expect("read the production ciphertext");
    println!(
        "[prod-squash] ciphertext {} ({} bytes)",
        ct_path,
        compressed.len()
    );

    // Exactly sns-worker's path: decompress the stored bytes, then squash with
    // compression enabled, then Keccak the serialized result.
    let ct = SupportedFheCiphertexts::decompress_no_memcheck(ct_type, &compressed)
        .expect("decompress the production ciphertext");
    let squashed = ct
        .squash_noise_and_serialize(true)
        .expect("squash the production ciphertext");
    let digest = digest_of(&squashed);
    println!("[prod-squash] recomputed ct128 digest = {digest}");

    // Twice, so a single odd answer cannot be mistaken for a stable one.
    let again = digest_of(
        &SupportedFheCiphertexts::decompress_no_memcheck(ct_type, &compressed)
            .expect("decompress again")
            .squash_noise_and_serialize(true)
            .expect("squash again"),
    );
    println!("[prod-squash] recomputed again        = {again}");
    println!(
        "[prod-squash] stable across two attempts: {}",
        if digest == again { "yes" } else { "NO" }
    );

    for (name, var) in [("majority", "EXPECT_A"), ("odd operator", "EXPECT_B")] {
        if let Ok(expected) = std::env::var(var) {
            println!(
                "[prod-squash] {name} recorded {} -> {}",
                &expected[..24.min(expected.len())],
                if expected == digest {
                    "MATCHES the recomputation"
                } else {
                    "does not match"
                }
            );
        }
    }
}

/// Squashes a production ciphertext BOTH ways — compressed and not — to see
/// which serialization each recorded digest corresponds to.
///
/// Production recorded two distinct ct128 digests for what is provably one
/// ciphertext value, on a single operator, with a deterministic squash. So the
/// two digests must come from two different serializations of the same squash,
/// and `enable_compression` is the flag that chooses between them.
#[test]
#[ignore = "needs a keyset and ciphertext extracted from a live database"]
fn squash_production_ciphertext_both_serializations() {
    let keyset_bytes = std::fs::read(std::env::var("PROD_KEYSET").expect("PROD_KEYSET"))
        .expect("read the production keyset");
    let keyset: CompressedXofKeySet =
        fhevm_engine_common::utils::safe_deserialize_key(&keyset_bytes)
            .expect("deserialize the production keyset");
    let (_pk, server_key) = keyset.decompress().expect("decompress").into_raw_parts();
    tfhe::set_server_key(server_key);

    let compressed_ct = std::fs::read(std::env::var("PROD_CT").expect("PROD_CT"))
        .expect("read the production ciphertext");
    let ct_type: i16 = std::env::var("PROD_CT_TYPE")
        .unwrap_or_else(|_| "5".into())
        .parse()
        .unwrap();

    for enable_compression in [true, false] {
        let ct = SupportedFheCiphertexts::decompress_no_memcheck(ct_type, &compressed_ct)
            .expect("decompress the production ciphertext");
        let bytes = ct
            .squash_noise_and_serialize(enable_compression)
            .expect("squash the production ciphertext");
        println!(
            "[prod-squash] enable_compression={enable_compression:<5} len={:<9} digest={}",
            bytes.len(),
            digest_of(&bytes)
        );
    }
}

/// Does decompressing the SAME compressed keyset twice yield the same key?
///
/// This is the gap every earlier arm shared: each decompressed the keyset once
/// per process and then squashed repeatedly, so all of them held the key
/// constant. Production does not — `decode_server_key` runs on every worker
/// start, and the deployed worker logged "Decompressing CompressedXofKeySet to
/// ServerKey" more than once across restarts.
///
/// If XOF expansion is not deterministic, two loads of one byte-identical
/// keyset give two different ServerKeys, and every squash performed under one
/// differs from every squash under the other. That would explain the whole of
/// B-1: exactly two ct128 digests for one ciphertext value, interleaved in
/// time, appearing on every operator independently, with the split determined
/// by which load was live when each handle was squashed.
#[test]
#[ignore = "needs a keyset extracted from a live database"]
fn keyset_decompression_determinism() {
    let keyset_bytes = std::fs::read(std::env::var("PROD_KEYSET").expect("PROD_KEYSET"))
        .expect("read the production keyset");
    let compressed_ct = std::fs::read(std::env::var("PROD_CT").expect("PROD_CT"))
        .expect("read the production ciphertext");
    let ct_type: i16 = std::env::var("PROD_CT_TYPE")
        .unwrap_or_else(|_| "5".into())
        .parse()
        .unwrap();

    let mut digests = Vec::new();
    for load in 0..2 {
        // A fresh deserialize AND a fresh decompress each time, which is what
        // a restarting worker does.
        let keyset: CompressedXofKeySet =
            fhevm_engine_common::utils::safe_deserialize_key(&keyset_bytes)
                .expect("deserialize the keyset");
        let (_pk, server_key) = keyset
            .decompress()
            .expect("decompress the keyset")
            .into_raw_parts();
        tfhe::set_server_key(server_key);

        let ct = SupportedFheCiphertexts::decompress_no_memcheck(ct_type, &compressed_ct)
            .expect("decompress the ciphertext");
        let digest = digest_of(&ct.squash_noise_and_serialize(true).expect("squash"));
        println!("[keyset-load] load {load}: squash digest = {digest}");
        digests.push(digest);
    }

    if digests[0] == digests[1] {
        println!(
            "[keyset-load] two independent loads agree — keyset decompression is deterministic"
        );
    } else {
        println!(
            "[keyset-load] TWO LOADS OF ONE BYTE-IDENTICAL KEYSET PRODUCE DIFFERENT SQUASH RESULTS.\n\
             [keyset-load] That is B-1's root cause: the key, not the ciphertext, is what differs.\n\
             [keyset-load] Every squash under one load disagrees with every squash under another,\n\
             [keyset-load] so operators diverge according to when each last restarted."
        );
    }
}

/// Squashes one ciphertext under two independent clones of the same
/// ServerKey, in one process, sequentially.
///
/// This isolates the clone from the concurrency. The concurrent arm reproduced
/// sns-worker's `rayon::broadcast` + `par_iter_mut` shape and matched serial
/// results, but every worker there received `keys.server_key.clone()` and the
/// arm never varied the clone independently of the pool. The hypothesis was
/// that a clone carries or seeds RNG state, since the two production ct128
/// objects for one handle differed in 99.3% of their bytes at identical length
/// -- the signature of a different random mask rather than a numeric
/// perturbation.
///
/// Falsified: two clones agree. The mask difference came from two *backends*
/// (CPU and CUDA), not two clones. Kept because "cloning the key does not
/// perturb the squash" is worth having pinned down.
#[test]
#[ignore = "needs a keyset and ciphertext extracted from a live database"]
fn squash_under_two_server_key_clones() {
    let keyset_bytes =
        std::fs::read(std::env::var("PROD_KEYSET").expect("PROD_KEYSET")).expect("read the keyset");
    let compressed_ct =
        std::fs::read(std::env::var("PROD_CT").expect("PROD_CT")).expect("read the ciphertext");
    let ct_type: i16 = std::env::var("PROD_CT_TYPE")
        .unwrap_or_else(|_| "5".into())
        .parse()
        .unwrap();

    let keyset: CompressedXofKeySet =
        fhevm_engine_common::utils::safe_deserialize_key(&keyset_bytes).expect("deserialize");
    let (_pk, server_key) = keyset.decompress().expect("decompress").into_raw_parts();

    let mut digests = Vec::new();
    for clone_index in 0..3 {
        // A fresh clone each round, exactly as process_tasks hands one to
        // every rayon worker.
        tfhe::set_server_key(server_key.clone());
        let ct = SupportedFheCiphertexts::decompress_no_memcheck(ct_type, &compressed_ct)
            .expect("decompress the ciphertext");
        let bytes = ct.squash_noise_and_serialize(true).expect("squash");
        let digest = digest_of(&bytes);
        println!(
            "[clone] clone {clone_index}: len={} digest={digest}",
            bytes.len()
        );
        digests.push(digest);
    }

    let distinct: std::collections::BTreeSet<&String> = digests.iter().collect();
    println!(
        "[clone] {} distinct result(s) across 3 independent clones",
        distinct.len()
    );
    if distinct.len() > 1 {
        println!(
            "[clone] CLONING THE SERVER KEY CHANGES THE SQUASH. Each rayon worker gets its own \\
             clone, so operators diverge according to which clone did the work."
        );
    } else {
        println!("[clone] clones are equivalent — the clone is not the variable");
    }
}
