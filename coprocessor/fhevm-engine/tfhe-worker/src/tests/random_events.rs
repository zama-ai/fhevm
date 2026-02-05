use std::str::FromStr;

use alloy::primitives::FixedBytes;
use bigdecimal::num_bigint::BigInt;
use host_listener::contracts::TfheContract;
use host_listener::contracts::TfheContract::TfheContractEvents;
use host_listener::database::tfhe_event_propagate::ClearConst;
use serial_test::serial;

use crate::tests::events::{
    allow_handle, insert_tfhe_event, listener_db, next_handle, tfhe_log, to_ty,
};
use crate::tests::utils::{
    decrypt_ciphertexts, setup_test_app, wait_until_all_allowed_handles_computed, DecryptionResult,
};

#[cfg(not(feature = "gpu"))]
fn random_supported_types() -> &'static [i32] {
    &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
}

#[cfg(feature = "gpu")]
fn random_supported_types() -> &'static [i32] {
    &[0, 1, 2, 3, 4, 5, 6, 7, 8]
}

#[cfg(not(feature = "gpu"))]
const RAND_BASIC_EXPECTED: [&str; 12] = [
    "true",
    "12",
    "156",
    "51868",
    "2619984540",
    "10012971755022961308",
    "315195588681747109104923061743488977564",
    "788883063982248783364032654023297427550759406236",
    "76547111697560389694815048035853209325312793002204865478146017769839135410844",
    "4620300173090955915208010885061878517829873825553098492220465724030076978650428492943844398737585329942351846650303374010943990494045198570449496290478748",
    "41494007511305936838300758727359299346214242261416586280906617017153184838995422253823927812442946876630762826820589205469843393333804107103032285280293973663368090988288979532934632731245358743733005968512630386343560460685593551815532487760564915903224952146254993560801533799573886208522311328831237638812",
    "19733886402355865032400462714816069368039690184989121557781677033259228589971262966675570089584500610059101607007199306637911675727319722458440947456208802538556773930772101832011802133239078178777473379921690577802297622966469875565470144225558503110585823618846573672732754957550113490674349293976082721088842023265656667800220207532026919372995977171149881718663219561963034867768017575285621955948819683133204048721069630295364245981944082975096068080088379308849511079501340419770780049292625884104832624282900045237278591546992102124172326011290396356133702950143455725743711191910217441597593339600775705905820",
];

#[cfg(feature = "gpu")]
const RAND_BASIC_EXPECTED: [&str; 9] = [
    "false",
    "12",
    "156",
    "51868",
    "2619984540",
    "10012971755022961308",
    "315195588681747109104923061743488977564",
    "788883063982248783364032654023297427550759406236",
    "76547111697560389694815048035853209325312793002204865478146017769839135410844",
];

