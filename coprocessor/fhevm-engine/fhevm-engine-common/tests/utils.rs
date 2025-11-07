use fhevm_engine_common::utils::DatabaseURL;

#[tokio::test]
async fn mask_database_url() {
    let db_url: DatabaseURL = "postgres://postgres:mypassword@localhost:5432/coprocessor".into();

    let debug_fmt = format!("{:?}", db_url);
    assert!(!debug_fmt.contains("mypassword"));

    let display_fmt = format!("{}", db_url);
    assert!(!display_fmt.contains("mypassword"));
    println!("DatabaseURL: {}", db_url);

    let db_url: DatabaseURL = DatabaseURL::new_with_app_name(
        "postgres://user:secret@dbhost:5432/mydb?sslmode=require",
        "tfhe-worker",
    );

    assert_eq!(
        db_url.as_str(),
        "postgres://user:secret@dbhost:5432/mydb?sslmode=require&application_name=tfhe-worker"
    );

    let db_url: DatabaseURL =
        DatabaseURL::new_with_app_name("postgres://user:secret@dbhost:5432/mydb", "tfhe-worker");

    assert_eq!(
        db_url.as_str(),
        "postgres://user:secret@dbhost:5432/mydb?application_name=tfhe-worker"
    );

    println!("DatabaseURL: {}", db_url);

    let db_url: DatabaseURL =
        DatabaseURL::new_with_app_name("postgres://user:secret@dbhost:5432/mydb", " ");

    assert_eq!(db_url.as_str(), "postgres://user:secret@dbhost:5432/mydb");
}
