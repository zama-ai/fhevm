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
    let block_hash = next_handle();
    let block_number = 12345_u64;
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
            block_hash: block_hash.clone(),
            block_number,
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
            block_hash: block_hash.clone(),
            block_number,
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
            block_hash: block_hash.clone(),
            block_number,
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
        DecryptionResult { value: "14".to_string(), output_type: 1 },
        DecryptionResult { value: "110".to_string(), output_type: 2 },
        DecryptionResult { value: "25710".to_string(), output_type: 3 },
        DecryptionResult { value: "2301977710".to_string(), output_type: 4 },
        DecryptionResult { value: "2124469003621852270".to_string(), output_type: 5 },
        DecryptionResult { value: "302505226437502196976291892336864355438".to_string(), output_type: 6 },
        DecryptionResult { value: "1108505784518728418941925401722025312826563060846".to_string(), output_type: 7 },
        DecryptionResult { value: "91126377170185961821859299570816872415960892403427050990577017965002762839150".to_string(), output_type: 8 },
        DecryptionResult { value: "6922152174502812819541117929565886419979032194201713031364562045239089177214539727726262702229414643692655368798839136842332367875784413253251929757541486".to_string(), output_type: 9 },
        DecryptionResult { value: "46292958359651000446442970822030237984729180059428482069495508220766054410039851432242501028059680884559363235277734516881603292424012563704460966342066139304397727662470435440766471281500712626984856035127831101691856273260533177950768622436605340597015447884662833094737801854406574285721065553810196620398".to_string(), output_type: 10 },
        DecryptionResult { value: "19366715041382087641703612484552249244187072336000885470385288586981801677187904586405444522508800282573665369274273890968159817127001139444178452126787109107071987648588764258452153875757367092164435262611470070330101502732418033994048490250401360902192046837680752648261413452195148137545219876661724890351331347604933664429281088816477375296876876531304998388499127947392826174401162163758970998813610929416858500220952663636485501844293391382813958363730841469302791359890855173063921329851153104221774931052580016325789314293273437735628912181501829333776496467493732834524085385578650231652302155872382861272174".to_string(), output_type: 11 }
    ];
    #[cfg(feature = "gpu")]
    let expected: Vec<DecryptionResult> = vec![
        DecryptionResult {
            value: "true".to_string(),
            output_type: 0,
        },
        DecryptionResult {
            value: "10".to_string(),
            output_type: 1,
        },
        DecryptionResult {
            value: "42".to_string(),
            output_type: 2,
        },
        DecryptionResult {
            value: "13098".to_string(),
            output_type: 3,
        },
        DecryptionResult {
            value: "24916778".to_string(),
            output_type: 4,
        },
        DecryptionResult {
            value: "13546293436763353898".to_string(),
            output_type: 5,
        },
        DecryptionResult {
            value: "244133020262364485587543763849414718250".to_string(),
            output_type: 6,
        },
        DecryptionResult {
            value: "861771350222443153516072989587279784988534125354".to_string(),
            output_type: 7,
        },
        DecryptionResult {
            value: "2972409412398590319234166868940218748864587370013358645802893111142926529322"
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
        "2",
        "46",
        "9326",
        "154494062",
        "2124469003621852270",
        "47293451246798349378760936763038196846",
        "12379556520551230289161777184813048084613653614",
        "4282310242198815254181060814300941526008403904196627960983829959067915609198",
        "218248209531514269754105430462963356239349283905516342502781323378207162177766239325325553145962929847639439705596111415455426469811128280035105254499438",
        "1350629988093102753210341052304619644279755585870817751137987931332885458664610649065381697457796879279334765309886177466905850220408408081249108682197608209955754306254064121690916381189189097121585297106579659482318602674917457582290344801398531306177820213516258534750990369786645209512226471404140586094",
        "3208212005726583991346174140217273263965021001143143454320115873219474107753959139806843816747343550729306388813324881221100037551755678896634375933562967546756548964938266212577055000562541038766406443419436286191705393411108155913129443081163275666901223911662600126817625506424615233241443677099759697590374180910099493218938601423195090549448788513641837359460225117727313078046932006683841702381522371053886698361721734957686326268142568430612109557114197853689228517535445068201342778987687442486935659762251667358266315659096938416521149598307110666004695399777118032201426146276174157555524229066853063156846",
    ];
    #[cfg(feature = "gpu")]
    let results = [
        "true",
        "2",
        "42",
        "13098",
        "24916778",
        "4322921399908578090",
        "73991836801895253855856460133530612522",
        "131020531556991694414230573229138275160567853866",
        "2972409412398590319234166868940218748864587370013358645802893111142926529322",
    ];
    let block_hash = next_handle();
    let block_number = 12345_u64;
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
            block_hash: block_hash.clone(),
            block_number,
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
