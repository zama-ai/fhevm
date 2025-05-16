// TODO: hide behind feature flag if someone wants to launch that manually
#[cfg(not(feature = "ci"))]
#[cfg(test)]
pub mod test_utils {
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

    /// Setup function that starts nodes and runs mock and relayer before testing
    #[ctor]
    pub fn setup() {
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

        let mut mock_service_changer = RELAYER_MOCK_SERVICE.lock().unwrap();
        let mut relayer_kill_result: Option<_> = None;
        if let Some(mut mock_service) = mock_service_changer.take() {
            relayer_kill_result = Some(mock_service.kill());
        } else {
            println!("Relayer mutex is empty");
        }

        let mut mock_service_changer = GATEWAY_PROCESSORS_MOCK_SERVICE.lock().unwrap();
        let mut gateway_kill_result: Option<_> = None;
        if let Some(mut mock_service) = mock_service_changer.take() {
            gateway_kill_result = Some(mock_service.kill());
        } else {
            println!("Gateway mutex is empty");
        }
        println!(
            "Relayer-kill-output: {:?}\nGateway-kill-output{:?}",
            relayer_kill_result, gateway_kill_result
        );

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
