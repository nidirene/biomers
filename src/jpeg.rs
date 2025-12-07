// JPEGL support is currently disabled.
// The biomeval_nbis_jpegl_encode_mem function requires an IMG_DAT struct
// which needs more complex setup. WSQ is the primary fingerprint format anyway.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum JpegError {
    #[error("JPEG Operation failed: {0}")]
    OperationFailed(i32),
    #[error("JPEGL encoding not yet implemented")]
    NotImplemented,
}

/// Encodes raw data using NBIS JPEGL (Lossless)
///
/// Note: This function is not yet implemented as the underlying C API
/// requires complex IMG_DAT struct setup.
pub fn jpegl_encode(_raw_data: &[u8], _width: i32, _height: i32) -> Result<Vec<u8>, JpegError> {
    Err(JpegError::NotImplemented)
}
