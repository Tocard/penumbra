/// Requests a range of compact block data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactBlockRangeRequest {
    /// The expected chain id (empty string if no expectation).
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// The start height of the range.
    #[prost(uint64, tag = "2")]
    pub start_height: u64,
    /// The end height of the range, defaults to the latest block height.
    #[prost(uint64, tag = "3")]
    pub end_height: u64,
    /// If set, keeps the connection alive past `end_height`,
    /// streaming new compact blocks as they are created.
    #[prost(bool, tag = "4")]
    pub keep_alive: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactBlockRangeResponse {
    #[prost(message, optional, tag = "1")]
    pub compact_block: ::core::option::Option<
        super::super::core::chain::v1alpha1::CompactBlock,
    >,
}
/// Requests the global configuration data for the chain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainParametersRequest {
    /// The expected chain id (empty string if no expectation).
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainParametersResponse {
    #[prost(message, optional, tag = "1")]
    pub chain_parameters: ::core::option::Option<
        super::super::core::chain::v1alpha1::ChainParameters,
    >,
}
/// Requests the governance-mutable parameters available for the chain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutableParametersRequest {
    /// The expected chain id (empty string if no expectation).
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutableParametersResponse {
    #[prost(message, optional, tag = "1")]
    pub chain_parameter: ::core::option::Option<
        super::super::core::governance::v1alpha1::MutableChainParameter,
    >,
}
/// Requests information on the chain's validators.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorInfoRequest {
    /// The expected chain id (empty string if no expectation).
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// Whether or not to return inactive validators
    #[prost(bool, tag = "2")]
    pub show_inactive: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub validator_info: ::core::option::Option<
        super::super::core::stake::v1alpha1::ValidatorInfo,
    >,
}
/// Lists all assets in Asset Registry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetListRequest {
    /// The expected chain id (empty string if no expectation).
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetListResponse {
    /// TODO: deprecate in favor of SpecificQuery.AssetInfo
    #[prost(message, optional, tag = "1")]
    pub asset_list: ::core::option::Option<
        super::super::core::chain::v1alpha1::KnownAssets,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionByNoteRequest {
    #[prost(message, optional, tag = "1")]
    pub note_commitment: ::core::option::Option<
        super::super::core::crypto::v1alpha1::StateCommitment,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionByNoteResponse {
    #[prost(message, optional, tag = "1")]
    pub note_source: ::core::option::Option<
        super::super::core::chain::v1alpha1::NoteSource,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorStatusRequest {
    /// The expected chain id (empty string if no expectation).
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub identity_key: ::core::option::Option<
        super::super::core::crypto::v1alpha1::IdentityKey,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorStatusResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<
        super::super::core::stake::v1alpha1::ValidatorStatus,
    >,
}
/// Requests the compounded penalty for a validator over a range of epochs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorPenaltyRequest {
    /// The expected chain id (empty string if no expectation).
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub identity_key: ::core::option::Option<
        super::super::core::crypto::v1alpha1::IdentityKey,
    >,
    #[prost(uint64, tag = "3")]
    pub start_epoch_index: u64,
    #[prost(uint64, tag = "4")]
    pub end_epoch_index: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorPenaltyResponse {
    #[prost(message, optional, tag = "1")]
    pub penalty: ::core::option::Option<super::super::core::stake::v1alpha1::Penalty>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NextValidatorRateRequest {
    #[prost(message, optional, tag = "1")]
    pub identity_key: ::core::option::Option<
        super::super::core::crypto::v1alpha1::IdentityKey,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NextValidatorRateResponse {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<super::super::core::stake::v1alpha1::RateData>,
}
/// Requests batch swap data associated with a given height and trading pair from the view service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchSwapOutputDataRequest {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(message, optional, tag = "2")]
    pub trading_pair: ::core::option::Option<
        super::super::core::dex::v1alpha1::TradingPair,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchSwapOutputDataResponse {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<
        super::super::core::dex::v1alpha1::BatchSwapOutputData,
    >,
}
/// Requests CPMM reserves data associated with a given trading pair from the view service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StubCpmmReservesRequest {
    #[prost(message, optional, tag = "1")]
    pub trading_pair: ::core::option::Option<
        super::super::core::dex::v1alpha1::TradingPair,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StubCpmmReservesResponse {
    #[prost(message, optional, tag = "1")]
    pub reserves: ::core::option::Option<super::super::core::dex::v1alpha1::Reserves>,
}
/// Requests information on an asset by asset id
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetInfoRequest {
    /// The expected chain id (empty string if no expectation).
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// The asset id to request information on.
    #[prost(message, optional, tag = "2")]
    pub asset_id: ::core::option::Option<super::super::core::crypto::v1alpha1::AssetId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetInfoResponse {
    /// If present, information on the requested asset.
    ///
    /// If the requested asset was unknown, this field will not be present.
    #[prost(message, optional, tag = "1")]
    pub asset: ::core::option::Option<super::super::core::crypto::v1alpha1::Asset>,
}
/// Performs a key-value query, either by key or by key hash.
///
/// Proofs are only supported by key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValueRequest {
    /// The expected chain id (empty string if no expectation).
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// If set, the key to fetch from storage.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// whether to return a proof
    #[prost(bool, tag = "3")]
    pub proof: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValueResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub proof: ::core::option::Option<
        ::ibc_proto::ibc::core::commitment::v1::MerkleProof,
    >,
}
/// Performs a prefixed key-value query, by string prefix.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrefixValueRequest {
    /// The expected chain id (empty string if no expectation).
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// The prefix to fetch subkeys from storage.
    #[prost(string, tag = "2")]
    pub prefix: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrefixValueResponse {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// GetTxRequest is the request type for the GetTx RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxRequest {
    /// Hash of transaction to retrieve
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// Include proofs of the transaction's inclusion in the block
    #[prost(bool, tag = "2")]
    pub prove: bool,
}
/// GetTxResponse is the response type for the GetTx RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTxResponse {
    /// Hash of transaction
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(uint64, tag = "3")]
    pub index: u64,
    #[prost(message, optional, tag = "4")]
    pub tx_result: ::core::option::Option<TxResult>,
    #[prost(bytes = "vec", tag = "5")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxResult {
    #[prost(string, tag = "1")]
    pub log: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub gas_wanted: u64,
    #[prost(uint64, tag = "3")]
    pub gas_used: u64,
    #[prost(message, repeated, tag = "4")]
    pub tags: ::prost::alloc::vec::Vec<Tag>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "3")]
    pub index: bool,
}
/// BroadcastTxAsyncRequest is the request type for the BroadcastTxAsync RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastTxAsyncRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub params: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub req_id: u64,
}
/// BroadcastTxAsyncResponse is the response type for the BroadcastTxAsync RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastTxAsyncResponse {
    #[prost(uint64, tag = "1")]
    pub code: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub log: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// BroadcastTxSyncRequest is the request type for the BroadcastTxSync RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastTxSyncRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub params: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub req_id: u64,
}
/// BroadcastTxSyncResponse is the response type for the BroadcastTxSync RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastTxSyncResponse {
    #[prost(uint64, tag = "1")]
    pub code: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub log: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// GetStatusRequest is the request type for the Query/GetStatus RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusRequest {}
