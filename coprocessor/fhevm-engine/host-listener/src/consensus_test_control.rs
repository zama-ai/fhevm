//! File-control access for opt-in tests on runtime images without a shell.
use std::{io, path::Path};

pub fn execute(action: &str) -> io::Result<String> {
    at(Path::new("/tmp/fhevm-test-broker-ack"), action)
}
/// Called before ordinary argument parsing in each listener role.
pub fn key_download_cli() -> io::Result<bool> {
    if std::env::args().nth(1).as_deref() != Some("--key-download-control") {
        return Ok(false);
    }
    let args: Vec<_> = std::env::args().skip(2).collect();
    if args.len() != 1 {
        return Err(io::Error::other("one key control action required"));
    }
    println!(
        "{}",
        at(Path::new("/tmp/fhevm-test-key-download"), &args[0])?
    );
    Ok(true)
}

fn at(path: &Path, action: &str) -> io::Result<String> {
    match action {
        "empty" => {
            for file in [
                path.to_owned(),
                path.with_extension("observed"),
                path.with_extension("redelivered"),
            ] {
                if file.try_exists()? {
                    return Err(io::Error::other(
                        "prior broker control needs recovery",
                    ));
                }
            }
            Ok("empty".to_owned())
        }
        "observed" | "redelivered" => {
            std::fs::read_to_string(path.with_extension(action))
        }
        "clear" => {
            let mut error = None;
            for file in [
                path.to_owned(),
                path.with_extension("observed"),
                path.with_extension("redelivered"),
            ] {
                if let Err(failure) = std::fs::remove_file(file) {
                    if failure.kind() != io::ErrorKind::NotFound {
                        error = Some(failure);
                    }
                }
            }
            if let Some(error) = error {
                Err(error)
            } else {
                Ok("cleared".to_owned())
            }
        }
        _ => Err(io::Error::other("unknown private control action")),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn private_control_never_accepts_a_caller_path() {
        let root = std::env::temp_dir()
            .join(format!("private-ack-control-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let control = root.join("control");
        let unrelated = root.join("unrelated");
        std::fs::write(&unrelated, "retain").unwrap();
        assert_eq!(at(&control, "empty").unwrap(), "empty");
        for path in [
            control.clone(),
            control.with_extension("observed"),
            control.with_extension("redelivered"),
        ] {
            std::fs::write(path, "owned").unwrap();
        }
        assert!(at(&control, "empty").is_err());
        assert_eq!(at(&control, "observed").unwrap(), "owned");
        assert!(at(&control, "../../unrelated").is_err());
        at(&control, "clear").unwrap();
        at(&control, "empty").unwrap();
        assert_eq!(std::fs::read_to_string(unrelated).unwrap(), "retain");
        std::fs::remove_dir_all(root).unwrap();
    }
}
