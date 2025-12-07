mod sys;
pub mod wsq;
pub mod jpeg;

// Re-export main types
pub use wsq::{wsq_encode, wsq_decode, WsqError};
pub use jpeg::{jpegl_encode, JpegError};