/// GetStatusResponse is the response type for the Query/GetStatus RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusResponse {
    #[prost(message, optional, tag = "1")]
    pub node_info: ::core::option::Option<
        super::super::super::tendermint::p2p::DefaultNodeInfo,
    >,
    #[prost(message, optional, tag = "2")]
    pub sync_info: ::core::option::Option<SyncInfo>,
    #[prost(message, optional, tag = "3")]
    pub validator_info: ::core::option::Option<
        super::super::super::tendermint::types::Validator,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncInfo {
    #[prost(bytes = "vec", tag = "1")]
    pub latest_block_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub latest_app_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub latest_block_height: u64,
    #[prost(message, optional, tag = "4")]
    pub latest_block_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// These are implemented in tendermint, but not
    /// in tendermint-rpc.
    /// bytes earliest_block_hash = 5;
    /// bytes earliest_app_hash = 6;
    /// uint64 earliest_block_height = 7;
    /// google.protobuf.Timestamp earliest_block_time = 8;
    #[prost(bool, tag = "9")]
    pub catching_up: bool,
}
/// ABCIQueryRequest defines the request structure for the ABCIQuery gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbciQueryRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub height: i64,
    #[prost(bool, tag = "4")]
    pub prove: bool,
}
/// ABCIQueryResponse defines the response structure for the ABCIQuery gRPC query.
///
/// Note: This type is a duplicate of the ResponseQuery proto type defined in
/// Tendermint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbciQueryResponse {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    /// nondeterministic
    #[prost(string, tag = "3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub index: i64,
    #[prost(bytes = "vec", tag = "6")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "8")]
    pub proof_ops: ::core::option::Option<
        super::super::super::tendermint::crypto::ProofOps,
    >,
    #[prost(int64, tag = "9")]
    pub height: i64,
    #[prost(string, tag = "10")]
    pub codespace: ::prost::alloc::string::String,
}
/// GetBlockByHeightRequest is the request type for the Query/GetBlockByHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockByHeightRequest {
    #[prost(int64, tag = "1")]
    pub height: i64,
}
/// GetBlockByHeightResponse is the response type for the Query/GetBlockByHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockByHeightResponse {
    #[prost(message, optional, tag = "1")]
    pub block_id: ::core::option::Option<
        super::super::super::tendermint::types::BlockId,
    >,
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<super::super::super::tendermint::types::Block>,
}
/// Generated client implementations.
#[cfg(feature = "rpc")]
pub mod oblivious_query_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Methods for accessing chain state that are "oblivious" in the sense that they
    /// do not request specific portions of the chain state that could reveal private
    /// client data.  For instance, requesting all asset denominations is oblivious,
    /// but requesting the asset denomination for a specific asset id is not, because
    /// it reveals that the client has an interest in that asset specifically.
    #[derive(Debug, Clone)]
    pub struct ObliviousQueryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ObliviousQueryServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ObliviousQueryServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ObliviousQueryServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ObliviousQueryServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn compact_block_range(
            &mut self,
            request: impl tonic::IntoRequest<super::CompactBlockRangeRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CompactBlockRangeResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.ObliviousQueryService/CompactBlockRange",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn chain_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::ChainParametersRequest>,
        ) -> Result<tonic::Response<super::ChainParametersResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.ObliviousQueryService/ChainParameters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn mutable_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::MutableParametersRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::MutableParametersResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.ObliviousQueryService/MutableParameters",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn validator_info(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidatorInfoRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ValidatorInfoResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.ObliviousQueryService/ValidatorInfo",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn asset_list(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetListRequest>,
        ) -> Result<tonic::Response<super::AssetListResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.ObliviousQueryService/AssetList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
