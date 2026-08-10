//! Off-chain `fhe_execute` encoding helpers for clients and tests. Never compiled into the SBF
//! build — enable the `encode` feature to use it.

use anchor_lang::prelude::Pubkey;

/// Builds the execution's interned 32-byte constant dictionary while a caller assembles its
/// steps (fhevm-internal#1853 W7). Interning deduplicates: repeated constants share one entry.
#[derive(Default)]
pub struct ExecutionDictionary(Vec<[u8; 32]>);

impl ExecutionDictionary {
    /// Interns `bytes`, returning its dictionary index.
    pub fn intern(&mut self, bytes: [u8; 32]) -> u8 {
        if let Some(index) = self.0.iter().position(|entry| *entry == bytes) {
            return u8::try_from(index).expect("execution dictionary fits u8");
        }
        let index = u8::try_from(self.0.len()).expect("execution dictionary fits u8");
        self.0.push(bytes);
        index
    }

    /// Interns a pubkey's 32 bytes.
    pub fn intern_key(&mut self, key: Pubkey) -> u8 {
        self.intern(key.to_bytes())
    }

    /// Interns each subject key, returning their dictionary indexes in order.
    pub fn intern_subjects(&mut self, subjects: impl IntoIterator<Item = Pubkey>) -> Vec<u8> {
        subjects
            .into_iter()
            .map(|subject| self.intern_key(subject))
            .collect()
    }

    /// The finished dictionary, in interning order — the `dictionary` field of `FheExecuteArgs`.
    pub fn into_entries(self) -> Vec<[u8; 32]> {
        self.0
    }
}
