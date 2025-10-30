use alloy::{
    consensus::Account,
    eips::{BlockId, BlockNumberOrTag, Encodable2718},
    network::{Ethereum, Network, TransactionBuilder},
    primitives::{
        Address, B256, BlockHash, BlockNumber, Bytes, StorageKey, StorageValue, TxHash, U64, U128,
        U256,
    },
    providers::{
        EthCall, EthCallMany, EthGetBlock, FilterPollerBuilder, GetSubscription,
        PendingTransaction, PendingTransactionBuilder, PendingTransactionConfig,
        PendingTransactionError, Provider, ProviderCall, RootProvider, RpcWithBlock, SendableTx,
        fillers::{
            BlobGasFiller, CachedNonceManager, FillProvider, GasFiller, JoinFill, NonceManager,
            TxFiller,
        },
    },
    rpc::{
        client::NoParams,
        types::{
            AccessListResult, Bundle, EIP1186AccountProofResponse, EthCallResponse, FeeHistory,
            Filter, FilterChanges, Index, Log, SyncStatus, TransactionReceipt, TransactionRequest,
            erc4337::TransactionConditional,
            pubsub::{Params, SubscriptionKind},
            simulate::{SimulatePayload, SimulatedBlock},
        },
    },
    transports::{TransportError, TransportResult},
};
use futures::lock::Mutex;
use serde_json::value::RawValue;
use std::{borrow::Cow, sync::Arc};

pub type FillersWithoutNonceManagement = JoinFill<GasFiller, BlobGasFiller>;

/// A wrapper around an `alloy` provider that recovers its nonce manager on error.
///
/// Note that the provider given by the user must not have nonce management enabled, as this
/// is done by the `NonceManagedProvider` itself.
/// Users can use the default `FillersWithoutNonceManagement` to create a provider.
pub struct NonceManagedProvider<F, P, N = Ethereum>
where
    N: Network,
    F: TxFiller<N>,
    P: Provider<N>,
{
    inner: FillProvider<F, P, N>,
    signer_address: Address,
    nonce_manager: Arc<Mutex<CachedNonceManager>>,
}

impl<F, P> NonceManagedProvider<F, P>
where
    F: TxFiller<Ethereum>,
    P: Provider<Ethereum>,
{
    pub fn new(provider: FillProvider<F, P, Ethereum>, signer_address: Address) -> Self {
        Self {
            inner: provider,
            signer_address,
            nonce_manager: Default::default(),
        }
    }

    pub async fn send_transaction_sync(
        &self,
        mut tx: TransactionRequest,
    ) -> TransportResult<TransactionReceipt> {
        let nonce = self
            .nonce_manager
            .lock()
            .await
            .get_next_nonce(&self.inner, self.signer_address)
            .await?;
        tx.set_nonce(nonce);

        let mut tx_bytes = Vec::new();
        self.inner
            .fill(tx)
            .await?
            .try_into_envelope()
            .map_err(|e| TransportError::LocalUsageError(Box::new(e)))?
            .encode_2718(&mut tx_bytes);

        let res = self
            .client()
            .request("eth_sendRawTransactionSync", (Bytes::from(tx_bytes),))
            .await;
        if res.is_err() {
            // Reset the nonce manager if the transaction sending failed.
            *self.nonce_manager.lock().await = Default::default();
        }
        res
    }
}

impl<F, P, N> Clone for NonceManagedProvider<F, P, N>
where
    N: Network,
    F: TxFiller<N>,
    P: Provider<N> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            signer_address: self.signer_address,
            nonce_manager: self.nonce_manager.clone(),
        }
    }
}

