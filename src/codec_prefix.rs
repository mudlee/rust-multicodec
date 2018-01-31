use std::io::{Write, Error};
use integer_encoding::VarInt;
use codec::CodecType;

/// Returns the data prefixed with the codec's code in a u8 buffer.
///
/// # Arguments
///
/// * `codec_code` - The codec type, eg. CodecType::JSON
/// * `data` - the data to be prefixed
///
/// # Example
///
/// ```
/// extern crate rust_multicodec;
/// extern crate hex_slice;
///
/// use rust_multicodec::codec_prefix;
/// use rust_multicodec::codec::CodecType;
/// use hex_slice::AsHex;
/// use std::process;
///
/// fn main(){
///     let data="Live long and prosper";
///
///     println!("{:X}", codec_prefix::add(CodecType::JSON,data.as_bytes()).unwrap().as_hex());
///     // it will print [1 4C 69 76 65 20 6C 6F 6E 67 20 61 6E 64 20 70 72 6F 73 70 65 72]
/// }
/// ```
///
pub fn add(codec: CodecType, data: &[u8]) -> Result<Vec<u8>, Error> {
    // encoding codec's (as decimal) into a varint
    let mut target: Vec<u8> = codec.hex().encode_var_vec();
    target.write(data)?;
    Ok(target)
}

/// Returns the codec's code the data was prefixed with.
///
/// # Arguments
///
/// * `data` - the data with prefix
///
/// # Example
///
/// ```
/// extern crate rust_multicodec;
/// extern crate hex_slice;
///
/// use rust_multicodec::codec_prefix;
/// use rust_multicodec::codec::CodecType;
/// use hex_slice::AsHex;
/// use std::process;
///
/// fn main(){
///     let data = "Live long and prosper";
///
///     let prefixed = codec_prefix::add(CodecType::JSON,data.as_bytes()).unwrap();
///     println!("{:?}", codec_prefix::get(prefixed.as_slice()).unwrap());
///     // it will print "JSON"
/// }
/// ```
///
pub fn get(data: &[u8]) -> Option<CodecType> {
    let decoded: (u64, usize) = u64::decode_var_vec(&Vec::from(data));
    CodecType::by_hex(decoded.0)
}

/// Removes the codec prefix and returns the raw data.
///
/// # Arguments
///
/// * `data` - the data with prefix
///
/// # Example
///
/// ```
/// extern crate rust_multicodec;
/// extern crate hex_slice;
///
/// use rust_multicodec::codec_prefix;
/// use rust_multicodec::codec::CodecType;
/// use hex_slice::AsHex;
/// use std::process;
///
/// fn main(){
///     let data="Live long and prosper";
///
///     let prefixed = codec_prefix::add(CodecType::JSON,data.as_bytes()).unwrap();
///     let raw_data = codec_prefix::remove(prefixed.as_slice());
///     println!("Original data was {:?}", String::from_utf8(raw_data.to_vec()).unwrap())
///     // it will print return "Original data was Live long and prosper"
/// }
/// ```
///
pub fn remove<'a>(data: &'a [u8]) -> &'a [u8] {
    let decoded: (u64, usize) = u64::decode_var_vec(&Vec::from(data));
    &data[decoded.1..]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "Live long and prosper";

    #[test]
    fn prefix_works() {
        let result = add(CodecType::JSON, DATA.as_bytes());
        assert_eq!(result.is_ok(), true);

        let prefixed = result.unwrap();
        assert_eq!(get(prefixed.as_slice()).unwrap(), CodecType::JSON);
        assert_eq!(remove(prefixed.as_slice()), DATA.as_bytes());
    }
}
