//! Capture canonical CPI payloads for host-listener golden decode tests.

#[test]
#[ignore = "run manually to refresh host-listener/tests/fixtures/transfer_sub_cpi.bin"]
fn capture_transfer_sub_cpi_payload() {
    use zama_solana_litesvm_harness::{
        collect_zama_host_cpi_payloads, decode_zama_host_cpi_event, run_transfer_scenario,
        token_fixture, FheBinaryOpCode, TransferSetup, ZamaHostEvent,
    };

    let mut fixture = token_fixture();
    let scenario = run_transfer_scenario(&mut fixture, TransferSetup::default());
    let payloads = collect_zama_host_cpi_payloads(
        &scenario.meta,
        &scenario.account_keys,
        scenario.host_program_id,
    );
    let decoded: Vec<_> = payloads
        .iter()
        .filter_map(|payload| decode_zama_host_cpi_event(payload))
        .collect();
    let sub = decoded
        .iter()
        .find_map(|event| match event {
            ZamaHostEvent::FheBinaryOp(event) if event.op == FheBinaryOpCode::Sub => Some(event),
            _ => None,
        })
        .expect("expected sub binary op CPI event");

    let payload = payloads
        .iter()
        .find(|payload| {
            matches!(
                decode_zama_host_cpi_event(payload),
                Some(ZamaHostEvent::FheBinaryOp(event)) if event.result == sub.result
            )
        })
        .expect("expected sub CPI payload bytes");

    std::fs::write(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../coprocessor/fhevm-engine/host-listener/tests/fixtures/transfer_sub_cpi.bin"
        ),
        payload,
    )
    .expect("write golden fixture");
}
