// TODO: hide behind feature flag if someone wants to launch that manually
#[cfg(not(feature = "ci"))]
#[cfg(test)]
pub mod test_utils {
    use nix::errno::Errno;
    use nix::sys::signal::{self, Signal};
    use nix::unistd::Pid;
    use std::fs;
    use std::path::Path;
    use std::process::{Child, Command};
    use std::sync::Mutex;
    use std::sync::Once;
    use std::time::Duration;
    use std::{env, thread};

    static INIT: Once = Once::new();
    static GATEWAY_PROCESSORS_MOCK_SERVICE: Mutex<Option<Child>> = Mutex::new(None);
    static RELAYER_MOCK_SERVICE: Mutex<Option<Child>> = Mutex::new(None);

    use ctor::{ctor, dtor};

    pub fn kill_gracefully(mut child_process: Child) -> Result<(), String> {
        let child_pid_raw = child_process.id();
        let child_pid = Pid::from_raw(child_pid_raw as i32);
        println!("Child process started with PID: {}", child_pid_raw);

        // 2. Try to send SIGINT for graceful shutdown
        println!("Sending SIGINT to child process (PID: {})...", child_pid);
        match signal::kill(child_pid, Signal::SIGINT) {
            Ok(_) => println!("SIGINT sent successfully."),
            Err(Errno::ESRCH) => {
                // ESRCH means "No such process"
                println!(
                    "Child process (PID: {}) already exited before SIGINT.",
                    child_pid
                );
                // Process is already gone, no need to do anything else
                return Ok(());
            }
            Err(e) => {
                eprintln!(
                    "Error sending SIGINT to PID {}: {}. Attempting SIGKILL.",
                    child_pid, e
                );
                // Proceed to SIGKILL if sending SIGINT failed for other reasons
            }
        }

        // 3. Wait for a timeout
        let timeout = Duration::from_secs(5); // Wait 5 seconds for graceful shutdown
        let start_time = std::time::Instant::now();

        loop {
            match child_process.try_wait() {
                Ok(Some(status)) => {
                    // Process has exited
                    println!(
                        "Child process (PID: {}) exited gracefully with status: {}",
                        child_pid_raw, status
                    );
                    return Ok(()); // Successfully terminated
                }
                Ok(None) => {
                    // Process is still running
                    if start_time.elapsed() > timeout {
                        println!(
                            "Child process (PID: {}) did not exit after SIGINT and timeout.",
                            child_pid_raw
                        );
                        break; // Timeout expired, proceed to SIGKILL
                    }
                    // Wait a bit before checking again
                    thread::sleep(Duration::from_millis(100));
                }
                Err(e) => {
                    // Error trying to wait (e.g., permissions, or if the PID was reused quickly, though unlikely here)
                    eprintln!("Error waiting for child process (PID: {}): {}. Assuming it's gone or needs SIGKILL.", child_pid_raw, e);
                    // It's safer to try SIGKILL if unsure
                    break;
                }
            }
        }

        // 4. If timeout expired and process is still running (or if SIGINT send failed), send SIGKILL
        println!(
            "Sending SIGKILL to child process (PID: {})...",
            child_pid_raw
        );
        match signal::kill(child_pid, Signal::SIGKILL) {
            Ok(_) => println!("SIGKILL sent successfully to PID {}.", child_pid_raw),
            Err(Errno::ESRCH) => {
                // ESRCH means "No such process"
                println!(
                    "Child process (PID: {}) already exited before SIGKILL.",
                    child_pid_raw
                );
            }
            Err(e) => {
                // This is problematic, as SIGKILL should generally work if the process exists and you have permissions
                eprintln!("Error sending SIGKILL to PID {}: {}. The process might be unkillable or already gone.", child_pid_raw, e);
                // You might still want to wait to see if it terminates despite the error
            }
        }

        // 5. Wait for the child to exit after SIGKILL (optional, but good practice)
        //    If SIGKILL was sent, the process should terminate almost immediately.
        //    `child_process.wait()` will clean up the zombie process.
        match child_process.wait() {
            Ok(status) => println!(
                "Child process (PID: {}) terminated with status after SIGKILL: {}",
                child_pid_raw, status
            ),
            Err(e) => eprintln!(
                "Error waiting for child process (PID: {}) after SIGKILL: {}",
                child_pid_raw, e
            ),
        }
        Ok(())
    }

