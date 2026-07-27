use crate::blockchain::nonce_manager::ZamaNonceManager;
use alloy::{
    network::{Network, TransactionBuilder},
    primitives::{Address, Bytes, U64},
    providers::{
        EthCall, PendingTransactionBuilder, Provider, RootProvider,
        fillers::{BlobGasFiller, GasFiller, JoinFill, NonceManager},
    },
    rpc::json_rpc::ErrorPayload,
    transports::{RpcError, TransportErrorKind, TransportResult},
};

pub type FillersWithoutNonceManagement = JoinFill<GasFiller, BlobGasFiller>;

/// A wrapper around an `alloy` provider that recovers its nonce manager on error.
///
/// Note that the provider given by the user must not have nonce management enabled, as this
/// is done by the `NonceManagedProvider` itself.
/// Users can use the default `FillersWithoutNonceManagement` to create a provider.
pub struct NonceManagedProvider<P> {
    inner: P,
    signer_address: Address,
    nonce_manager: ZamaNonceManager,
}

impl<P> NonceManagedProvider<P> {
    pub fn new(provider: P, signer_address: Address) -> Self {
        Self {
            inner: provider,
            signer_address,
            nonce_manager: Default::default(),
        }
    }
}

impl<P: Clone> Clone for NonceManagedProvider<P> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            signer_address: self.signer_address,
            nonce_manager: self.nonce_manager.clone(),
        }
    }
}

// We forward to `self.inner` exactly the methods that rely on the wallet: the `send_*`/
// `sign_transaction` methods (which build and sign the transaction) and `call`/`estimate_gas`
// (which need the `from` field set to the signer address). Any other method left to the trait
// default reaches the same underlying node and behaves identically, so it doesn't need forwarding.
//
// If a wallet-dependent method were missing here, it would fall back to the trait default, which
// skips the fillers: `estimate_gas`, for example, would then run with `from = 0x0` instead of the
// signer address, and could revert on contracts that restrict callers. This list is complete for
// the current `alloy` version, and the unit tests below guard the methods our services rely on.
#[async_trait::async_trait]
impl<N, P> Provider<N> for NonceManagedProvider<P>
where
    N: Network,
    P: Provider<N>,
{
    fn root(&self) -> &RootProvider<N> {
        self.inner.root()
    }

    async fn send_transaction_sync(
        &self,
        mut tx: N::TransactionRequest,
    ) -> TransportResult<N::ReceiptResponse> {
        let signer_addr = self.signer_address;
        let nonce = self
            .nonce_manager
            .get_next_nonce(&self.inner, signer_addr)
            .await?;
        tx.set_nonce(nonce);

        let res = self.inner.send_transaction_sync(tx).await;
        match &res {
            Err(e) if is_nonce_too_low(e) => {
                self.nonce_manager.confirm_nonce(signer_addr, nonce).await;
            }
            Err(_) => {
                self.nonce_manager.release_nonce(signer_addr, nonce).await;
            }
            Ok(_) => self.nonce_manager.confirm_nonce(signer_addr, nonce).await,
        }

        res
    }

    async fn send_transaction(
        &self,
        mut tx: N::TransactionRequest,
    ) -> TransportResult<PendingTransactionBuilder<N>> {
        let signer_addr = self.signer_address;
        let nonce = self
            .nonce_manager
            .get_next_nonce(&self.inner, signer_addr)
            .await?;
        tx.set_nonce(nonce);

        let res = self.inner.send_transaction(tx).await;
        match &res {
            Err(e) if is_nonce_too_low(e) => {
                self.nonce_manager.confirm_nonce(signer_addr, nonce).await;
            }
            Err(_) => {
                self.nonce_manager.release_nonce(signer_addr, nonce).await;
            }
            Ok(_) => self.nonce_manager.confirm_nonce(signer_addr, nonce).await,
        }

        res
    }

    async fn sign_transaction(&self, tx: N::TransactionRequest) -> TransportResult<Bytes> {
        self.inner.sign_transaction(tx).await
    }

    fn call(&self, tx: N::TransactionRequest) -> EthCall<N, Bytes> {
        self.inner.call(tx)
    }

    fn estimate_gas(&self, tx: N::TransactionRequest) -> EthCall<N, U64, u64> {
        self.inner.estimate_gas(tx)
    }
}

// See https://ethereum-json-rpc.com/errors
const ETH_INVALID_INPUT_RPC_ERROR_CODE: i64 = -32000;

/// Returns `true` if the RPC error is "nonce too low" or "already known", `false` otherwise.
fn is_nonce_too_low(error: &RpcError<TransportErrorKind>) -> bool {
    match error {
        RpcError::ErrorResp(ErrorPayload { code, message, .. })
            if *code == ETH_INVALID_INPUT_RPC_ERROR_CODE =>
        {
            let lowercase_msg = message.to_lowercase();
            lowercase_msg.starts_with("nonce too low") || lowercase_msg.starts_with("already known")
        }
        _ => false,
    }
}
