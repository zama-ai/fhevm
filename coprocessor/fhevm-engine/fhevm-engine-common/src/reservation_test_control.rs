//! Opt-in, expiring admission budget for isolated GPU reservation tests.
//! This rejects allocation admission, never allocates GPU memory or fabricates
//! an execution error: the ordinary reservation wait and deadline remain active.
use std::cell::RefCell;
use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

thread_local! {
    static TRANSACTION: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Synchronous partition execution owns this context; the guard also clears it
/// on unwind, so a reused blocking thread cannot inherit another transaction.
pub struct TransactionScope(Option<String>);
impl TransactionScope {
    pub fn enter(transaction: &[u8]) -> Self {
        Self(TRANSACTION.with(|current| current.replace(Some(hex::encode(transaction)))))
    }
}
impl Drop for TransactionScope {
    fn drop(&mut self) {
        TRANSACTION.with(|current| current.replace(self.0.take()));
    }
}

pub fn denied(amount: u64, gpu: usize) -> bool {
    let path = Path::new("/tmp").join(format!("fhevm-test-gpu-reservation-{}", std::process::id()));
    denied_at(
        &path,
        amount,
        gpu,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    )
}

fn denied_at(path: &Path, amount: u64, gpu: usize, now: u64) -> bool {
    let Ok(control) = std::fs::read_to_string(path) else {
        return false;
    };
    let mut fields = control.split_whitespace();
    let Some(deadline) = fields.next().and_then(|field| field.parse::<u64>().ok()) else {
        return false;
    };
    if let Some(transaction) = fields.next() {
        if fields.next().is_some()
            || transaction.len() != 64
            || !transaction.bytes().all(|byte| byte.is_ascii_hexdigit())
            || !TRANSACTION.with(|current| current.borrow().as_deref() == Some(transaction))
        {
            return false;
        }
    }
    // Refuse indefinite controls, expired controls and zero-byte admissions.
    if amount == 0 || deadline <= now || deadline > now.saturating_add(600) {
        return false;
    }
    // Acknowledgment comes only from an actual nonzero reservation admission.
    let _ = std::fs::write(
        path.with_extension("observed"),
        format!(
            "gpu={gpu} amount={amount} at={now} transaction={}\n",
            TRANSACTION.with(|current| current.borrow().clone().unwrap_or_default())
        ),
    );
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pressure_is_expiring_and_observed_only_for_real_admission() {
        let root =
            std::env::temp_dir().join(format!("reservation-control-test-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let path = root.join("control");
        assert!(!denied_at(&path, 7, 0, 100));
        for invalid in ["broken", "99", "100", "701"] {
            std::fs::write(&path, invalid).unwrap();
            assert!(!denied_at(&path, 7, 0, 100));
        }
        std::fs::write(&path, "120").unwrap();
        assert!(!denied_at(&path, 0, 0, 100));
        assert!(!path.with_extension("observed").exists());
        assert!(denied_at(&path, 7, 0, 100));
        assert_eq!(
            std::fs::read_to_string(path.with_extension("observed")).unwrap(),
            "gpu=0 amount=7 at=100 transaction=\n"
        );
        assert!(!denied_at(&path, 7, 0, 120));
        std::fs::write(&path, format!("120 {}", hex::encode([7u8; 32]))).unwrap();
        assert!(!denied_at(&path, 7, 0, 100));
        {
            let _scope = TransactionScope::enter(&[7u8; 32]);
            assert!(denied_at(&path, 7, 0, 100));
            {
                let _other = TransactionScope::enter(&[8u8; 32]);
                assert!(!denied_at(&path, 7, 0, 100));
            }
            assert!(denied_at(&path, 7, 0, 100));
        }
        assert!(!denied_at(&path, 7, 0, 100));
        std::fs::remove_file(&path).unwrap();
        assert!(!denied_at(&path, 7, 0, 101));
        std::fs::remove_dir_all(root).unwrap();
    }
}
