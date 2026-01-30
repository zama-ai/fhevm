use std::str::FromStr;

use bigdecimal::num_bigint::BigInt;
use tonic::metadata::MetadataValue;

use crate::{
    server::{
        common::FheOperation,
        tfhe_worker::{
            async_computation_input::Input, fhevm_coprocessor_client::FhevmCoprocessorClient,
            AsyncComputation, AsyncComputationInput, AsyncComputeRequest,
        },
    },
    tests::utils::{
        decrypt_ciphertexts, default_api_key, random_handle, setup_test_app,
        wait_until_all_allowed_handles_computed, DecryptionResult,
    },
};

use super::operators::supported_types;

fn random_test_supported_types() -> &'static [i32] {
    if !cfg!(feature = "gpu") {
        supported_types()
    } else {
        &supported_types()[0..9]
    }
}

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

    let random_test_types = random_test_supported_types();

    let deterministic_seed = 123u8;
    for the_type in random_test_types {
        let transaction_id = next_handle();
        let output_handle = next_handle();
        output_handles.push(output_handle.clone());

        async_computations.push(AsyncComputation {
            operation: FheOperation::FheRand.into(),
            transaction_id: transaction_id.clone(),
            output_handle: output_handle.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![deterministic_seed])),
                },
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![*the_type as u8])),
                },
            ],
            is_allowed: true,
        });
    }

    for the_type in random_test_types {
        let transaction_id = next_handle();
        let output_handle = next_handle();
        repeated_output_handles.push(output_handle.clone());

        async_computations.push(AsyncComputation {
            operation: FheOperation::FheRand.into(),
            transaction_id: transaction_id.clone(),
            output_handle: output_handle.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![deterministic_seed])),
                },
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![*the_type as u8])),
                },
            ],
            is_allowed: true,
        });
    }

    let deterministic_seed = 124u8;
    for the_type in random_test_types {
        let transaction_id = next_handle();
        let output_handle = next_handle();
        other_seed_output_handles.push(output_handle.clone());

        async_computations.push(AsyncComputation {
            operation: FheOperation::FheRand.into(),
            transaction_id: transaction_id.clone(),
            output_handle: output_handle.clone(),
            inputs: vec![
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![deterministic_seed])),
                },
                AsyncComputationInput {
                    input: Some(Input::Scalar(vec![*the_type as u8])),
                },
            ],
            is_allowed: true,
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

    wait_until_all_allowed_handles_computed(&app).await?;

    let decrypt_request = output_handles.clone();
    let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;
    #[cfg(not(feature = "gpu"))]
    let expected: Vec<DecryptionResult> = vec![
        DecryptionResult { value: "true".to_string(), output_type: 0 },
        DecryptionResult { value: "12".to_string(), output_type: 1 },
        DecryptionResult { value: "156".to_string(), output_type: 2 },
        DecryptionResult { value: "51868".to_string(), output_type: 3 },
        DecryptionResult { value: "2619984540".to_string(), output_type: 4 },
        DecryptionResult { value: "10012971755022961308".to_string(), output_type: 5 },
        DecryptionResult { value: "315195588681747109104923061743488977564".to_string(), output_type: 6 },
        DecryptionResult { value: "788883063982248783364032654023297427550759406236".to_string(), output_type: 7 },
        DecryptionResult { value: "76547111697560389694815048035853209325312793002204865478146017769839135410844".to_string(), output_type: 8 },
        DecryptionResult { value: "4620300173090955915208010885061878517829873825553098492220465724030076978650428492943844398737585329942351846650303374010943990494045198570449496290478748".to_string(), output_type: 9 },
        DecryptionResult { value: "41494007511305936838300758727359299346214242261416586280906617017153184838995422253823927812442946876630762826820589205469843393333804107103032285280293973663368090988288979532934632731245358743733005968512630386343560460685593551815532487760564915903224952146254993560801533799573886208522311328831237638812".to_string(), output_type: 10 },
        DecryptionResult { value: "19733886402355865032400462714816069368039690184989121557781677033259228589971262966675570089584500610059101607007199306637911675727319722458440947456208802538556773930772101832011802133239078178777473379921690577802297622966469875565470144225558503110585823618846573672732754957550113490674349293976082721088842023265656667800220207532026919372995977171149881718663219561963034867768017575285621955948819683133204048721069630295364245981944082975096068080088379308849511079501340419770780049292625884104832624282900045237278591546992102124172326011290396356133702950143455725743711191910217441597593339600775705905820".to_string(), output_type: 11 }
    ];
    #[cfg(feature = "gpu")]
    let expected: Vec<DecryptionResult> = vec![
        DecryptionResult {
            value: "false".to_string(),
            output_type: 0,
        },
        DecryptionResult {
            value: "12".to_string(),
            output_type: 1,
        },
        DecryptionResult {
            value: "156".to_string(),
            output_type: 2,
        },
        DecryptionResult {
            value: "51868".to_string(),
            output_type: 3,
        },
        DecryptionResult {
            value: "2619984540".to_string(),
            output_type: 4,
        },
        DecryptionResult {
            value: "10012971755022961308".to_string(),
            output_type: 5,
        },
        DecryptionResult {
            value: "315195588681747109104923061743488977564".to_string(),
            output_type: 6,
        },
        DecryptionResult {
            value: "788883063982248783364032654023297427550759406236".to_string(),
            output_type: 7,
        },
        DecryptionResult {
            value: "76547111697560389694815048035853209325312793002204865478146017769839135410844"
                .to_string(),
            output_type: 8,
        },
    ];

    println!("results: {:#?}", resp);

    assert_eq!(expected, resp);

    let resp_repeated = decrypt_ciphertexts(&pool, 1, repeated_output_handles).await?;
    assert_eq!(
        resp, resp_repeated,
        "randomness generation is not deterministic"
    );

    let resp_repeated = decrypt_ciphertexts(&pool, 1, other_seed_output_handles).await?;
    assert_ne!(resp, resp_repeated, "seed has changed, so must the values");

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
        "3351951982485649274893506249551461531869841455148098344430890360930441007518386744200468574541725856922507964546621512713438470702986642486608412251521024",
        "44942328371557897693232629769725618340449424473557664318357520289433168951375240783177119330601884005280028469967848339414697442203604155623211857659868531094441973356216371319075554900311523529863270738021251442209537670585615720368478277635206809290837627671146574559986811484619929076208839082406056034304",
        "8079251517827751825178719172167487990111025667428871008032586356881163784716972723299300352880728365922179490230474504873529889787622730273772038096612070780157719341825249022937549437597413026699014409596016892069198054660654939040459523584619042617645411463009076260721893972885266452151888099780982596380478583347417085605171243696641142373714044008831580514519451414832756548177115078537564648216044279181485900929615464339399587788075411476100924403308321807806781421177705052431289275431732830867419635645164174483761499317088249659553881291597359333885900533858307401161329619651238037048388963402764899057664",
    ];
    #[cfg(not(feature = "gpu"))]
    let results = [
        "true",
        "0",
        "92",
        "2716",
        "472500892",
        "789599718168185500",
        "59983813491043261507392106169662818972",
        "58132245316797324262190237665155917722793134748",
        "18651067078902291983029555531509255398677800669384583458417225765882570590876",
        "1268348190605306640314504635510416985960032370405000147789575363099635971132041748743375824195859473019843882103681861297505519791058556083841084038957724",
        "41494007511305936838300758727359299346214242261416586280906617017153184838995422253823927812442946876630762826820589205469843393333804107103032285280293973663368090988288979532934632731245358743733005968512630386343560460685593551815532487760564915903224952146254993560801533799573886208522311328831237638812",
        "3575383366700361382043024370481093387817638850131379541716504319496901020537317520076969383823043878214742626546250296890851896152074261910896871262984660978241335247121603786136703258044252125379444560729656793663901513645159997484551097056320417875295000692828421151288967011779580586370573094414117528327884856570822496589877720138744634625567889153486720689624316732297521771413787418210492659516731124770232246861838701616565070405793260022894219273471735693235948237145930314908201498429160222369993352992571696269755592912815602805064563428095677688361901882426840923421051952607741367500815412795245907790492",
    ];
    #[cfg(feature = "gpu")]
    let results = [
        "false",
        "0",
        "92",
        "2716",
        "472500892",
        "789599718168185500",
        "59983813491043261507392106169662818972",
        "58132245316797324262190237665155917722793134748",
        "18651067078902291983029555531509255398677800669384583458417225765882570590876",
    ];

    for (idx, the_type) in random_test_supported_types().iter().enumerate() {
        let transaction_id = next_handle();
        let output_handle = next_handle();
        output_handles.push(output_handle.clone());

        async_computations.push(AsyncComputation {
            operation: FheOperation::FheRandBounded.into(),
            transaction_id: transaction_id.clone(),
            output_handle: output_handle.clone(),
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
            is_allowed: true,
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

    wait_until_all_allowed_handles_computed(&app).await?;

    let decrypt_request = output_handles.clone();
    let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;
    assert_eq!(resp.len(), results.len());

    println!("response: {:#?}", resp);
    for idx in 0..results.len() {
        assert_eq!(
            resp[idx].output_type,
            random_test_supported_types()[idx] as i16
        );
        assert_eq!(resp[idx].value, results[idx]);
        // skip boolean bounds check
        if resp[idx].output_type > 0 {
            assert!(BigInt::from_str(bounds[idx])
                .unwrap()
                .gt(&BigInt::from_str(&resp[idx].value).unwrap()));
        }
    }

    Ok(())
}
