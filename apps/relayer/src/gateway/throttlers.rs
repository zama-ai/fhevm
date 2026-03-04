use tokio::sync::mpsc;

use crate::{
    config::settings::Settings,
    gateway::arbitrum::transaction::tx_throttler::{
        GatewayTxTask, TxSenders, TxThrottlers, TxThrottlingSender, TxThrottlingType,
    },
    readiness::throttler::{
        DelegatedUserDecryptReadinessTask, PublicDecryptReadinessTask, ReadinessSender,
        ReadinessSenders, ReadinessThrottlers, ReadinessThrottlingType, UserDecryptReadinessTask,
    },
};

pub struct GatewayThrottlers {
    pub tx_throttlers: TxThrottlers,
    pub readiness_throttlers: ReadinessThrottlers,
}

impl GatewayThrottlers {
    pub fn new(tx_throttlers: TxThrottlers, readiness_throttlers: ReadinessThrottlers) -> Self {
        Self {
            tx_throttlers,
            readiness_throttlers,
        }
    }
}

pub struct BouncerThrottlers {
    pub input_proof_throttler_control_tx: Option<mpsc::Sender<u32>>,
    pub user_decrypt_throttler_control_tx: Option<mpsc::Sender<u32>>,
    pub public_decrypt_throttler_control_tx: Option<mpsc::Sender<u32>>,
    pub tx_throttlers: TxSenders,
    pub readiness_throttling_senders: ReadinessSenders,
}

impl BouncerThrottlers {
    pub fn new(
        input_proof_throttler_control_tx: Option<mpsc::Sender<u32>>,
        user_decrypt_throttler_control_tx: Option<mpsc::Sender<u32>>,
        public_decrypt_throttler_control_tx: Option<mpsc::Sender<u32>>,
        tx_throttlers: TxSenders,
        readiness_throttling_senders: ReadinessSenders,
    ) -> Self {
        Self {
            input_proof_throttler_control_tx,
            user_decrypt_throttler_control_tx,
            public_decrypt_throttler_control_tx,
            tx_throttlers,
            readiness_throttling_senders,
        }
    }
}

pub fn init_throttlers(settings: &Settings) -> (GatewayThrottlers, BouncerThrottlers) {
    let (input_proof_tx_throttler, input_proof_tx_worker, throttler_control_input_proof) =
        TxThrottlingSender::<GatewayTxTask>::new(
            TxThrottlingType::InputProof,
            settings
                .gateway
                .tx_engine
                .tx_throttlers
                .input_proof
                .capacity,
            settings
                .gateway
                .tx_engine
                .tx_throttlers
                .input_proof
                .safety_margin,
            settings
                .gateway
                .tx_engine
                .tx_throttlers
                .input_proof
                .per_seconds,
            settings.http.enable_admin_endpoint,
        );

    let (user_decrypt_tx_throttler, user_decrypt_tx_worker, throttler_control_user_decrypt) =
        TxThrottlingSender::<GatewayTxTask>::new(
            TxThrottlingType::UserDecrypt,
            settings
                .gateway
                .tx_engine
                .tx_throttlers
                .user_decrypt
                .capacity,
            settings
                .gateway
                .tx_engine
                .tx_throttlers
                .user_decrypt
                .safety_margin,
            settings
                .gateway
                .tx_engine
                .tx_throttlers
                .user_decrypt
                .per_seconds,
            settings.http.enable_admin_endpoint,
        );

    let (public_decrypt_tx_throttler, public_decrypt_tx_worker, throttler_control_public_decrypt) =
        TxThrottlingSender::<GatewayTxTask>::new(
            TxThrottlingType::PublicDecrypt,
            settings
                .gateway
                .tx_engine
                .tx_throttlers
                .public_decrypt
                .capacity,
            settings
                .gateway
                .tx_engine
                .tx_throttlers
                .public_decrypt
                .safety_margin,
            settings
                .gateway
                .tx_engine
                .tx_throttlers
                .public_decrypt
                .per_seconds,
            settings.http.enable_admin_endpoint,
        );

    let (public_decrypt_readiness_throttler, public_decrypt_readiness_worker) =
        ReadinessSender::<PublicDecryptReadinessTask>::new(
            ReadinessThrottlingType::PublicDecrypt,
            settings.gateway.readiness_checker.public_decrypt.capacity,
            settings
                .gateway
                .readiness_checker
                .public_decrypt
                .safety_margin,
            settings
                .gateway
                .readiness_checker
                .public_decrypt
                .max_concurrency,
        );

    let (user_decrypt_readiness_throttler, user_decrypt_readiness_worker) =
        ReadinessSender::<UserDecryptReadinessTask>::new(
            ReadinessThrottlingType::UserDecrypt,
            settings.gateway.readiness_checker.user_decrypt.capacity,
            settings
                .gateway
                .readiness_checker
                .user_decrypt
                .safety_margin,
            settings
                .gateway
                .readiness_checker
                .user_decrypt
                .max_concurrency,
        );

    let (delegated_user_decrypt_readiness_throttler, delegated_user_decrypt_readiness_worker) =
        ReadinessSender::<DelegatedUserDecryptReadinessTask>::new(
            ReadinessThrottlingType::UserDecrypt,
            settings
                .gateway
                .readiness_checker
                .delegated_user_decrypt
                .capacity,
            settings
                .gateway
                .readiness_checker
                .delegated_user_decrypt
                .safety_margin,
            settings
                .gateway
                .readiness_checker
                .delegated_user_decrypt
                .max_concurrency,
        );

    let tx_throttlers = TxThrottlers::new(
        input_proof_tx_throttler.clone(),
        input_proof_tx_worker,
        user_decrypt_tx_throttler.clone(),
        user_decrypt_tx_worker,
        public_decrypt_tx_throttler.clone(),
        public_decrypt_tx_worker,
    );

    let readiness_throttlers = ReadinessThrottlers::new(
        user_decrypt_readiness_throttler.clone(),
        user_decrypt_readiness_worker,
        delegated_user_decrypt_readiness_throttler.clone(),
        delegated_user_decrypt_readiness_worker,
        public_decrypt_readiness_throttler.clone(),
        public_decrypt_readiness_worker,
    );

    let tx_throttling_senders = TxSenders::new(
        input_proof_tx_throttler.clone(),
        user_decrypt_tx_throttler.clone(),
        public_decrypt_tx_throttler.clone(),
    );

    let readiness_throttling_senders = ReadinessSenders::new(
        user_decrypt_readiness_throttler.clone(),
        delegated_user_decrypt_readiness_throttler.clone(),
        public_decrypt_readiness_throttler.clone(),
    );

    let gateway_throttlers = GatewayThrottlers::new(tx_throttlers, readiness_throttlers);

    let bouncer_throttlers = BouncerThrottlers::new(
        throttler_control_input_proof,
        throttler_control_user_decrypt,
        throttler_control_public_decrypt,
        tx_throttling_senders,
        readiness_throttling_senders,
    );

    (gateway_throttlers, bouncer_throttlers)
}
