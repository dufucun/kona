#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/op-rs/kona/main/assets/square.png",
    html_favicon_url = "https://raw.githubusercontent.com/op-rs/kona/main/assets/favicon.ico",
    issue_tracker_base_url = "https://github.com/op-rs/kona/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod batch;
pub use batch::{
    Batch, BatchDecodingError, BatchEncodingError, BatchReader, BatchTransaction, BatchType,
    BatchValidationProvider, BatchValidity, BatchWithInclusionBlock, RawSpanBatch, SingleBatch,
    SpanBatch, SpanBatchBits, SpanBatchEip1559TransactionData, SpanBatchEip2930TransactionData,
    SpanBatchEip7702TransactionData, SpanBatchElement, SpanBatchError,
    SpanBatchLegacyTransactionData, SpanBatchPayload, SpanBatchPrefix, SpanBatchTransactionData,
    SpanBatchTransactions, SpanDecodingError, MAX_SPAN_BATCH_ELEMENTS, SINGLE_BATCH_TYPE,
    SPAN_BATCH_TYPE,
};

mod errors;
pub use errors::OpBlockConversionError;

mod block;
pub use block::{BlockInfo, FromBlockError, L2BlockInfo};

mod frame;
pub use frame::{
    Frame, FrameDecodingError, FrameParseError, DERIVATION_VERSION_0, FRAME_OVERHEAD, MAX_FRAME_LEN,
};

mod compression;
pub use compression::{
    compress_zlib, decompress_brotli, decompress_zlib, BrotliDecompressionError, BrotliLevel,
    ChannelCompressor, CompressionAlgo, CompressorError, CompressorResult, CompressorType,
    CompressorWriter, Config, ZlibCompressor,
};
#[cfg(feature = "std")]
pub use compression::{
    BrotliCompressionError, BrotliCompressor, RatioCompressor, ShadowCompressor, VariantCompressor,
};

mod iter;
pub use iter::FrameIter;

mod utils;
pub use utils::{read_tx_data, starts_with_2718_deposit, starts_with_7702_tx, to_system_config};

mod channel_out;
pub use channel_out::{ChannelOut, ChannelOutError};

mod channel;
pub use channel::{
    Channel, ChannelError, ChannelId, CHANNEL_ID_LENGTH, FJORD_MAX_RLP_BYTES_PER_CHANNEL,
    MAX_RLP_BYTES_PER_CHANNEL,
};

mod deposits;
pub use deposits::{
    decode_deposit, DepositError, DEPOSIT_EVENT_ABI, DEPOSIT_EVENT_ABI_HASH,
    DEPOSIT_EVENT_VERSION_0,
};

mod info;
pub use info::{
    closing_deposit_context_tx, BlockInfoError, DecodeError, L1BlockInfoBedrock,
    L1BlockInfoEcotone, L1BlockInfoInterop, L1BlockInfoIsthmus, L1BlockInfoTx,
};

mod fee;
pub use fee::{
    calculate_tx_l1_cost_bedrock, calculate_tx_l1_cost_bedrock_empty_scalars,
    calculate_tx_l1_cost_ecotone, calculate_tx_l1_cost_fjord, calculate_tx_l1_cost_regolith,
    data_gas_bedrock, data_gas_fjord, data_gas_regolith, flz_compress_len, tx_estimated_size_fjord,
};

mod predeploys;
pub use predeploys::Predeploys;

#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;
