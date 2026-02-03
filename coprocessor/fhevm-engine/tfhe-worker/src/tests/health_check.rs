use crate::tests::utils::setup_test_app;
use serial_test::serial;
use test_harness::health_check;
use tokio::process::Command;

#[tokio::test]
#[serial(db)]
async fn test_health_check() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    eprintln!("App started");
    assert!(health_check::wait_alive(app.health_url(), 10, 1).await);
    assert!(health_check::wait_healthy(app.health_url(), 10, 1).await);
    tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
    assert!(health_check::wait_alive(app.health_url(), 10, 1).await);
    assert!(health_check::wait_healthy(app.health_url(), 10, 1).await);
    eprintln!("Pausing database");
    let db_id = app
        .db_docker_id()
        .expect("Database Docker ID should be set");
    Command::new("docker").args(["pause", &db_id]).spawn()?;
    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
    assert!(!health_check::wait_healthy(app.health_url(), 10, 1).await);
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    eprintln!("Unpausing database");
    Command::new("docker").args(["unpause", &db_id]).spawn()?;
    assert!(health_check::wait_healthy(app.health_url(), 10, 1).await);
    Ok(())
}
