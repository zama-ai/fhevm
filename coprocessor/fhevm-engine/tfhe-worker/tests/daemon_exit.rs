//! Regression coverage for the tfhe-worker's exit status on a fatal runtime
//! failure (inventory case REG-02-TFHE-DAEMON-EXIT).
//!
//! The defect: `start_runtime_inner` logged the runtime error and returned
//! normally, so the process exited 0. A supervisor can only act on the exit
//! status, and `restart: on-failure` is defined to ignore 0 -- so a failed
//! daemon looked like a clean shutdown and was never restarted. The failure
//! matrix's database cells exposed it: all three tfhe-workers stayed `exited`
//! while every other worker recovered.
//!
//! Asserting this needs a SUBPROCESS. The exit status is the contract, and a
//! test that calls the function in-process cannot observe it -- if the fix is
//! reached in-process it terminates the test runner instead, which is why the
//! `close_recv` path must be exempt and why that exemption is asserted here
//! too.
//!
//!   cargo test -p tfhe-worker --test daemon_exit
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// A fatal failure must terminate the daemon promptly, not after a retry
/// campaign: the supervisor cannot restart a process that is still running.
const EXIT_BUDGET: Duration = Duration::from_secs(120);

/// A database URL that resolves but refuses every connection, so the failure is
/// deterministic and local: port 1 on loopback is not listening, and no name
/// resolution or outbound network is involved.
const UNREACHABLE_DATABASE: &str = "postgres://unused:unused@127.0.0.1:1/unused";

fn daemon() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_tfhe_worker"));
    command
        .arg("--run-bg-worker")
        .arg(format!("--database-url={UNREACHABLE_DATABASE}"))
        // Keep the process small and its ports unused, so a developer running
        // the suite locally does not have this bind over something.
        .arg("--tokio-threads=1")
        .arg("--coprocessor-fhe-threads=1")
        .arg("--pg-pool-max-connections=1")
        .arg("--health-check-port=0")
        .arg("--metrics-addr=127.0.0.1:0")
        .arg("--log-level=error")
        .env("RUST_LOG", "error")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
}

/// Waits for a child, bounded, and kills it if it outlives the budget.
///
/// A daemon that hangs is the failure this case is about just as much as one
/// that exits 0: either way the supervisor has nothing to act on. So the
/// timeout is a failure rather than an inconclusive result.
fn wait_bounded(mut child: std::process::Child, budget: Duration) -> std::process::Output {
    let deadline = Instant::now() + budget;
    loop {
        match child.try_wait().expect("the child must be waitable") {
            Some(_) => return child.wait_with_output().expect("read daemon output"),
            None => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    panic!(
                        "the daemon was still running {}s after a fatal runtime failure; a supervisor \
                         can only act on an exit status",
                        budget.as_secs()
                    );
                }
                std::thread::sleep(Duration::from_millis(200));
            }
        }
    }
}

#[test]
fn a_fatal_runtime_failure_exits_non_zero_within_the_budget() {
    let child = daemon()
        .spawn()
        .expect("the daemon binary must be spawnable");
    let started = Instant::now();
    let output = wait_bounded(child, EXIT_BUDGET);
    let status = output.status;
    let logs = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        logs.contains("Runtime error") && logs.contains("pool timed out"),
        "must exercise the database runtime failure: {logs}"
    );
    let elapsed = started.elapsed();

    assert!(
        !status.success(),
        "the daemon exited {status:?} after {elapsed:?} on an unreachable database. Exit 0 is \
         defined as a clean shutdown, so `restart: on-failure` ignores it and the worker is never \
         restarted -- which is the defect this case pins"
    );
    assert_eq!(
        status.code(),
        Some(1),
        "a fatal runtime failure must exit 1 (it exited {status:?} after {elapsed:?}); a signal \
         death would mean something killed it rather than it reporting its own failure"
    );
    eprintln!("daemon exited {status:?} after {elapsed:?}");
}

/// The in-process path must NOT terminate its host process.
///
/// `start_runtime` is called with a close channel by the crate's own tests and
/// by the benchmark harness, which own the process themselves. If the
/// exit-for-restart path were reached there, it would take the caller down with
/// it -- and this test is the caller, so a regression fails the suite by killing
/// the test binary rather than by an assertion.
///
/// It is also the "voluntary shutdown remains successful" half: the same call
/// returns normally when the close channel fires.
#[test]
fn the_in_process_close_channel_path_does_not_terminate_its_caller() {
    use clap::Parser;

    let args = tfhe_worker::daemon_cli::Args::parse_from([
        "tfhe_worker",
        "--run-bg-worker",
        &format!("--database-url={UNREACHABLE_DATABASE}"),
        "--tokio-threads=1",
        "--coprocessor-fhe-threads=1",
        "--pg-pool-max-connections=1",
        "--health-check-port=0",
        "--metrics-addr=127.0.0.1:0",
        "--log-level=error",
    ]);

    // A failing runtime, entered through the in-process path.
    let (_close_send, close_recv) = tokio::sync::watch::channel(false);
    let (done_tx, done_rx) = std::sync::mpsc::channel();
    let failing = std::thread::spawn(move || {
        tfhe_worker::start_runtime(args, Some(close_recv));
        done_tx.send(()).unwrap();
    });
    done_rx
        .recv_timeout(EXIT_BUDGET)
        .expect("in-process failure must return within budget");
    failing
        .join()
        .expect("the in-process path must return rather than exiting the process");

    // Reaching this line at all is the assertion: `std::process::exit(1)` in
    // the failing branch would have taken this test binary with it.
    eprintln!("in-process failure returned to its caller; the test binary is still alive");

    // And a voluntary close returns too.
    let args = tfhe_worker::daemon_cli::Args::parse_from([
        "tfhe_worker",
        "--run-bg-worker",
        &format!("--database-url={UNREACHABLE_DATABASE}"),
        "--tokio-threads=1",
        "--coprocessor-fhe-threads=1",
        "--pg-pool-max-connections=1",
        "--health-check-port=0",
        "--metrics-addr=127.0.0.1:0",
        "--log-level=error",
    ]);
    let (close_send, close_recv) = tokio::sync::watch::channel(false);
    let (done_tx, done_rx) = std::sync::mpsc::channel();
    let voluntary = std::thread::spawn(move || {
        tfhe_worker::start_runtime(args, Some(close_recv));
        done_tx.send(()).unwrap();
    });
    // The runtime may fail before the close arrives; either way the call must
    // return without terminating this process, which is what is being asserted.
    let _ = close_send.send(true);
    done_rx
        .recv_timeout(Duration::from_secs(5))
        .expect("voluntary shutdown must return promptly");
    voluntary
        .join()
        .expect("a voluntary shutdown must return rather than exiting the process");
}
