use crate::tests::event_helpers::{
    allow_handle, decrypt_handles, insert_event, insert_trivial_encrypt, next_handle, scalar_flag,
    scalar_u128_handle, setup_event_harness, wait_until_computed, zero_address,
};
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use serial_test::serial;

fn sample_count(default_count: usize) -> usize {
    std::env::var("FHEVM_TEST_NUM_SAMPLES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(default_count)
}

#[tokio::test]
#[serial(db)]
async fn schedule_erc20_whitepaper() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let num_samples = sample_count(7);
    let mut tx = harness.listener_db.new_transaction().await?;
    let mut output_handles = Vec::with_capacity(num_samples * 5);
    let caller = zero_address();

    for _ in 0..num_samples {
        let tx_id = next_handle();
        let bals = next_handle();
        let trxa = next_handle();
        let bald = next_handle();
        insert_trivial_encrypt(&harness.listener_db, &mut tx, tx_id, 100, 5, bals, false).await?;
        insert_trivial_encrypt(&harness.listener_db, &mut tx, tx_id, 10, 5, trxa, false).await?;
        insert_trivial_encrypt(&harness.listener_db, &mut tx, tx_id, 20, 5, bald, false).await?;

        let has_funds = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheGe(TfheContract::FheGe {
                caller,
                lhs: bals,
                rhs: trxa,
                scalarByte: scalar_flag(false),
                result: has_funds,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &has_funds).await?;
        output_handles.push(has_funds);

        let new_to_target = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller,
                lhs: bald,
                rhs: trxa,
                scalarByte: scalar_flag(false),
                result: new_to_target,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &new_to_target).await?;
        output_handles.push(new_to_target);

        let new_to = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                caller,
                control: has_funds,
                ifTrue: new_to_target,
                ifFalse: bald,
                result: new_to,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &new_to).await?;
        output_handles.push(new_to);

        let new_from_target = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheSub(TfheContract::FheSub {
                caller,
                lhs: bals,
                rhs: trxa,
                scalarByte: scalar_flag(false),
                result: new_from_target,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &new_from_target).await?;
        output_handles.push(new_from_target);

        let new_from = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheIfThenElse(TfheContract::FheIfThenElse {
                caller,
                control: has_funds,
                ifTrue: new_from_target,
                ifFalse: bals,
                result: new_from,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &new_from).await?;
        output_handles.push(new_from);
    }
    tx.commit().await?;

    wait_until_computed(&harness.app).await?;
    let resp = decrypt_handles(&harness.pool, &output_handles).await?;
    assert_eq!(resp.len(), output_handles.len());
    for (i, r) in resp.iter().enumerate() {
        match r.value.as_str() {
            "true" if i % 5 == 0 => (),
            "30" if i % 5 == 1 => (),
            "30" if i % 5 == 2 => (),
            "90" if i % 5 == 3 => (),
            "90" if i % 5 == 4 => (),
            s => panic!("unexpected result: {s} for output {i}"),
        }
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn schedule_erc20_no_cmux() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let num_samples = sample_count(7);
    let mut tx = harness.listener_db.new_transaction().await?;
    let mut output_handles = Vec::with_capacity(num_samples * 5);
    let caller = zero_address();

    for _ in 0..num_samples {
        let tx_id = next_handle();
        let bals = next_handle();
        let trxa = next_handle();
        let bald = next_handle();
        insert_trivial_encrypt(&harness.listener_db, &mut tx, tx_id, 100, 5, bals, false).await?;
        insert_trivial_encrypt(&harness.listener_db, &mut tx, tx_id, 10, 5, trxa, false).await?;
        insert_trivial_encrypt(&harness.listener_db, &mut tx, tx_id, 20, 5, bald, false).await?;

        let has_funds = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheGe(TfheContract::FheGe {
                caller,
                lhs: bals,
                rhs: trxa,
                scalarByte: scalar_flag(false),
                result: has_funds,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &has_funds).await?;
        output_handles.push(has_funds);

        let cast_funds = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::Cast(TfheContract::Cast {
                caller,
                ct: has_funds,
                toType: crate::tests::event_helpers::to_ty(5),
                result: cast_funds,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &cast_funds).await?;
        output_handles.push(cast_funds);

        let selected = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheMul(TfheContract::FheMul {
                caller,
                lhs: trxa,
                rhs: cast_funds,
                scalarByte: scalar_flag(false),
                result: selected,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &selected).await?;
        output_handles.push(selected);

        let new_to = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller,
                lhs: bald,
                rhs: selected,
                scalarByte: scalar_flag(false),
                result: new_to,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &new_to).await?;
        output_handles.push(new_to);

        let new_from = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheSub(TfheContract::FheSub {
                caller,
                lhs: bals,
                rhs: selected,
                scalarByte: scalar_flag(false),
                result: new_from,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &new_from).await?;
        output_handles.push(new_from);
    }
    tx.commit().await?;

    wait_until_computed(&harness.app).await?;
    let resp = decrypt_handles(&harness.pool, &output_handles).await?;
    assert_eq!(resp.len(), output_handles.len());
    for (i, r) in resp.iter().enumerate() {
        match r.value.as_str() {
            "true" if i % 5 == 0 => (),
            "1" if i % 5 == 1 => (),
            "10" if i % 5 == 2 => (),
            "30" if i % 5 == 3 => (),
            "90" if i % 5 == 4 => (),
            s => panic!("unexpected result: {s} for output {i}"),
        }
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn schedule_dependent_erc20_no_cmux() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let num_samples = sample_count(7);
    let mut tx = harness.listener_db.new_transaction().await?;
    let mut output_handles = Vec::with_capacity(num_samples * 5);
    let caller = zero_address();

    let init_tx = next_handle();
    let mut bald = next_handle();
    insert_trivial_encrypt(&harness.listener_db, &mut tx, init_tx, 20, 5, bald, false).await?;

    for _ in 0..num_samples {
        let tx_id = next_handle();
        let bals = next_handle();
        let trxa = next_handle();
        insert_trivial_encrypt(&harness.listener_db, &mut tx, tx_id, 100, 5, bals, false).await?;
        insert_trivial_encrypt(&harness.listener_db, &mut tx, tx_id, 10, 5, trxa, false).await?;

        let has_funds = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheGe(TfheContract::FheGe {
                caller,
                lhs: bals,
                rhs: trxa,
                scalarByte: scalar_flag(false),
                result: has_funds,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &has_funds).await?;
        output_handles.push(has_funds);

        let cast_funds = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::Cast(TfheContract::Cast {
                caller,
                ct: has_funds,
                toType: crate::tests::event_helpers::to_ty(5),
                result: cast_funds,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &cast_funds).await?;
        output_handles.push(cast_funds);

        let selected = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheMul(TfheContract::FheMul {
                caller,
                lhs: trxa,
                rhs: cast_funds,
                scalarByte: scalar_flag(false),
                result: selected,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &selected).await?;
        output_handles.push(selected);

        let new_to = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller,
                lhs: bald,
                rhs: selected,
                scalarByte: scalar_flag(false),
                result: new_to,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &new_to).await?;
        output_handles.push(new_to);

        let new_from = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheSub(TfheContract::FheSub {
                caller,
                lhs: bals,
                rhs: selected,
                scalarByte: scalar_flag(false),
                result: new_from,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &new_from).await?;
        output_handles.push(new_from);

        bald = new_to;
    }
    tx.commit().await?;

    wait_until_computed(&harness.app).await?;
    let resp = decrypt_handles(&harness.pool, &output_handles).await?;
    assert_eq!(resp.len(), output_handles.len());
    for (i, r) in resp.iter().enumerate() {
        let to_bal = (20 + (i / 5 + 1) * 10).to_string();
        match r.value.as_str() {
            "true" if i % 5 == 0 => (),
            "1" if i % 5 == 1 => (),
            "10" if i % 5 == 2 => (),
            val if i % 5 == 3 => assert_eq!(val, to_bal, "Destination balances don't match."),
            "90" if i % 5 == 4 => (),
            s => panic!("unexpected result: {s} for output {i}"),
        }
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn counter_increment() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let num_samples = sample_count(7);
    let mut tx = harness.listener_db.new_transaction().await?;
    let tx_id = next_handle();

    let mut counter = next_handle();
    insert_trivial_encrypt(&harness.listener_db, &mut tx, tx_id, 42, 5, counter, false).await?;

    let caller = zero_address();
    let mut output_handles = Vec::with_capacity(num_samples);
    for _ in 0..num_samples {
        let new_counter = next_handle();
        insert_event(
            &harness.listener_db,
            &mut tx,
            tx_id,
            TfheContractEvents::FheAdd(TfheContract::FheAdd {
                caller,
                lhs: counter,
                rhs: scalar_u128_handle(7),
                scalarByte: scalar_flag(true),
                result: new_counter,
            }),
            true,
        )
        .await?;
        allow_handle(&harness.listener_db, &mut tx, &new_counter).await?;
        output_handles.push(new_counter);
        counter = new_counter;
    }
    tx.commit().await?;

    wait_until_computed(&harness.app).await?;
    let resp = decrypt_handles(&harness.pool, &output_handles).await?;
    assert_eq!(resp.len(), output_handles.len());
    for (i, r) in resp.iter().enumerate() {
        let target = (42 + (i + 1) * 7).to_string();
        assert_eq!(r.value.as_str(), target, "Counter value incorrect.");
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn tree_reduction() -> Result<(), Box<dyn std::error::Error>> {
    let harness = setup_event_harness().await?;
    let num_samples = sample_count(16);
    let mut tx = harness.listener_db.new_transaction().await?;
    let tx_id = next_handle();
    let caller = zero_address();

    let num_levels = (num_samples as f64).log2().ceil() as usize;
    let mut num_comps_at_level = 2_f64.powi((num_levels - 1) as i32) as usize;
    let expected = num_comps_at_level * 2;

    let mut level_inputs = Vec::with_capacity(num_comps_at_level * 2);
    for _ in 0..num_comps_at_level {
        let lhs = next_handle();
        let rhs = next_handle();
        insert_trivial_encrypt(&harness.listener_db, &mut tx, tx_id, 1, 5, lhs, false).await?;
        insert_trivial_encrypt(&harness.listener_db, &mut tx, tx_id, 1, 5, rhs, false).await?;
        level_inputs.push(lhs);
        level_inputs.push(rhs);
    }

    let mut last_output = next_handle();
    for _ in 0..num_levels {
        let mut level_outputs = Vec::with_capacity(num_comps_at_level);
        for i in 0..num_comps_at_level {
            let out = next_handle();
            insert_event(
                &harness.listener_db,
                &mut tx,
                tx_id,
                TfheContractEvents::FheAdd(TfheContract::FheAdd {
                    caller,
                    lhs: level_inputs[2 * i],
                    rhs: level_inputs[2 * i + 1],
                    scalarByte: scalar_flag(false),
                    result: out,
                }),
                true,
            )
            .await?;
            allow_handle(&harness.listener_db, &mut tx, &out).await?;
            level_outputs.push(out);
            last_output = out;
        }
        num_comps_at_level /= 2;
        if num_comps_at_level < 1 {
            break;
        }
        level_inputs = level_outputs;
    }
    tx.commit().await?;

    wait_until_computed(&harness.app).await?;
    let resp = decrypt_handles(&harness.pool, &[last_output]).await?;
    assert_eq!(resp.len(), 1);
    assert_eq!(resp[0].value, expected.to_string(), "Incorrect result.");

    Ok(())
}
