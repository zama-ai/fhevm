use std::{process::Command, time::Duration};

#[test]
fn legacy_override_is_rejected_before_database_connection() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_sns_worker"))
        .args([
            "--database-url",
            "postgres://postgres:postgres@127.0.0.1:1/postgres",
            "--pg-notify-channel",
            "sns_worker_chan",
            "--bucket-name",
            "ct128",
        ])
        .env("FORCE_LEGACY_SERVER_KEY", "true")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("start SNS worker");
    for _ in 0..100 {
        if child.try_wait().expect("poll SNS worker").is_some() {
            let output = child.wait_with_output().expect("read SNS worker output");
            assert!(!output.status.success());
            let logs = format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(
                logs.contains("FORCE_LEGACY_SERVER_KEY=true is not supported"),
                "{logs}"
            );
            return;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    child.kill().expect("stop stuck SNS worker");
    child.wait().expect("reap SNS worker");
    panic!("SNS worker did not reject the legacy override at startup");
}