    /// Setup function that starts nodes and runs mock and relayer before testing
    #[ctor]
    pub fn setup() {
        // TODO: should we init tracing here?
        // To get proper tracing logs from the library if we want to

        INIT.call_once(|| {
            // Start docker compose nodes + smart-contracts
            let project_root = env::var("CARGO_MANIFEST_DIR")
                .expect("CARGO_MANIFEST_DIR environment variable not set");

            let mnemonic =
                "coyote sketch defense hover finger envelope celery urge panther venue verb cheese";

            let docker_compose_command = Some(
                Command::new("docker")
                    .current_dir(&project_root)
                    .env("MNEMONIC", mnemonic)
                    .args([
                        "compose",
                        "-f",
                        "./mock/docker-compose.yaml",
                        "up",
                        "-d",
                        "--wait",
                    ])
                    .spawn()
                    .expect("Failed to start docker compose"),
            );
            if let Some(mut docker_compose_exit) = docker_compose_command {
                let ecode = docker_compose_exit
                    .wait()
                    .expect("failed to up docker compose");
                assert!(ecode.success());
            }

            // Remove config/local.yaml to avoid interaction with script
            let path = "config/local.yaml";
            if Path::new(project_root.as_str()).join(path).exists() {
                match fs::remove_file(path) {
                    Ok(()) => println!("File deleted successfully"),
                    Err(e) => println!("Error deleting file: {}", e),
                }
            }
            // Config setup
            let mut config_setup = Command::new("./setup-config.sh")
                .current_dir(&project_root)
                .spawn()
                .expect("Failed to setup-config");
            let ecode = config_setup.wait().expect("failed to wait on config-setup");
            assert!(ecode.success());

            // We build services before launching them to avoid not knowing how long it would take
            // to start the services. Indeed if the binaries are already built then `cargo run`
            // should start the services pretty quickly
            let mut gateway_mock_build = Command::new("cargo")
                .args(["build", "--bin", "gateway-processors-mock"])
                .current_dir(&project_root)
                .spawn()
                .expect("Failed to build gateway mock");
            let ecode = gateway_mock_build
                .wait()
                .expect("failed to wait on config-setup");
            assert!(ecode.success());

            let mut relayer_mock_build = Command::new("cargo")
                .args(["build", "--bin", "fhevm-relayer"])
                .current_dir(&project_root)
                .spawn()
                .expect("Failed to build relayer");
            let ecode = relayer_mock_build
                .wait()
                .expect("failed to wait on config-setup");
            assert!(ecode.success());

            // Start the mock service
            println!("Starting mock service...");

            // Run gateway-processors-mock
            // cargo run --bin gateway-processors-mock
            let mut mock_service_changer = GATEWAY_PROCESSORS_MOCK_SERVICE.lock().unwrap();
            *mock_service_changer = Some(
                Command::new("cargo")
                    .args(["run", "--bin", "gateway-processors-mock"])
                    .env(
                        "RUST_LOG", // NOTE: maybe it would be more appropriate to have a
                        // specific env-var instead of inheriting from `RUST_LOG`
                        env::var("RUST_LOG").unwrap_or("info,fhevm_relayer=debug".to_string()),
                    )
                    .current_dir(&project_root)
                    .spawn()
                    .expect("Failed to start gateway-processors-mock"),
            );
            std::mem::drop(mock_service_changer);

            // NOTE: Instead of relying on this command to launch the relayer
            // we should probably create an instance of the relayer in the test itself
            // One potential issue of doing that though is using test parallelism.
            // Because we'll need to make sure that there is no conflict between tests
            // (i.e. make sure that the port used is different for example)

            // Run fhevm-relayer
            // APP_TRANSACTION__RETRY__MOCK_MODE=true cargo run --bin fhevm-relayer
            let mut mock_service_changer = RELAYER_MOCK_SERVICE.lock().unwrap();
            *mock_service_changer = Some(
                Command::new("cargo")
                    .args(["run", "--bin", "fhevm-relayer"])
                    .env("APP_TRANSACTION__RETRY__MOCK_MODE", "true")
                    .env(
                        "RUST_LOG",
                        env::var("RUST_LOG").unwrap_or("info,fhevm_relayer=debug".to_string()),
                    )
                    .current_dir(&project_root)
                    .spawn()
                    .expect("Failed to start relayer-mock"),
            );
            std::mem::drop(mock_service_changer);

            // Give the services time to start
            thread::sleep(Duration::from_secs(3));

            println!("Test environment is ready!");
        });
    }

    /// Teardown function that cleans up resources after tests finish
    #[dtor]
    pub fn teardown() {
        // Use mutex to kill long-running processes

        let mut relayer_service_changer = RELAYER_MOCK_SERVICE.lock().unwrap();
        if let Some(mut child) = relayer_service_changer.take() {
            kill_gracefully(child);
        }

        let mut mock_service_changer = GATEWAY_PROCESSORS_MOCK_SERVICE.lock().unwrap();
        if let Some(mut child) = mock_service_changer.take() {
            kill_gracefully(child);
        }

        // Shutdown docker compose
        let project_root = env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR environment variable not set");
        let docker_compose_command = Some(
            Command::new("docker")
                .current_dir(&project_root)
                .args([
                    "compose",
                    "-f",
                    "./mock/docker-compose.yaml",
                    "down",
                    "--volumes",
                    "--remove-orphans",
                ])
                .spawn()
                .expect("Failed to shutdown docker compose"),
        );
        if let Some(mut docker_compose_exit) = docker_compose_command {
            let ecode = docker_compose_exit
                .wait()
                .expect("failed to shutdown docker compose");
            assert!(ecode.success());
        }
    }
}
