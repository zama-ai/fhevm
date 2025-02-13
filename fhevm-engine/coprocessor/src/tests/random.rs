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
    #[cfg(not(feature = "gpu"))]
    let expected: Vec<DecryptionResult> = vec![
        DecryptionResult { value: "false".to_string(), output_type: 0 },
        DecryptionResult { value: "10".to_string(), output_type: 1 },
        DecryptionResult { value: "74".to_string(), output_type: 2 },
        DecryptionResult { value: "20042".to_string(), output_type: 3 },
        DecryptionResult { value: "2317110858".to_string(), output_type: 4 },
        DecryptionResult { value: "3781517732040429130".to_string(), output_type: 5 },
        DecryptionResult { value: "173067494276272686594848549919321247306".to_string(), output_type: 6 },
        DecryptionResult { value: "1094515384870572566830339459490292190735847149130".to_string(), output_type: 7 },
        DecryptionResult { value: "708862794572017520787824189501889310665101791528313059693660933975816556106".to_string(), output_type: 8 },
        DecryptionResult { value: "12730235082833568380334594754488662004233832431992980449804758406069160864275028020962155601245523679819346268699252919365791199191241634613862853049601610".to_string(), output_type: 9 },
        DecryptionResult { value: "108080741562249556931009330483536710085758122079454929899567131003800646884284657303009577197577435564912115080988607041335379236995946564760399287610685200564494202954787898550157622716115053586508022488999714877512770859433667427323545700100098005683083248085496846062049536709006011184102093099103655054922".to_string(), output_type: 10 },
        DecryptionResult { value: "20861508589877342907727084703622550598935352998746120842950395081753632435267183114988504993162213367166862517403829273790814314107740319854089900370739997411799274100417555415192972122100470155923419604034291807093541986108659977790911517837161376588397655175961399498725202642377614760931166783035115963547101181507813456254749873668327253075828261674997156280447164928833111149986197323155994656673039741155124831297907623530345138812274608251091216970172997278005915715893955210806849703477333174085073707097135005438554387123608323887293974874745441977610516712982266319433597579122608490187852748993192739491402".to_string(), output_type: 11 }
    ];
    #[cfg(feature = "gpu")]
    let expected: Vec<DecryptionResult> = vec![
        DecryptionResult { value: "false".to_string(), output_type: 0 },
        DecryptionResult { value: "5".to_string(), output_type: 1 },
        DecryptionResult { value: "149".to_string(), output_type: 2 },
        DecryptionResult { value: "19349".to_string(), output_type: 3 },
        DecryptionResult { value: "417483669".to_string(), output_type: 4 },
        DecryptionResult { value: "18215029179758562197".to_string(), output_type: 5 },
        DecryptionResult { value: "285171577856650533455529174907356334997".to_string(), output_type: 6 },
        DecryptionResult { value: "152905830138175504431271466872053891096341793685".to_string(), output_type: 7 },
        DecryptionResult { value: "75047619794910340784669152516253626220419211295364484836071532923723869997973".to_string(), output_type: 8 },
        DecryptionResult { value: "349780784510418262902040665399285801727082007983000585469369865909231330563294145578970577364371909994875185361545296833571683466843935904940636405648277".to_string(), output_type: 9 },
        DecryptionResult { value: "171738377129917967436345594293257284762353650530386021250612102300810915559078926240760554417297547360292892239635175923515301804258782398062670768578011802573955312346636109421853242135805160085475970193277794019696322884249110858403790775226268809271163552060562930044042204075187142693715233344518365531029".to_string(), output_type: 10 },
        DecryptionResult { value: "23011518767875069512636716445371001254752832170388493231908154456531448560534447660025296332812920678862851739752053299852838774028475881128210422000455499284435972428726433160524380665603633868755094527969467996057520428645434245018037450371155948494643475469241326170683896768567625761004528705839494428438218605230209633486159150400503849489806523016437545092095042453015740298878618948656025939925399651396176524511837723097378533063790942072701533215994079205919166519674954609602207838815187324718030081530929497091171188932094924158847373533994038323782999583833402052703292634830232831666420753273719368076181".to_string(), output_type: 11 }
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
        "false",
        "2",
        "10",
        "3658",
        "169627210",
        "3781517732040429130",
        "2926310815803454863161246203437141578",
        "363764566205121107728497043132150680907880877642",
        "708862794572017520787824189501889310665101791528313059693660933975816556106",
        "2674379135376620555654076005834277408624308066548685416512087323277837841719867788360749877620346109051822375059388381225475787082281707154037616295038538",
        "18196084819133761544544070944085473404859273132339601262852090424934308981534175736655338536373667554352058141052910362505984352588738253513975572290948138375610256242355155912006512915492006526781481012957211993093695518262435986586589144829684387101407992743203696942075913739766153031684414934291542986314",
        "4703005554221839257369646359287574618713301663888378826885222367991304865833237668389904287400756635322503536942880264043754534532494859306545824177515855851483835416767057369317873246905644102525390784842258022955145876787350099709992470667923291353106832249943246977281414696607081856627390583473150770786144014812979285044407386275044968328400173657333995251408262099167598053631967166080865360240951182792153029438676694851545963236123785298889368163556353662392352873538545105944271152613867512350234435806806656471031388489431824568186212291550723309838715645265651517110938339820132416091074822187662941376074",
    ];
    #[cfg(feature = "gpu")]
    let results = [
        "false",
        "1",
        "85",
        "2965",
        "417483669",
        "4379971124476398485",
        "29959802665946685857998219333530176405",
        "152905830138175504431271466872053891096341793685",
        "17151575176252243072883660011909672293784218962544202816342740919767305178005",
        "349780784510418262902040665399285801727082007983000585469369865909231330563294145578970577364371909994875185361545296833571683466843935904940636405648277",
        "36911392015244274356647704984080429741005377109713028295539541432511408704953203891229196425491895344452806829731630905271209477647969931193035195598406209290629392277986995464626577434870589495886157979214039693067709872492263697298355942320648381398650669047123206364081769621327355465088716097300197428117",
        "6853015732219565862279278101036025274530780835530751215842981742769120991100502213426695627051463947018492759291104290105778994453230420580666345807231357724120533745075935114649281790408807815357065708777434211919124319324124366937118403201917863259352652543223173649240108822797092856700752506277529235677261438535375462275816663007221564742378434998774384063056139623350227202524388791580896643493311093033204722652606794418579357487640119120499684409377435590305603677319544504739629287951721662983190810240601148123648190297918424839739610950799319656011198516116787250380633395527756757569642826468189569960853",
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
