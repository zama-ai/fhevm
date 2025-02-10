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
        DecryptionResult { value: "false".to_string(), output_type: 0 },
        DecryptionResult { value: "0".to_string(), output_type: 1 },
        DecryptionResult { value: "32".to_string(), output_type: 2 },
        DecryptionResult { value: "11808".to_string(), output_type: 3 },
        DecryptionResult { value: "629550624".to_string(), output_type: 4 },
        DecryptionResult { value: "3599940269858237984".to_string(), output_type: 5 },
        DecryptionResult { value: "150656153923308677215624867801423293984".to_string(), output_type: 6 },
        DecryptionResult { value: "1261613769508706031045665600428040676770937450016".to_string(), output_type: 7 },
        DecryptionResult { value: "18986873298894233193145929080150256577322576591471711287367330129765741243936".to_string(), output_type: 8 },
        DecryptionResult { value: "10232350915780295990148615700987438737697741448785160860135259387386215782819850109804323105785900141458372909187170741294522096847981892229609774582738464".to_string(), output_type: 9 },
        DecryptionResult { value: "114367637199514847280448439417333260889725768550794741460093029758684557000312443906223981262973346694472790204966807897959645636684378913727005482338418513179184032059107143496248018885275767404657195894019321555130941197468256351150140818663045904088790403066627628439931419938370391918367209538518541282848".to_string(), output_type: 10 },
        DecryptionResult { value: "23940285086149406195104910222696349773472952003969972901666479954284412735482075737869257567496112381323535591795817090039940361948031502028694254712019646399685913783316013749903402108425160731492109549265457576392817155024878310259964474796614021191592424323638774487508424508416957672843897425589775197725978251329316157873335206056648769835100133311157452200251642784760686809522544321619752333854545498304174217444123021187312416085732051585256387418702550559610091526362070737477336848035323232793394860744635340095871616414245206326412912897684765261334599455438505400455977427966529860876570606701934348217888".to_string(), output_type: 11 }
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
    let results = [
        "false",
        "0",
        "32",
        "11808",
        "629550624",
        "3599940269858237984",
        "65585562193074061349781215943481241120",
        "165487541510528842392901975890828412028988042784",
        "18986873298894233193145929080150256577322576591471711287367330129765741243936",
        "176494968323348165468096952333054142088217083340865826842588304594892760264689877202917382160722570690849015547306203154206684739021964769784537828175392",
        "24482980456399051893983179877882024208826919603679412823377989179818219097561962339869742601769578683912733265031111219130250752277170602480581767018681450990300085346674400858096909084652720344930654417976818670711865856297024910413184263392632285507115147724334479319957796969130533765949531373706429214240",
        "7781782050493902544747471878361373793250900669112230885601307240522085166048130291270656861734655649479176611334868080292880582372786041481150178518795504839370475099665515704028303233230334678094080730073423792254421045703568432179045427627375935956301601397620621966064636562646424768540121226027810004965021084634481986662992718663366485087672045293494291171212739955095173713168314164544623037422456939941202415584892092508513240509581228633054538612085906943996528684006660632614758297171857571058555589454306991128348617780068707007305150314490046593562798387721890598133318188664053786779792679896404550102560",
    ];

    for (idx, the_type) in supported_types().iter().enumerate() {
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
        // skip boolean bounds check
        if resp[idx].output_type > 0 {
            assert!(BigInt::from_str(bounds[idx])
                .unwrap()
                .gt(&BigInt::from_str(&resp[idx].value).unwrap()));
        }
    }

    Ok(())
}
