#![allow(clippy::uninlined_format_args)]

mod error;
mod standalone;
mod utilities;
mod whisper_ctx;
mod whisper_grammar;
mod whisper_params;
mod whisper_state;

pub use error::WhisperError;
pub use standalone::*;
pub use utilities::*;
pub use whisper_ctx::WhisperContext;
pub use whisper_ctx::WhisperContextParameters;
pub use whisper_grammar::{WhisperGrammarElement, WhisperGrammarElementType};
pub use whisper_params::{FullParams, SamplingStrategy};
pub use whisper_state::WhisperState;

pub type WhisperSysContext = whisper_rs_sys::whisper_context;
pub type WhisperSysState = whisper_rs_sys::whisper_state;

pub type WhisperTokenData = whisper_rs_sys::whisper_token_data;
pub type WhisperToken = whisper_rs_sys::whisper_token;
pub type WhisperNewSegmentCallback = whisper_rs_sys::whisper_new_segment_callback;
pub type WhisperStartEncoderCallback = whisper_rs_sys::whisper_encoder_begin_callback;
pub type WhisperProgressCallback = whisper_rs_sys::whisper_progress_callback;
pub type WhisperLogitsFilterCallback = whisper_rs_sys::whisper_logits_filter_callback;
pub type WhisperAbortCallback = whisper_rs_sys::ggml_abort_callback;
pub type WhisperLogCallback = whisper_rs_sys::ggml_log_callback;
