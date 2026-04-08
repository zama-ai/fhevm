//! Verify that UniFFI-generated Swift and Kotlin bindings compile.
//!
//! This test is automatically discovered by `uniffi::build_foreign_language_testcases!`.
//! It generates Swift/Kotlin source from the Rust UDL and compiles it,
//! catching type mismatches and missing exports at test time.

uniffi::build_foreign_language_testcases!(
    "src/lib.rs",
    // Test scripts would go here, e.g.:
    // "tests/bindings/test_keypair.swift",
    // "tests/bindings/test_keypair.kts",
);
