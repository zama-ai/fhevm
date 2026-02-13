use crate::tests::event_helpers::{
    allow_handle, decrypt_handles, insert_trivial_encrypt, next_handle, setup_event_harness,
    wait_until_computed,
};

#[tokio::test]
async fn test_fhe_binary_operands() -> Result<(), Box<dyn std::error::Error>> {
    super::operators_from_events::test_fhe_binary_operands_events()
}

#[tokio::test]
async fn test_fhe_unary_operands() -> Result<(), Box<dyn std::error::Error>> {
    super::operators_from_events::test_fhe_unary_operands_events()
}

#[tokio::test]
async fn test_fhe_casts() -> Result<(), Box<dyn std::error::Error>> {
    super::operators_from_events::test_fhe_cast_events()
}

#[tokio::test]
async fn test_op_trivial_encrypt() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let tx_id = next_handle();
    let mut tx = harness.listener_db.new_transaction().await?;

    let output = next_handle();
    insert_trivial_encrypt(&harness.listener_db, &mut tx, tx_id, 123, 5, output, false).await?;
    allow_handle(&harness.listener_db, &mut tx, &output).await?;
    tx.commit().await?;

    wait_until_computed(&harness.app).await?;
    let decrypted = decrypt_handles(&harness.pool, &[output]).await?;
    assert_eq!(decrypted.len(), 1);
    assert_eq!(decrypted[0].output_type, 5);
    assert_eq!(decrypted[0].value, "123");

    Ok(())
}

#[tokio::test]
async fn test_fhe_if_then_else() -> Result<(), Box<dyn std::error::Error>> {
    super::operators_from_events::test_fhe_if_then_else_events()
}
