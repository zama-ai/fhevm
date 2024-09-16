use std::str::FromStr;

use bigdecimal::num_bigint::BigInt;
use tonic::metadata::MetadataValue;

use crate::{
    server::{
        common::FheOperation,
        coprocessor::{
            async_computation_input::Input, fhevm_coprocessor_client::FhevmCoprocessorClient,
            AsyncComputation, AsyncComputationInput, AsyncComputeRequest,
        },
    },
    tests::utils::{
        decrypt_ciphertexts, default_api_key, random_handle, setup_test_app,
        wait_until_all_ciphertexts_computed, DecryptionResult,
    },
};

use super::operators::supported_types;

#[tokio::test]
async fn test_fhe_random_basic() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;

    let mut handle_counter = random_handle();
    let mut next_handle = || {
        let out: u64 = handle_counter;
        handle_counter += 1;
        out.to_be_bytes().to_vec()
    };

    let api_key_header = format!("bearer {}", default_api_key());

    let mut async_computations = Vec::new();
    let mut output_handles = Vec::new();
    // second batch
    let mut repeated_output_handles = Vec::new();
    let mut other_seed_output_handles = Vec::new();

    let random_test_types = supported_types();

    let deterministic_seed = 123u8;
    for the_type in random_test_types {
        let output_handle = next_handle();
        output_handles.push(output_handle.clone());

        async_computations.push(AsyncComputation {
            operation: FheOperation::FheRand.into(),
            output_handle,
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![deterministic_seed])),
                },
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![*the_type as u8])),
                },
            ],
        });
    }

    for the_type in random_test_types {
        let output_handle = next_handle();
        repeated_output_handles.push(output_handle.clone());

        async_computations.push(AsyncComputation {
            operation: FheOperation::FheRand.into(),
            output_handle,
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![deterministic_seed])),
                },
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![*the_type as u8])),
                },
            ],
        });
    }

    let deterministic_seed = 124u8;
    for the_type in random_test_types {
        let output_handle = next_handle();
        other_seed_output_handles.push(output_handle.clone());

        async_computations.push(AsyncComputation {
            operation: FheOperation::FheRand.into(),
            output_handle,
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![deterministic_seed])),
                },
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![*the_type as u8])),
                },
            ],
        });
    }
    println!("Scheduling computations...");
    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await?;
    println!("Computations scheduled, waiting upon completion...");

    wait_until_all_ciphertexts_computed(&app).await?;

    let decrypt_request = output_handles.clone();
    let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;
    let expected: Vec<DecryptionResult> = vec![
        DecryptionResult { value: "true".to_string(), output_type: 0 },
        DecryptionResult { value: "15".to_string(), output_type: 1 },
        DecryptionResult { value: "191".to_string(), output_type: 2 },
        DecryptionResult { value: "31935".to_string(), output_type: 3 },
        DecryptionResult { value: "50166975".to_string(), output_type: 4 },
        DecryptionResult { value: "1340532071352597695".to_string(), output_type: 5 },
        DecryptionResult { value: "124020002486967631373104553377985821887".to_string(), output_type: 6 },
        DecryptionResult { value: "1460743068773264656276930615123138630088589671615".to_string(), output_type: 7 },
        DecryptionResult { value: "9073722870604321437325343932192605854447491367692274273821339035224667684031".to_string(), output_type: 8 },
        DecryptionResult { value: "305206023014776799985230721993414106811955216557006729499329334660464767143319849635649940645070409986179511078348380864394886287375691494634900011449535".to_string(), output_type: 9 },
        DecryptionResult { value: "47107745454094956047032061683228950257813886136471742519491594819930746822037387709806363956594057633886089295195718013393612776278364590927545759803018868145037996301653648244673256559634442220254150128066779099210193723099178032989466601285160252652521543788771151245736124664154459945466742327296640515263".to_string(), output_type: 10 },
        DecryptionResult { value: "15993832479142254582179552051978985171942987041581078316307196452126031943098588175022265343465618290530692834276986750976786634669778707390871076641509515302233954805902327473616311936122148439292611965911683385306561778692974324402831129553488123365277966756540494010076336201015765082225027076061721793429534417162741021200774038978621182013568300885057697263422825507901057344782449000322153843538116752641484000884044992485272920464597070056681924136192427539146031607003264945583126338515417165811516149183380031917969225191063070817311680151162289712932918856949044454757316587442727824006976546517520450682047".to_string(), output_type: 11 }
    ];

    println!("results: {:#?}", resp);

    assert_eq!(expected, resp);

    let resp_repeated = decrypt_ciphertexts(&pool, 1, repeated_output_handles).await?;
    assert_eq!(
        resp, resp_repeated,
        "randomness generation is not deterministic"
    );

    let resp_repeated = decrypt_ciphertexts(&pool, 1, other_seed_output_handles).await?;
    assert_ne!(resp, resp_repeated, "seed has change, so must the values");

    Ok(())
}

fn to_be_bytes(input: &str) -> Vec<u8> {
    let num = BigInt::from_str(input).unwrap();
    let (_, bytes_be) = num.to_bytes_be();
    bytes_be
}

#[tokio::test]
async fn test_fhe_random_bounded() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;

    let mut handle_counter = random_handle();
    let mut next_handle = || {
        let out: u64 = handle_counter;
        handle_counter += 1;
        out.to_be_bytes().to_vec()
    };

    let api_key_header = format!("bearer {}", default_api_key());

    let mut async_computations = Vec::new();
    let mut output_handles = Vec::new();

    let deterministic_seed = 123u8;
    let bounds = [
        "2",
        "4",
        "128",
        "16384",
        "1073741824",
        "4611686018427387904",
        "85070591730234615865843651857942052864",
        "365375409332725729550921208179070754913983135744",
        "28948022309329048855892746252171976963317496166410141009864396001978282409984",
    ];
    let results = [
        "true",
        "3",
        "127",
        "15551",
        "50166975",
        "1340532071352597695",
        "38949410756733015507260901520043769023",
        "364616840775087467624166990585926365346640264383",
        "9073722870604321437325343932192605854447491367692274273821339035224667684031",
    ];

    for (idx, the_type) in supported_types().iter().enumerate() {
        if *the_type > 8 {
            // don't support bounded numbers larger than 256 scalar type
            break;
        }

        let output_handle = next_handle();
        output_handles.push(output_handle.clone());

        async_computations.push(AsyncComputation {
            operation: FheOperation::FheRandBounded.into(),
            output_handle,
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![deterministic_seed])),
                },
                AsyncComputationInput {
                    input: Some(Input::Scalar(to_be_bytes(bounds[idx]))),
                },
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![*the_type as u8])),
                },
            ],
        });
    }

    println!("Scheduling computations...");
    let mut compute_request = tonic::Request::new(AsyncComputeRequest {
        computations: async_computations,
    });
    compute_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let _resp = client.async_compute(compute_request).await?;
    println!("Computations scheduled, waiting upon completion...");

    wait_until_all_ciphertexts_computed(&app).await?;

    let decrypt_request = output_handles.clone();
    let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;
    assert_eq!(resp.len(), bounds.len());

    println!("response: {:#?}", resp);
    for idx in 0..bounds.len() {
        assert_eq!(resp[idx].output_type, supported_types()[idx] as i16);
        assert_eq!(resp[idx].value, results[idx]);
    }

    Ok(())
}