#[cfg(feature = "rpc")]
pub mod specific_query_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Methods for accessing chain state that are "specific" in the sense that they
    /// request specific portions of the chain state that could reveal private
    /// client data.  For instance, requesting all asset denominations is oblivious,
    /// but requesting the asset denomination for a specific asset id is not, because
    /// it reveals that the client has an interest in that asset specifically.
    #[derive(Debug, Clone)]
    pub struct SpecificQueryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SpecificQueryServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SpecificQueryServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SpecificQueryServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SpecificQueryServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn transaction_by_note(
            &mut self,
            request: impl tonic::IntoRequest<super::TransactionByNoteRequest>,
        ) -> Result<tonic::Response<super::TransactionByNoteResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.SpecificQueryService/TransactionByNote",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn validator_status(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidatorStatusRequest>,
        ) -> Result<tonic::Response<super::ValidatorStatusResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.SpecificQueryService/ValidatorStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn validator_penalty(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidatorPenaltyRequest>,
        ) -> Result<tonic::Response<super::ValidatorPenaltyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.SpecificQueryService/ValidatorPenalty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn next_validator_rate(
            &mut self,
            request: impl tonic::IntoRequest<super::NextValidatorRateRequest>,
        ) -> Result<tonic::Response<super::NextValidatorRateResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.SpecificQueryService/NextValidatorRate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn batch_swap_output_data(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchSwapOutputDataRequest>,
        ) -> Result<tonic::Response<super::BatchSwapOutputDataResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.SpecificQueryService/BatchSwapOutputData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn stub_cpmm_reserves(
            &mut self,
            request: impl tonic::IntoRequest<super::StubCpmmReservesRequest>,
        ) -> Result<tonic::Response<super::StubCpmmReservesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.SpecificQueryService/StubCPMMReserves",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn asset_info(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetInfoRequest>,
        ) -> Result<tonic::Response<super::AssetInfoResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.SpecificQueryService/AssetInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// General-purpose key-value state query API, that can be used to query
        /// arbitrary keys in the JMT storage.
        pub async fn key_value(
            &mut self,
            request: impl tonic::IntoRequest<super::KeyValueRequest>,
        ) -> Result<tonic::Response<super::KeyValueResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.SpecificQueryService/KeyValue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// General-purpose prefixed key-value state query API, that can be used to query
        /// arbitrary prefixes in the JMT storage.
        pub async fn prefix_value(
            &mut self,
            request: impl tonic::IntoRequest<super::PrefixValueRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::PrefixValueResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.SpecificQueryService/PrefixValue",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
#[cfg(feature = "rpc")]
pub mod tendermint_proxy_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Defines the gRPC query service for proxying requests to an upstream Tendermint RPC.
    #[derive(Debug, Clone)]
    pub struct TendermintProxyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TendermintProxyServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TendermintProxyServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TendermintProxyServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            TendermintProxyServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Status queries the current status.
        pub async fn get_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStatusRequest>,
        ) -> Result<tonic::Response<super::GetStatusResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.TendermintProxyService/GetStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Broadcast a transaction asynchronously.
        pub async fn broadcast_tx_async(
            &mut self,
            request: impl tonic::IntoRequest<super::BroadcastTxAsyncRequest>,
        ) -> Result<tonic::Response<super::BroadcastTxAsyncResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.TendermintProxyService/BroadcastTxAsync",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Broadcast a transaction synchronously.
        pub async fn broadcast_tx_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::BroadcastTxSyncRequest>,
        ) -> Result<tonic::Response<super::BroadcastTxSyncResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.TendermintProxyService/BroadcastTxSync",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Fetch a transaction by hash.
        pub async fn get_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTxRequest>,
        ) -> Result<tonic::Response<super::GetTxResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.TendermintProxyService/GetTx",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ABCIQuery defines a query handler that supports ABCI queries directly to the
        /// application, bypassing Tendermint completely. The ABCI query must contain
        /// a valid and supported path, including app, custom, p2p, and store.
        pub async fn abci_query(
            &mut self,
            request: impl tonic::IntoRequest<super::AbciQueryRequest>,
        ) -> Result<tonic::Response<super::AbciQueryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.TendermintProxyService/ABCIQuery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetBlockByHeight queries block for given height.
        pub async fn get_block_by_height(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockByHeightRequest>,
        ) -> Result<tonic::Response<super::GetBlockByHeightResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.client.v1alpha1.TendermintProxyService/GetBlockByHeight",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "rpc")]
