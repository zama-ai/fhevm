mod common;

use common::{insert_rand_public_decrypt_response, insert_rand_user_decrypt_response};
use connector_utils::tests::setup::shared::{clean_test_instance, run_with_shared_db_setup};
use ctor::dtor;
use rstest::rstest;
use serial_test::serial;
use std::time::Duration;
use tracing::info;
use tx_sender::core::{DbKmsResponsePicker, KmsResponsePicker};

#[dtor]
fn on_shutdown() {
    clean_test_instance();
}

#[rstest]
#[timeout(Duration::from_secs(10))]
#[serial]
fn test_pick_public_decryption() -> anyhow::Result<()> {
    run_with_shared_db_setup(async |test_instance| {
        let mut response_picker =
            DbKmsResponsePicker::connect(test_instance.db().clone(), 1).await?;

        info!("Triggering Postgres notification with PublicDecryptionResponse insertion...");
        let inserted_response = insert_rand_public_decrypt_response(test_instance.db()).await?;

        info!("Picking PublicDecryptionResponse...");
        let responses = response_picker.pick_responses().await?;

        info!("Checking PublicDecryptionResponse data...");
        assert_eq!(responses[0], inserted_response);
        info!("Data OK!");
        Ok(())
    })
}

#[rstest]
#[timeout(Duration::from_secs(10))]
#[serial]
fn test_pick_user_decryption() -> anyhow::Result<()> {
    run_with_shared_db_setup(async |test_instance| {
        let mut response_picker =
            DbKmsResponsePicker::connect(test_instance.db().clone(), 1).await?;

        info!("Triggering Postgres notification with UserDecryptionResponse insertion...");
        let inserted_response = insert_rand_user_decrypt_response(test_instance.db()).await?;
        info!("Picking UserDecryptionResponse...");
        let responses = response_picker.pick_responses().await?;

        info!("Checking UserDecryptionResponse data...");
        assert_eq!(responses[0], inserted_response);
        info!("Data OK!");
        Ok(())
    })
}
