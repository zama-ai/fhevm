mod common;

use common::{insert_rand_public_decrypt_response, insert_rand_user_decrypt_response};
use connector_tests::setup::test_instance_with_db_only;
use tx_sender::core::{DbKmsResponsePicker, KmsResponsePicker};

#[tokio::test]
async fn test_pick_public_decryption() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_only().await?;

    let mut response_picker = DbKmsResponsePicker::connect(test_instance.db.clone()).await?;

    println!("Triggering Postgres notification with PublicDecryptionResponse insertion...");
    let inserted_response = insert_rand_public_decrypt_response(&test_instance.db).await?;

    println!("Picking PublicDecryptionResponse...");
    let response = response_picker.pick_response().await?;

    println!("Checking PublicDecryptionResponse data...");
    assert_eq!(response, inserted_response,);
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_pick_user_decryption() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_only().await?;

    let mut response_picker = DbKmsResponsePicker::connect(test_instance.db.clone()).await?;

    println!("Triggering Postgres notification with UserDecryptionResponse insertion...");
    let inserted_response = insert_rand_user_decrypt_response(&test_instance.db).await?;
    println!("Picking UserDecryptionResponse...");
    let response = response_picker.pick_response().await?;

    println!("Checking UserDecryptionResponse data...");
    assert_eq!(response, inserted_response);
    println!("Data OK!");
    Ok(())
}
