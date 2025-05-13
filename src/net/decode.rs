use std::error::Error;
use std::io::Read;
use flate2::read::GzDecoder;
use reqwest::blocking::Response;

pub fn decode_response(response : Response) -> Result<String, Box<(dyn Error)>> {
    let encoding = response
        .headers()
        .get("content-encoding")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let mut reader: Box<dyn Read> = match encoding {
        "gzip" => Box::new(GzDecoder::new(response)),
        "br" => Box::new(brotli::Decompressor::new(response, 4096)),
        "deflate" => Box::new(flate2::read::ZlibDecoder::new(response)),
        _ => Box::new(response),
    };

    let mut body = Vec::new();
    reader.read_to_end(&mut body)?;

    let text = String::from_utf8(body)?;
    Ok(text)
}