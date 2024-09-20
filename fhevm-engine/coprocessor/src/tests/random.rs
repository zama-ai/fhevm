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
        DecryptionResult { value: "6".to_string(), output_type: 1 },
        DecryptionResult { value: "6".to_string(), output_type: 2 },
        DecryptionResult { value: "23046".to_string(), output_type: 3 },
        DecryptionResult { value: "2257672710".to_string(), output_type: 4 },
        DecryptionResult { value: "12138718414261803526".to_string(), output_type: 5 },
        DecryptionResult { value: "130536719590611940049803920731387550214".to_string(), output_type: 6 },
        DecryptionResult { value: "971176705489787087023559718483701127113677560326".to_string(), output_type: 7 },
        DecryptionResult { value: "62210255757460412253332620363065848989112923584999887570035464828426661222918".to_string(), output_type: 8 },
        DecryptionResult { value: "167958935840398111366003661819132943572579228212385323643009044778284654758971531763634195717060767316412295162146605242695852136468800900790045270694406".to_string(), output_type: 9 },
        DecryptionResult { value: "127460563385689404084570635453516642982330737396307363709535669246693726363369279326274116849562765049033667934125131507607869225026009107310544028242879211116101076829363291657387574479716476869613221980036198477470920343187777849916436388023322996436007563319615378730113313056846971613305517149919649028614".to_string(), output_type: 10 },
        DecryptionResult { value: "29687326363179539154232170826093317060572491263948154715413122357200687474061448043555291795321984983113829977114301561317315809196828773909981565653610082891472340553741585442577497506409472143098823132371629384036451019214072899732235656145602725111017828708028912154841404994944466545632048686969494346234325709069045453046020648098209481065154942201598888424765642988091655940417557742117518483932517015160272576663001732809302519121630949039706341063098676812339442637939392896074884484156187775746589025164758187166306751922076107008755031211360389068550389609734783888124482836062055425119177200121882346609158".to_string(), output_type: 11 }
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
        "2",
        "6",
        "6662",
        "110189062",
        "2915346377407027718",
        "45466127860377324183960268873445497350",
        "240425886824335627921717302125559617285711288838",
        "4314211138802314541547127858721895062477931252179605550306672824470096402950",
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