const RAND_BOUNDED_BOUNDS: [&str; 12] = [
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
const RAND_BOUNDED_EXPECTED: [&str; 12] = [
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
const RAND_BOUNDED_EXPECTED: [&str; 9] = [
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

fn seed_bytes(seed: u8) -> FixedBytes<16> {
    let mut bytes = [0_u8; 16];
    bytes[15] = seed;
    FixedBytes::from(bytes)
}

fn as_clear_uint(big_int: &BigInt) -> ClearConst {
    let (_, bytes) = big_int.to_bytes_be();
    ClearConst::from_be_slice(&bytes)
}

#[tokio::test]
#[serial(db)]
async fn test_fhe_random_basic_events() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let listener = listener_db(&app).await;

    let caller = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();
    let deterministic_seed = 123_u8;

    let mut output_handles = Vec::new();
    let mut repeated_output_handles = Vec::new();
    let mut other_seed_output_handles = Vec::new();

    let mut tx = listener.new_transaction().await?;
    for &the_type in random_supported_types() {
        let transaction_id = next_handle();
        let output_handle = next_handle();
        output_handles.push(output_handle.to_vec());

        let log = tfhe_log(
            TfheContractEvents::FheRand(TfheContract::FheRand {
                caller,
                randType: to_ty(the_type),
                seed: seed_bytes(deterministic_seed),
                result: output_handle,
            }),
            transaction_id,
        );
        insert_tfhe_event(&listener, &mut tx, log, true).await?;
        allow_handle(&listener, &mut tx, output_handle.as_ref()).await?;
    }

    for &the_type in random_supported_types() {
        let transaction_id = next_handle();
        let output_handle = next_handle();
        repeated_output_handles.push(output_handle.to_vec());

        let log = tfhe_log(
            TfheContractEvents::FheRand(TfheContract::FheRand {
                caller,
                randType: to_ty(the_type),
                seed: seed_bytes(deterministic_seed),
                result: output_handle,
            }),
            transaction_id,
        );
        insert_tfhe_event(&listener, &mut tx, log, true).await?;
        allow_handle(&listener, &mut tx, output_handle.as_ref()).await?;
    }

    let other_seed = 124_u8;
    for &the_type in random_supported_types() {
        let transaction_id = next_handle();
        let output_handle = next_handle();
        other_seed_output_handles.push(output_handle.to_vec());

        let log = tfhe_log(
            TfheContractEvents::FheRand(TfheContract::FheRand {
                caller,
                randType: to_ty(the_type),
                seed: seed_bytes(other_seed),
                result: output_handle,
            }),
            transaction_id,
        );
        insert_tfhe_event(&listener, &mut tx, log, true).await?;
        allow_handle(&listener, &mut tx, output_handle.as_ref()).await?;
    }
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&app).await?;

    let resp = decrypt_ciphertexts(&pool, 1, output_handles).await?;
    let expected: Vec<DecryptionResult> = RAND_BASIC_EXPECTED
        .iter()
        .enumerate()
        .map(|(idx, value)| DecryptionResult {
            value: (*value).to_string(),
            output_type: random_supported_types()[idx] as i16,
        })
        .collect();
    assert_eq!(expected, resp);

    let resp_repeated = decrypt_ciphertexts(&pool, 1, repeated_output_handles).await?;
    assert_eq!(resp, resp_repeated, "randomness is not deterministic");

    let resp_other_seed = decrypt_ciphertexts(&pool, 1, other_seed_output_handles).await?;
    assert_ne!(resp, resp_other_seed, "seed changed, values should differ");

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn test_fhe_random_bounded_events() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;
    let listener = listener_db(&app).await;

    let caller = "0x0000000000000000000000000000000000000000"
        .parse()
        .unwrap();
    let deterministic_seed = 123_u8;

    let mut output_handles = Vec::new();
    let mut tx = listener.new_transaction().await?;

    for (idx, &the_type) in random_supported_types().iter().enumerate() {
        let transaction_id = next_handle();
        let output_handle = next_handle();
        output_handles.push(output_handle.to_vec());

        let log = tfhe_log(
            TfheContractEvents::FheRandBounded(TfheContract::FheRandBounded {
                caller,
                upperBound: as_clear_uint(&BigInt::from_str(RAND_BOUNDED_BOUNDS[idx]).unwrap()),
                randType: to_ty(the_type),
                seed: seed_bytes(deterministic_seed),
                result: output_handle,
            }),
            transaction_id,
        );
        insert_tfhe_event(&listener, &mut tx, log, true).await?;
        allow_handle(&listener, &mut tx, output_handle.as_ref()).await?;
    }
    tx.commit().await?;

    wait_until_all_allowed_handles_computed(&app).await?;

    let resp = decrypt_ciphertexts(&pool, 1, output_handles).await?;
    assert_eq!(resp.len(), RAND_BOUNDED_EXPECTED.len());

    for idx in 0..RAND_BOUNDED_EXPECTED.len() {
        assert_eq!(resp[idx].output_type, random_supported_types()[idx] as i16);
        assert_eq!(resp[idx].value, RAND_BOUNDED_EXPECTED[idx]);
        if resp[idx].output_type > 0 {
            assert!(
                BigInt::from_str(RAND_BOUNDED_BOUNDS[idx])
                    .unwrap()
                    .gt(&BigInt::from_str(&resp[idx].value).unwrap()),
                "value must be < bound"
            );
        }
    }

    Ok(())
}