pub mod oblivious_query_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ObliviousQueryServiceServer.
    #[async_trait]
    pub trait ObliviousQueryService: Send + Sync + 'static {
        /// Server streaming response type for the CompactBlockRange method.
        type CompactBlockRangeStream: futures_core::Stream<
                Item = Result<super::CompactBlockRangeResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn compact_block_range(
            &self,
            request: tonic::Request<super::CompactBlockRangeRequest>,
        ) -> Result<tonic::Response<Self::CompactBlockRangeStream>, tonic::Status>;
        async fn chain_parameters(
            &self,
            request: tonic::Request<super::ChainParametersRequest>,
        ) -> Result<tonic::Response<super::ChainParametersResponse>, tonic::Status>;
        /// Server streaming response type for the MutableParameters method.
        type MutableParametersStream: futures_core::Stream<
                Item = Result<super::MutableParametersResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn mutable_parameters(
            &self,
            request: tonic::Request<super::MutableParametersRequest>,
        ) -> Result<tonic::Response<Self::MutableParametersStream>, tonic::Status>;
        /// Server streaming response type for the ValidatorInfo method.
        type ValidatorInfoStream: futures_core::Stream<
                Item = Result<super::ValidatorInfoResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn validator_info(
            &self,
            request: tonic::Request<super::ValidatorInfoRequest>,
        ) -> Result<tonic::Response<Self::ValidatorInfoStream>, tonic::Status>;
        async fn asset_list(
            &self,
            request: tonic::Request<super::AssetListRequest>,
        ) -> Result<tonic::Response<super::AssetListResponse>, tonic::Status>;
    }
    /// Methods for accessing chain state that are "oblivious" in the sense that they
    /// do not request specific portions of the chain state that could reveal private
    /// client data.  For instance, requesting all asset denominations is oblivious,
    /// but requesting the asset denomination for a specific asset id is not, because
    /// it reveals that the client has an interest in that asset specifically.
    #[derive(Debug)]
    pub struct ObliviousQueryServiceServer<T: ObliviousQueryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ObliviousQueryService> ObliviousQueryServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for ObliviousQueryServiceServer<T>
    where
        T: ObliviousQueryService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/penumbra.client.v1alpha1.ObliviousQueryService/CompactBlockRange" => {
                    #[allow(non_camel_case_types)]
                    struct CompactBlockRangeSvc<T: ObliviousQueryService>(pub Arc<T>);
                    impl<
                        T: ObliviousQueryService,
                    > tonic::server::ServerStreamingService<
                        super::CompactBlockRangeRequest,
                    > for CompactBlockRangeSvc<T> {
                        type Response = super::CompactBlockRangeResponse;
                        type ResponseStream = T::CompactBlockRangeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CompactBlockRangeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).compact_block_range(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CompactBlockRangeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.ObliviousQueryService/ChainParameters" => {
                    #[allow(non_camel_case_types)]
                    struct ChainParametersSvc<T: ObliviousQueryService>(pub Arc<T>);
                    impl<
                        T: ObliviousQueryService,
                    > tonic::server::UnaryService<super::ChainParametersRequest>
                    for ChainParametersSvc<T> {
                        type Response = super::ChainParametersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChainParametersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).chain_parameters(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChainParametersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.ObliviousQueryService/MutableParameters" => {
                    #[allow(non_camel_case_types)]
                    struct MutableParametersSvc<T: ObliviousQueryService>(pub Arc<T>);
                    impl<
                        T: ObliviousQueryService,
                    > tonic::server::ServerStreamingService<
                        super::MutableParametersRequest,
                    > for MutableParametersSvc<T> {
                        type Response = super::MutableParametersResponse;
                        type ResponseStream = T::MutableParametersStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MutableParametersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).mutable_parameters(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MutableParametersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.ObliviousQueryService/ValidatorInfo" => {
                    #[allow(non_camel_case_types)]
                    struct ValidatorInfoSvc<T: ObliviousQueryService>(pub Arc<T>);
                    impl<
                        T: ObliviousQueryService,
                    > tonic::server::ServerStreamingService<super::ValidatorInfoRequest>
                    for ValidatorInfoSvc<T> {
                        type Response = super::ValidatorInfoResponse;
                        type ResponseStream = T::ValidatorInfoStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ValidatorInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).validator_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ValidatorInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.ObliviousQueryService/AssetList" => {
                    #[allow(non_camel_case_types)]
                    struct AssetListSvc<T: ObliviousQueryService>(pub Arc<T>);
                    impl<
                        T: ObliviousQueryService,
                    > tonic::server::UnaryService<super::AssetListRequest>
                    for AssetListSvc<T> {
                        type Response = super::AssetListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssetListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).asset_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssetListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ObliviousQueryService> Clone for ObliviousQueryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ObliviousQueryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ObliviousQueryService> tonic::server::NamedService
    for ObliviousQueryServiceServer<T> {
        const NAME: &'static str = "penumbra.client.v1alpha1.ObliviousQueryService";
    }
}
/// Generated server implementations.
#[cfg(feature = "rpc")]
pub mod specific_query_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SpecificQueryServiceServer.
    #[async_trait]
    pub trait SpecificQueryService: Send + Sync + 'static {
        async fn transaction_by_note(
            &self,
            request: tonic::Request<super::TransactionByNoteRequest>,
        ) -> Result<tonic::Response<super::TransactionByNoteResponse>, tonic::Status>;
        async fn validator_status(
            &self,
            request: tonic::Request<super::ValidatorStatusRequest>,
        ) -> Result<tonic::Response<super::ValidatorStatusResponse>, tonic::Status>;
        async fn validator_penalty(
            &self,
            request: tonic::Request<super::ValidatorPenaltyRequest>,
        ) -> Result<tonic::Response<super::ValidatorPenaltyResponse>, tonic::Status>;
        async fn next_validator_rate(
            &self,
            request: tonic::Request<super::NextValidatorRateRequest>,
        ) -> Result<tonic::Response<super::NextValidatorRateResponse>, tonic::Status>;
        async fn batch_swap_output_data(
            &self,
            request: tonic::Request<super::BatchSwapOutputDataRequest>,
        ) -> Result<tonic::Response<super::BatchSwapOutputDataResponse>, tonic::Status>;
        async fn stub_cpmm_reserves(
            &self,
            request: tonic::Request<super::StubCpmmReservesRequest>,
        ) -> Result<tonic::Response<super::StubCpmmReservesResponse>, tonic::Status>;
        async fn asset_info(
            &self,
            request: tonic::Request<super::AssetInfoRequest>,
        ) -> Result<tonic::Response<super::AssetInfoResponse>, tonic::Status>;
        /// General-purpose key-value state query API, that can be used to query
        /// arbitrary keys in the JMT storage.
        async fn key_value(
            &self,
            request: tonic::Request<super::KeyValueRequest>,
        ) -> Result<tonic::Response<super::KeyValueResponse>, tonic::Status>;
        /// Server streaming response type for the PrefixValue method.
        type PrefixValueStream: futures_core::Stream<
                Item = Result<super::PrefixValueResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// General-purpose prefixed key-value state query API, that can be used to query
        /// arbitrary prefixes in the JMT storage.
        async fn prefix_value(
            &self,
            request: tonic::Request<super::PrefixValueRequest>,
        ) -> Result<tonic::Response<Self::PrefixValueStream>, tonic::Status>;
    }
    /// Methods for accessing chain state that are "specific" in the sense that they
    /// request specific portions of the chain state that could reveal private
    /// client data.  For instance, requesting all asset denominations is oblivious,
    /// but requesting the asset denomination for a specific asset id is not, because
    /// it reveals that the client has an interest in that asset specifically.
    #[derive(Debug)]
    pub struct SpecificQueryServiceServer<T: SpecificQueryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SpecificQueryService> SpecificQueryServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for SpecificQueryServiceServer<T>
    where
        T: SpecificQueryService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/penumbra.client.v1alpha1.SpecificQueryService/TransactionByNote" => {
                    #[allow(non_camel_case_types)]
                    struct TransactionByNoteSvc<T: SpecificQueryService>(pub Arc<T>);
                    impl<
                        T: SpecificQueryService,
                    > tonic::server::UnaryService<super::TransactionByNoteRequest>
                    for TransactionByNoteSvc<T> {
                        type Response = super::TransactionByNoteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransactionByNoteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).transaction_by_note(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransactionByNoteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.SpecificQueryService/ValidatorStatus" => {
                    #[allow(non_camel_case_types)]
                    struct ValidatorStatusSvc<T: SpecificQueryService>(pub Arc<T>);
                    impl<
                        T: SpecificQueryService,
                    > tonic::server::UnaryService<super::ValidatorStatusRequest>
                    for ValidatorStatusSvc<T> {
                        type Response = super::ValidatorStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ValidatorStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).validator_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ValidatorStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.SpecificQueryService/ValidatorPenalty" => {
                    #[allow(non_camel_case_types)]
                    struct ValidatorPenaltySvc<T: SpecificQueryService>(pub Arc<T>);
                    impl<
                        T: SpecificQueryService,
                    > tonic::server::UnaryService<super::ValidatorPenaltyRequest>
                    for ValidatorPenaltySvc<T> {
                        type Response = super::ValidatorPenaltyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ValidatorPenaltyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).validator_penalty(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ValidatorPenaltySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.SpecificQueryService/NextValidatorRate" => {
                    #[allow(non_camel_case_types)]
                    struct NextValidatorRateSvc<T: SpecificQueryService>(pub Arc<T>);
                    impl<
                        T: SpecificQueryService,
                    > tonic::server::UnaryService<super::NextValidatorRateRequest>
                    for NextValidatorRateSvc<T> {
                        type Response = super::NextValidatorRateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NextValidatorRateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).next_validator_rate(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NextValidatorRateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.SpecificQueryService/BatchSwapOutputData" => {
                    #[allow(non_camel_case_types)]
                    struct BatchSwapOutputDataSvc<T: SpecificQueryService>(pub Arc<T>);
                    impl<
                        T: SpecificQueryService,
                    > tonic::server::UnaryService<super::BatchSwapOutputDataRequest>
                    for BatchSwapOutputDataSvc<T> {
                        type Response = super::BatchSwapOutputDataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchSwapOutputDataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).batch_swap_output_data(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BatchSwapOutputDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.SpecificQueryService/StubCPMMReserves" => {
                    #[allow(non_camel_case_types)]
                    struct StubCPMMReservesSvc<T: SpecificQueryService>(pub Arc<T>);
                    impl<
                        T: SpecificQueryService,
                    > tonic::server::UnaryService<super::StubCpmmReservesRequest>
                    for StubCPMMReservesSvc<T> {
                        type Response = super::StubCpmmReservesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StubCpmmReservesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).stub_cpmm_reserves(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StubCPMMReservesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.SpecificQueryService/AssetInfo" => {
                    #[allow(non_camel_case_types)]
                    struct AssetInfoSvc<T: SpecificQueryService>(pub Arc<T>);
                    impl<
                        T: SpecificQueryService,
                    > tonic::server::UnaryService<super::AssetInfoRequest>
                    for AssetInfoSvc<T> {
                        type Response = super::AssetInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssetInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).asset_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssetInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.SpecificQueryService/KeyValue" => {
                    #[allow(non_camel_case_types)]
                    struct KeyValueSvc<T: SpecificQueryService>(pub Arc<T>);
                    impl<
                        T: SpecificQueryService,
                    > tonic::server::UnaryService<super::KeyValueRequest>
                    for KeyValueSvc<T> {
                        type Response = super::KeyValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KeyValueRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).key_value(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = KeyValueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.SpecificQueryService/PrefixValue" => {
                    #[allow(non_camel_case_types)]
                    struct PrefixValueSvc<T: SpecificQueryService>(pub Arc<T>);
                    impl<
                        T: SpecificQueryService,
                    > tonic::server::ServerStreamingService<super::PrefixValueRequest>
                    for PrefixValueSvc<T> {
                        type Response = super::PrefixValueResponse;
                        type ResponseStream = T::PrefixValueStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PrefixValueRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).prefix_value(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PrefixValueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: SpecificQueryService> Clone for SpecificQueryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SpecificQueryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SpecificQueryService> tonic::server::NamedService
    for SpecificQueryServiceServer<T> {
        const NAME: &'static str = "penumbra.client.v1alpha1.SpecificQueryService";
    }
}
/// Generated server implementations.
#[cfg(feature = "rpc")]
pub mod tendermint_proxy_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with TendermintProxyServiceServer.
    #[async_trait]
    pub trait TendermintProxyService: Send + Sync + 'static {
        /// Status queries the current status.
        async fn get_status(
            &self,
            request: tonic::Request<super::GetStatusRequest>,
        ) -> Result<tonic::Response<super::GetStatusResponse>, tonic::Status>;
        /// Broadcast a transaction asynchronously.
        async fn broadcast_tx_async(
            &self,
            request: tonic::Request<super::BroadcastTxAsyncRequest>,
        ) -> Result<tonic::Response<super::BroadcastTxAsyncResponse>, tonic::Status>;
        /// Broadcast a transaction synchronously.
        async fn broadcast_tx_sync(
            &self,
            request: tonic::Request<super::BroadcastTxSyncRequest>,
        ) -> Result<tonic::Response<super::BroadcastTxSyncResponse>, tonic::Status>;
        /// Fetch a transaction by hash.
        async fn get_tx(
            &self,
            request: tonic::Request<super::GetTxRequest>,
        ) -> Result<tonic::Response<super::GetTxResponse>, tonic::Status>;
        /// ABCIQuery defines a query handler that supports ABCI queries directly to the
        /// application, bypassing Tendermint completely. The ABCI query must contain
        /// a valid and supported path, including app, custom, p2p, and store.
        async fn abci_query(
            &self,
            request: tonic::Request<super::AbciQueryRequest>,
        ) -> Result<tonic::Response<super::AbciQueryResponse>, tonic::Status>;
        /// GetBlockByHeight queries block for given height.
        async fn get_block_by_height(
            &self,
            request: tonic::Request<super::GetBlockByHeightRequest>,
        ) -> Result<tonic::Response<super::GetBlockByHeightResponse>, tonic::Status>;
    }
    /// Defines the gRPC query service for proxying requests to an upstream Tendermint RPC.
    #[derive(Debug)]
    pub struct TendermintProxyServiceServer<T: TendermintProxyService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TendermintProxyService> TendermintProxyServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for TendermintProxyServiceServer<T>
    where
        T: TendermintProxyService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/penumbra.client.v1alpha1.TendermintProxyService/GetStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetStatusSvc<T: TendermintProxyService>(pub Arc<T>);
                    impl<
                        T: TendermintProxyService,
                    > tonic::server::UnaryService<super::GetStatusRequest>
                    for GetStatusSvc<T> {
                        type Response = super::GetStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.TendermintProxyService/BroadcastTxAsync" => {
                    #[allow(non_camel_case_types)]
                    struct BroadcastTxAsyncSvc<T: TendermintProxyService>(pub Arc<T>);
                    impl<
                        T: TendermintProxyService,
                    > tonic::server::UnaryService<super::BroadcastTxAsyncRequest>
                    for BroadcastTxAsyncSvc<T> {
                        type Response = super::BroadcastTxAsyncResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BroadcastTxAsyncRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).broadcast_tx_async(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BroadcastTxAsyncSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.TendermintProxyService/BroadcastTxSync" => {
                    #[allow(non_camel_case_types)]
                    struct BroadcastTxSyncSvc<T: TendermintProxyService>(pub Arc<T>);
                    impl<
                        T: TendermintProxyService,
                    > tonic::server::UnaryService<super::BroadcastTxSyncRequest>
                    for BroadcastTxSyncSvc<T> {
                        type Response = super::BroadcastTxSyncResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BroadcastTxSyncRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).broadcast_tx_sync(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BroadcastTxSyncSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.TendermintProxyService/GetTx" => {
                    #[allow(non_camel_case_types)]
                    struct GetTxSvc<T: TendermintProxyService>(pub Arc<T>);
                    impl<
                        T: TendermintProxyService,
                    > tonic::server::UnaryService<super::GetTxRequest> for GetTxSvc<T> {
                        type Response = super::GetTxResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTxRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_tx(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.TendermintProxyService/ABCIQuery" => {
                    #[allow(non_camel_case_types)]
                    struct ABCIQuerySvc<T: TendermintProxyService>(pub Arc<T>);
                    impl<
                        T: TendermintProxyService,
                    > tonic::server::UnaryService<super::AbciQueryRequest>
                    for ABCIQuerySvc<T> {
                        type Response = super::AbciQueryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AbciQueryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).abci_query(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ABCIQuerySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/penumbra.client.v1alpha1.TendermintProxyService/GetBlockByHeight" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByHeightSvc<T: TendermintProxyService>(pub Arc<T>);
                    impl<
                        T: TendermintProxyService,
                    > tonic::server::UnaryService<super::GetBlockByHeightRequest>
                    for GetBlockByHeightSvc<T> {
                        type Response = super::GetBlockByHeightResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockByHeightRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_block_by_height(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByHeightSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: TendermintProxyService> Clone for TendermintProxyServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TendermintProxyService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TendermintProxyService> tonic::server::NamedService
    for TendermintProxyServiceServer<T> {
        const NAME: &'static str = "penumbra.client.v1alpha1.TendermintProxyService";
    }
}