#[async_trait::async_trait]
impl<F, P, N> Provider<N> for NonceManagedProvider<F, P, N>
where
    N: Network,
    F: TxFiller<N>,
    P: Provider<N>,
{
    fn root(&self) -> &RootProvider<N> {
        self.inner.root()
    }

    async fn send_transaction(
        &self,
        mut tx: N::TransactionRequest,
    ) -> TransportResult<PendingTransactionBuilder<N>> {
        let nonce = self
            .nonce_manager
            .lock()
            .await
            .get_next_nonce(&self.inner, self.signer_address)
            .await?;
        tx.set_nonce(nonce);
        let res = self.inner.send_transaction(tx).await;
        if res.is_err() {
            // Reset the nonce manager if the transaction sending failed.
            *self.nonce_manager.lock().await = Default::default();
        }
        res
    }

    fn get_accounts(&self) -> ProviderCall<NoParams, Vec<Address>> {
        self.inner.get_accounts()
    }

    fn get_blob_base_fee(&self) -> ProviderCall<NoParams, U128, u128> {
        self.inner.get_blob_base_fee()
    }

    fn get_block_number(&self) -> ProviderCall<NoParams, U64, BlockNumber> {
        self.inner.get_block_number()
    }

    fn call<'req>(&self, tx: N::TransactionRequest) -> EthCall<N, Bytes> {
        self.inner.call(tx)
    }

    fn call_many<'req>(
        &self,
        bundles: &'req [Bundle],
    ) -> EthCallMany<'req, N, Vec<Vec<EthCallResponse>>> {
        self.inner.call_many(bundles)
    }

    fn simulate<'req>(
        &self,
        payload: &'req SimulatePayload,
    ) -> RpcWithBlock<&'req SimulatePayload, Vec<SimulatedBlock<N::BlockResponse>>> {
        self.inner.simulate(payload)
    }

    fn get_chain_id(&self) -> ProviderCall<NoParams, U64, u64> {
        self.inner.get_chain_id()
    }

    fn create_access_list<'a>(
        &self,
        request: &'a N::TransactionRequest,
    ) -> RpcWithBlock<&'a N::TransactionRequest, AccessListResult> {
        self.inner.create_access_list(request)
    }

    fn estimate_gas<'req>(&self, tx: N::TransactionRequest) -> EthCall<N, U64, u64> {
        self.inner.estimate_gas(tx)
    }

    async fn get_fee_history(
        &self,
        block_count: u64,
        last_block: BlockNumberOrTag,
        reward_percentiles: &[f64],
    ) -> TransportResult<FeeHistory> {
        self.inner
            .get_fee_history(block_count, last_block, reward_percentiles)
            .await
    }

    fn get_gas_price(&self) -> ProviderCall<NoParams, U128, u128> {
        self.inner.get_gas_price()
    }

    fn get_account(&self, address: Address) -> RpcWithBlock<Address, Account> {
        self.inner.get_account(address)
    }

    fn get_balance(&self, address: Address) -> RpcWithBlock<Address, U256, U256> {
        self.inner.get_balance(address)
    }

    fn get_block(&self, block: BlockId) -> EthGetBlock<N::BlockResponse> {
        self.inner.get_block(block)
    }

    fn get_block_by_hash(&self, hash: BlockHash) -> EthGetBlock<N::BlockResponse> {
        self.inner.get_block_by_hash(hash)
    }

    fn get_block_by_number(&self, number: BlockNumberOrTag) -> EthGetBlock<N::BlockResponse> {
        self.inner.get_block_by_number(number)
    }

    async fn get_block_transaction_count_by_hash(
        &self,
        hash: BlockHash,
    ) -> TransportResult<Option<u64>> {
        self.inner.get_block_transaction_count_by_hash(hash).await
    }

    async fn get_block_transaction_count_by_number(
        &self,
        block_number: BlockNumberOrTag,
    ) -> TransportResult<Option<u64>> {
        self.inner
            .get_block_transaction_count_by_number(block_number)
            .await
    }

    fn get_block_receipts(
        &self,
        block: BlockId,
    ) -> ProviderCall<(BlockId,), Option<Vec<N::ReceiptResponse>>> {
        self.inner.get_block_receipts(block)
    }

    fn get_code_at(&self, address: Address) -> RpcWithBlock<Address, Bytes> {
        self.inner.get_code_at(address)
    }

    async fn watch_blocks(&self) -> TransportResult<FilterPollerBuilder<B256>> {
        self.inner.watch_blocks().await
    }

    async fn watch_pending_transactions(&self) -> TransportResult<FilterPollerBuilder<B256>> {
        self.inner.watch_pending_transactions().await
    }

    async fn watch_logs(&self, filter: &Filter) -> TransportResult<FilterPollerBuilder<Log>> {
        self.inner.watch_logs(filter).await
    }

    async fn watch_full_pending_transactions(
        &self,
    ) -> TransportResult<FilterPollerBuilder<N::TransactionResponse>> {
        self.inner.watch_full_pending_transactions().await
    }

    async fn get_filter_changes_dyn(&self, id: U256) -> TransportResult<FilterChanges> {
        self.inner.get_filter_changes_dyn(id).await
    }

    async fn get_filter_logs(&self, id: U256) -> TransportResult<Vec<Log>> {
        self.inner.get_filter_logs(id).await
    }

    async fn uninstall_filter(&self, id: U256) -> TransportResult<bool> {
        self.inner.uninstall_filter(id).await
    }

    async fn watch_pending_transaction(
        &self,
        config: PendingTransactionConfig,
    ) -> Result<PendingTransaction, PendingTransactionError> {
        self.inner.watch_pending_transaction(config).await
    }

    async fn get_logs(&self, filter: &Filter) -> TransportResult<Vec<Log>> {
        self.inner.get_logs(filter).await
    }

    fn get_proof(
        &self,
        address: Address,
        keys: Vec<StorageKey>,
    ) -> RpcWithBlock<(Address, Vec<StorageKey>), EIP1186AccountProofResponse> {
        self.inner.get_proof(address, keys)
    }

    fn get_storage_at(
        &self,
        address: Address,
        key: U256,
    ) -> RpcWithBlock<(Address, U256), StorageValue> {
        self.inner.get_storage_at(address, key)
    }

    fn get_transaction_by_hash(
        &self,
        hash: TxHash,
    ) -> ProviderCall<(TxHash,), Option<N::TransactionResponse>> {
        self.inner.get_transaction_by_hash(hash)
    }

    fn get_transaction_by_block_hash_and_index(
        &self,
        block_hash: B256,
        index: usize,
    ) -> ProviderCall<(B256, Index), Option<N::TransactionResponse>> {
        self.inner
            .get_transaction_by_block_hash_and_index(block_hash, index)
    }

    fn get_raw_transaction_by_block_hash_and_index(
        &self,
        block_hash: B256,
        index: usize,
    ) -> ProviderCall<(B256, Index), Option<Bytes>> {
        self.inner
            .get_raw_transaction_by_block_hash_and_index(block_hash, index)
    }

    fn get_transaction_by_block_number_and_index(
        &self,
        block_number: BlockNumberOrTag,
        index: usize,
    ) -> ProviderCall<(BlockNumberOrTag, Index), Option<N::TransactionResponse>> {
        self.inner
            .get_transaction_by_block_number_and_index(block_number, index)
    }

    fn get_raw_transaction_by_block_number_and_index(
        &self,
        block_number: BlockNumberOrTag,
        index: usize,
    ) -> ProviderCall<(BlockNumberOrTag, Index), Option<Bytes>> {
        self.inner
            .get_raw_transaction_by_block_number_and_index(block_number, index)
    }

    fn get_raw_transaction_by_hash(&self, hash: TxHash) -> ProviderCall<(TxHash,), Option<Bytes>> {
        self.inner.get_raw_transaction_by_hash(hash)
    }

    fn get_transaction_count(
        &self,
        address: Address,
    ) -> RpcWithBlock<Address, U64, u64, fn(U64) -> u64> {
        self.inner.get_transaction_count(address)
    }

    fn get_transaction_receipt(
        &self,
        hash: TxHash,
    ) -> ProviderCall<(TxHash,), Option<N::ReceiptResponse>> {
        self.inner.get_transaction_receipt(hash)
    }

    async fn get_uncle(&self, tag: BlockId, idx: u64) -> TransportResult<Option<N::BlockResponse>> {
        self.inner.get_uncle(tag, idx).await
    }

    async fn get_uncle_count(&self, tag: BlockId) -> TransportResult<u64> {
        self.inner.get_uncle_count(tag).await
    }

    fn get_max_priority_fee_per_gas(&self) -> ProviderCall<NoParams, U128, u128> {
        self.inner.get_max_priority_fee_per_gas()
    }

    async fn new_block_filter(&self) -> TransportResult<U256> {
        self.inner.new_block_filter().await
    }

    async fn new_filter(&self, filter: &Filter) -> TransportResult<U256> {
        self.inner.new_filter(filter).await
    }

    async fn new_pending_transactions_filter(&self, full: bool) -> TransportResult<U256> {
        self.inner.new_pending_transactions_filter(full).await
    }

    async fn send_raw_transaction(
        &self,
        encoded_tx: &[u8],
    ) -> TransportResult<PendingTransactionBuilder<N>> {
        self.inner.send_raw_transaction(encoded_tx).await
    }

    async fn send_raw_transaction_conditional(
        &self,
        encoded_tx: &[u8],
        conditional: TransactionConditional,
    ) -> TransportResult<PendingTransactionBuilder<N>> {
        self.inner
            .send_raw_transaction_conditional(encoded_tx, conditional)
            .await
    }

    async fn send_transaction_internal(
        &self,
        tx: SendableTx<N>,
    ) -> TransportResult<PendingTransactionBuilder<N>> {
        self.inner.send_transaction_internal(tx).await
    }

    async fn sign_transaction(&self, tx: N::TransactionRequest) -> TransportResult<Bytes> {
        self.inner.sign_transaction(tx).await
    }

    fn subscribe_blocks(&self) -> GetSubscription<(SubscriptionKind,), N::HeaderResponse> {
        self.inner.subscribe_blocks()
    }

    fn subscribe_pending_transactions(&self) -> GetSubscription<(SubscriptionKind,), B256> {
        self.inner.subscribe_pending_transactions()
    }

    fn subscribe_full_pending_transactions(
        &self,
    ) -> GetSubscription<(SubscriptionKind, Params), N::TransactionResponse> {
        self.inner.subscribe_full_pending_transactions()
    }

    fn subscribe_logs(&self, filter: &Filter) -> GetSubscription<(SubscriptionKind, Params), Log> {
        self.inner.subscribe_logs(filter)
    }

    async fn unsubscribe(&self, id: B256) -> TransportResult<()> {
        self.inner.unsubscribe(id).await
    }

    fn syncing(&self) -> ProviderCall<NoParams, SyncStatus> {
        self.inner.syncing()
    }

    fn get_client_version(&self) -> ProviderCall<NoParams, String> {
        self.inner.get_client_version()
    }

    fn get_sha3(&self, data: &[u8]) -> ProviderCall<(String,), B256> {
        self.inner.get_sha3(data)
    }

    fn get_net_version(&self) -> ProviderCall<NoParams, U64, u64> {
        self.inner.get_net_version()
    }

    async fn raw_request_dyn(
        &self,
        method: Cow<'static, str>,
        params: &RawValue,
    ) -> TransportResult<Box<RawValue>> {
        self.inner.raw_request_dyn(method, params).await
    }

    fn transaction_request(&self) -> N::TransactionRequest {
        self.inner.transaction_request()
    }
}
