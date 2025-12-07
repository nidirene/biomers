// examples/demo.rs

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 500;
    let height = 500;
    // Create dummy gray image
    let raw_pixels = vec![128u8; (width * height) as usize];

    println!("Encoding WSQ...");
    let wsq_bytes = biomers::wsq_encode(&raw_pixels, width, height, 0.75)?;
    println!("Encoded {} bytes to WSQ", wsq_bytes.len());

    println!("Decoding WSQ...");
    let (decoded_pixels, w, h) = biomers::wsq_decode(&wsq_bytes)?;
    println!("Decoded {}x{} image", w, h);

    assert_eq!(decoded_pixels.len(), raw_pixels.len());
    
    Ok(())
}