use crate::sys;
use std::ffi::c_int;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WsqError {
    #[error("Failed to encode WSQ image (Error code: {0})")]
    EncodingFailed(i32),
    #[error("Failed to decode WSQ image (Error code: {0})")]
    DecodingFailed(i32),
    #[error("Invalid parameters")]
    InvalidInput,
}

/// Encodes raw grayscale image data to WSQ format.
/// 
/// * `raw_data`: 8-bit grayscale pixel data
/// * `width`: Image width
/// * `height`: Image height
/// * `bitrate`: Compression ratio (e.g., 0.75 or 2.25)
pub fn wsq_encode(
    raw_data: &[u8],
    width: i32,
    height: i32,
    bitrate: f32,
) -> Result<Vec<u8>, WsqError> {
    unsafe {
        let mut output_ptr: *mut u8 = std::ptr::null_mut();
        let mut output_len: c_int = 0;
        
        // NBIS usually wants mutable pointers even if it reads, 
        // but we cast const to mut for the API.
        let raw_ptr = raw_data.as_ptr() as *mut u8;

        // Note: Function signature varies slightly by NBIS version. 
        // Typically: wsq_encode_mem(data, w, h, depth, ppi, bitrate, comment, &out, &len)
        // Adjust arguments based on the specific version in libbiomeval.
        let ret = sys::biomeval_nbis_wsq_encode_mem(
            &mut output_ptr,
            &mut output_len,
            bitrate,
            raw_ptr,
            width,
            height,
            8, // depth
            500, // ppi (standard for biometrics)
            std::ptr::null_mut(), // comment
        );

        if ret != 0 {
            return Err(WsqError::EncodingFailed(ret));
        }

        if output_ptr.is_null() || output_len <= 0 {
            return Err(WsqError::EncodingFailed(-1));
        }

        // Copy data to Rust Vec
        let output_slice = std::slice::from_raw_parts(output_ptr, output_len as usize);
        let result = output_slice.to_vec();

        // Free C memory allocated by NBIS
        libc::free(output_ptr as *mut libc::c_void);

        Ok(result)
    }
}

/// Decodes a WSQ buffer into raw grayscale pixels.
pub fn wsq_decode(wsq_data: &[u8]) -> Result<(Vec<u8>, i32, i32), WsqError> {
    unsafe {
        let mut output_ptr: *mut u8 = std::ptr::null_mut();
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let mut depth: c_int = 0;
        let mut ppi: c_int = 0;
        let mut lossy: c_int = 0;

        let ret = sys::biomeval_nbis_wsq_decode_mem(
            &mut output_ptr,
            &mut width,
            &mut height,
            &mut depth,
            &mut ppi,
            &mut lossy,
            wsq_data.as_ptr() as *mut u8,
            wsq_data.len() as c_int,
        );

        if ret != 0 {
            return Err(WsqError::DecodingFailed(ret));
        }

        let len = (width * height) as usize;
        let output_slice = std::slice::from_raw_parts(output_ptr, len);
        let result = output_slice.to_vec();

        libc::free(output_ptr as *mut libc::c_void);

        Ok((result, width, height))
    }
}