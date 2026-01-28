use crate::gateway::{
    arbitrum::transaction::tx_throttler::{GatewayTxTask, TxThrottlingSender},
    readiness_check::readiness_throttler::{
        DelegatedUserDecryptReadinessTask, PublicDecryptReadinessTask, ReadinessSender,
        UserDecryptReadinessTask,
    },
};

pub async fn bounce_check(tx_throttler: TxThrottlingSender<GatewayTxTask>) -> bool {
    let tx_full = tx_throttler.is_queue_full().await;
    tx_full
}

pub async fn public_decrypt_bounce_check(
    tx_throttler: TxThrottlingSender<GatewayTxTask>,
    public_decrypt_readiness_throttler: ReadinessSender<PublicDecryptReadinessTask>,
) -> bool {
    let tx_full = tx_throttler.is_queue_full().await;
    let pub_dec_full = public_decrypt_readiness_throttler.is_queue_full().await;
    tx_full || pub_dec_full
}

pub async fn user_decrypt_bounce_check(
    tx_throttler: TxThrottlingSender<GatewayTxTask>,
    user_decrypt_readiness_throttler: ReadinessSender<UserDecryptReadinessTask>,
) -> bool {
    let tx_full = tx_throttler.is_queue_full().await;
    let user_dec_full = user_decrypt_readiness_throttler.is_queue_full().await;
    tx_full || user_dec_full
}

pub async fn delegated_user_decrypt_bounce_check(
    tx_throttler: TxThrottlingSender<GatewayTxTask>,
    delegated_user_decrypt_readiness_throttler: ReadinessSender<DelegatedUserDecryptReadinessTask>,
) -> bool {
    let tx_full = tx_throttler.is_queue_full().await;
    let delegated_user_dec_full = delegated_user_decrypt_readiness_throttler
        .is_queue_full()
        .await;
    tx_full || delegated_user_dec_full
}
