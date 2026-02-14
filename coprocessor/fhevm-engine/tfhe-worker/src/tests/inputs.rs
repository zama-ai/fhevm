use crate::tests::event_helpers::{
    allow_handle, decrypt_handles, insert_trivial_encrypt, next_handle, setup_event_harness,
    wait_until_computed,
};

#[tokio::test]
async fn test_fhe_inputs() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let tx_id = next_handle();
    let mut tx = harness.listener_db.new_transaction().await?;

    let test_cases: &[(u64, i32, i16, &str)] = &[
        (0, 0, 0, "false"), // bool
        (1, 2, 2, "1"),     // uint8
        (2, 3, 3, "2"),     // uint16
        (3, 4, 4, "3"),     // uint32
        (4, 5, 5, "4"),     // uint64
        (5, 6, 6, "5"),     // uint128
        (7, 8, 8, "7"),     // uint256
        (8, 9, 9, "8"),     // ebytes64
        (9, 10, 10, "9"),   // ebytes128
        (10, 11, 11, "10"), // ebytes256
    ];

    let mut output_handles = Vec::with_capacity(test_cases.len());
    for &(value, to_type, _, _) in test_cases {
        let handle = next_handle();
        insert_trivial_encrypt(
            &harness.listener_db,
            &mut tx,
            tx_id,
            value,
            to_type,
            handle,
            false,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &handle).await?;
        output_handles.push(handle);
    }
    tx.commit().await?;

    wait_until_computed(&harness.app).await?;
    let decrypted = decrypt_handles(&harness.pool, &output_handles).await?;
    assert_eq!(decrypted.len(), test_cases.len());
    for (idx, (_, _, expected_type, expected_value)) in test_cases.iter().enumerate() {
        assert_eq!(decrypted[idx].output_type, *expected_type);
        assert_eq!(decrypted[idx].value, *expected_value);
    }

    Ok(())
}
