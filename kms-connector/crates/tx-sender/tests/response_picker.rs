mod common;

use common::{insert_rand_public_decrypt_response, insert_rand_user_decrypt_response};
use connector_utils::tests::setup::TestInstanceBuilder;
use tx_sender::core::{DbKmsResponsePicker, KmsResponsePicker};

#[tokio::test]
async fn test_pick_public_decryption() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let mut response_picker = DbKmsResponsePicker::connect(test_instance.db().clone(), 1).await?;

    println!("Triggering Postgres notification with PublicDecryptionResponse insertion...");
    let inserted_response = insert_rand_public_decrypt_response(test_instance.db()).await?;

    println!("Picking PublicDecryptionResponse...");
    let responses = response_picker.pick_responses().await?;

    println!("Checking PublicDecryptionResponse data...");
    assert_eq!(responses[0], inserted_response);
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_pick_user_decryption() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let mut response_picker = DbKmsResponsePicker::connect(test_instance.db().clone(), 1).await?;

    println!("Triggering Postgres notification with UserDecryptionResponse insertion...");
    let inserted_response = insert_rand_user_decrypt_response(test_instance.db()).await?;
    println!("Picking UserDecryptionResponse...");
    let responses = response_picker.pick_responses().await?;

    println!("Checking UserDecryptionResponse data...");
    assert_eq!(responses[0], inserted_response);
    println!("Data OK!");
    Ok(())
}
