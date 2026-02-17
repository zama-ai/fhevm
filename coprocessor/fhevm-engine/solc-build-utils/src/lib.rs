use foundry_compilers::solc::Solc;
use fs4::fs_std::FileExt;
use semver::Version;
use std::fs::OpenOptions;

const SOLC_INSTALL_LOCK_FILE: &str = "fhevm-solc-install.lock";

pub fn find_or_install_solc_locked(version: &Version) -> Solc {
    let lock_path = std::env::temp_dir().join(SOLC_INSTALL_LOCK_FILE);
    let lock_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .unwrap_or_else(|err| {
            panic!(
                "Failed to open solc install lock file {}: {err}",
                lock_path.display()
            );
        });

    lock_file.lock_exclusive().unwrap_or_else(|err| {
        panic!(
            "Failed to acquire solc install lock {}: {err}",
            lock_path.display()
        );
    });

    Solc::find_or_install(version)
        .unwrap_or_else(|err| panic!("Failed to find or install solc {version}: {err:?}"))
}
