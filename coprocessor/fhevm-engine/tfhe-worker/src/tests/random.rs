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
        allow_handle, decrypt_ciphertexts, default_api_key, random_handle, setup_test_app,
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
        });
        allow_handle(&output_handle, &pool).await?;
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
        });
        allow_handle(&output_handle, &pool).await?;
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
        });
        allow_handle(&output_handle, &pool).await?;
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
        DecryptionResult { value: "15".to_string(), output_type: 1 },
        DecryptionResult { value: "159".to_string(), output_type: 2 },
        DecryptionResult { value: "20639".to_string(), output_type: 3 },
        DecryptionResult { value: "3144634527".to_string(), output_type: 4 },
        DecryptionResult { value: "11389097925934010527".to_string(), output_type: 5 },
        DecryptionResult { value: "69905114149028325220139576636480639135".to_string(), output_type: 6 },
        DecryptionResult { value: "317892124126989078445591712223758797229205180575".to_string(), output_type: 7 },
        DecryptionResult { value: "48332257473318369114446160603249796578974664224488597218947446324867726528671".to_string(), output_type: 8 },
        DecryptionResult { value: "12198094884744090542173285362698618341518582624536702161123123347979469866974417098283994537378240028766239364397082365302194163000740831280041405199634591".to_string(), output_type: 9 },
        DecryptionResult { value: "120156223302337630365848476036786830712221225837911304478354686571013607080800490006328148395527855607705362454110623036517495935100248170368774030250429933486550233102311509076010083356505508644166081668859818268613503527775654712000885291624154220587982215517677616908613791777014567328828870853665911427231".to_string(), output_type: 10 },
        DecryptionResult { value: "14248733319336792947145061074942516843891742027261939361102367792409448522228345179275022180060600939183011526580730562988252371177285716092409404887163918996653241343851071075489477752617083491985344705461476849828053937934675915068245283966656149780357819186409848959266003845101304608186407280044692551973982691162910974209860876803482926693301499577945372782427737240493529817720694750485232689175107806080251447653801599888643804328955230652322074872799064074283732867375742718184497278018988287728169906996217187811910919757316338009264522446180972826736170539938262086414309326211232387594250615674960429928607".to_string(), output_type: 11 }
    ];
    #[cfg(feature = "gpu")]
    let expected: Vec<DecryptionResult> = vec![
        DecryptionResult {
            value: "false".to_string(),
            output_type: 0,
        },
        DecryptionResult {
            value: "5".to_string(),
            output_type: 1,
        },
        DecryptionResult {
            value: "149".to_string(),
            output_type: 2,
        },
        DecryptionResult {
            value: "19349".to_string(),
            output_type: 3,
        },
        DecryptionResult {
            value: "417483669".to_string(),
            output_type: 4,
        },
        DecryptionResult {
            value: "18215029179758562197".to_string(),
            output_type: 5,
        },
        DecryptionResult {
            value: "285171577856650533455529174907356334997".to_string(),
            output_type: 6,
        },
        DecryptionResult {
            value: "152905830138175504431271466872053891096341793685".to_string(),
            output_type: 7,
        },
        DecryptionResult {
            value: "75047619794910340784669152516253626220419211295364484836071532923723869997973"
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
        "3351951982485649274893506249551461531869841455148098344430890360930441007518386744200468574541725856922507964546621512713438470702986642486608412251521024",
        "44942328371557897693232629769725618340449424473557664318357520289433168951375240783177119330601884005280028469967848339414697442203604155623211857659868531094441973356216371319075554900311523529863270738021251442209537670585615720368478277635206809290837627671146574559986811484619929076208839082406056034304",
        "8079251517827751825178719172167487990111025667428871008032586356881163784716972723299300352880728365922179490230474504873529889787622730273772038096612070780157719341825249022937549437597413026699014409596016892069198054660654939040459523584619042617645411463009076260721893972885266452151888099780982596380478583347417085605171243696641142373714044008831580514519451414832756548177115078537564648216044279181485900929615464339399587788075411476100924403308321807806781421177705052431289275431732830867419635645164174483761499317088249659553881291597359333885900533858307401161329619651238037048388963402764899057664",
    ];
    #[cfg(not(feature = "gpu"))]
    let results = [
        "true",
        "3",
        "95",
        "4255",
        "997150879",
        "2165725889079234719",
        "69905114149028325220139576636480639135",
        "317892124126989078445591712223758797229205180575",
        "19384235163989320258553414351077819615657168058078456209083050322889444118687",
        "2142238937287142717492766614044233745909058259092407127830452265188146844419256865682588813753062457998715470757217827161878750891780903820216168445071519",
        "30271566559221834979383216497335594031322376890795975841639645992147269178050008439973909734324087597145305514174926357688101050693039859122350314930692871297666286389878766437858973555882461584439540192817315384194428186604423271263928736353740602006306960175384467788640168807774709176411192688853799358623",
        "6169481801509041121966341902775028853780716359833068353069781435528284737511372455975721827179872573260832036350256058114722481389662985818637366790551848216495522002025822052551928315019670465286330295865459957758855883274020976027785760382037107162712407723400772698544109872216038156034519180263709955593504107815493888604689633106841784319587455569113792267908285825660773269543579671947668040959063526898765546724186135549244216540879819176221150469490742266476951446198037665753208002587255456860750271351053013328149420440228088349710641154583613492850270006079954685252979706559994350545861652272195530870943",
    ];
    #[cfg(feature = "gpu")]
    let results = [
        "true",
        "3",
        "95",
        "4255",
        "997150879",
        "2165725889079234719",
        "69905114149028325220139576636480639135",
        "317892124126989078445591712223758797229205180575",
        "19384235163989320258553414351077819615657168058078456209083050322889444118687",
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
        });
        allow_handle(&output_handle, &pool).await?;
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
