//! Central repository for all the sv2 constants
#![no_std]

pub const SV2_FRAME_HEADER_SIZE: usize = 6;
pub const SV2_FRAME_HEADER_LEN_OFFSET: usize = 3;
pub const SV2_FRAME_HEADER_LEN_END: usize = 3;

pub const NOISE_FRAME_HEADER_SIZE: usize = 2;
pub const NOISE_FRAME_HEADER_LEN_OFFSET: usize = 0;
pub const NOISE_FRAME_HEADER_LEN_END: usize = 2;
pub const NOISE_FRAME_MAX_SIZE: usize = u16::MAX as usize;

pub const NOISE_PARAMS: &str = "Noise_NX_25519_ChaChaPoly_BLAKE2s";
pub const SNOW_PSKLEN: usize = 32;
pub const SNOW_TAGLEN: usize = 16;

pub const SV2_MINING_PROTOCOL_DISCRIMINANT: u8 = 0;
pub const SV2_JOB_NEG_PROTOCOL_DISCRIMINANT: u8 = 1;
pub const SV2_TEMPLATE_DISTR_PROTOCOL_DISCRIMINANT: u8 = 2;
pub const SV2_JOB_DISTR_PROTOCOL_DISCRIMINANT: u8 = 3;

// COMMON MESSAGES TYPES
pub const MESSAGE_TYPE_SETUP_CONNECTION: u8 = 0;
pub const MESSAGE_TYPE_SETUP_CONNECTION_SUCCESS: u8 = 1;
pub const MESSAGE_TYPE_SETUP_CONNECTION_ERROR: u8 = 2;
pub const MESSAGE_TYPE_CHANNEL_ENDPOINT_CHANGES: u8 = 3;
// TEMPLATE DISTRIBUTION PROTOCOL MESSAGES TYPES
pub const MESSAGE_TYPE_COINBASE_OUTPUT_DATA_SIZE: u8 = 70;
pub const MESSAGE_TYPE_NEW_TEMPLATE: u8 = 71;
pub const MESSAGE_TYPE_SET_NEW_PREV_HASH: u8 = 72;
pub const MESSAGE_TYPE_REQUEST_TRANSACTION_DATA: u8 = 73;
pub const MESSAGE_TYPE_REQUEST_TRANSACTION_DATA_SUCCESS: u8 = 74;
pub const MESSAGE_TYPE_REQUEST_TRANSACTION_DATA_ERROR: u8 = 75;
pub const MESSAGE_TYPE_SUBMIT_SOLUTION: u8 = 76;
