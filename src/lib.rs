//! This library is a Rust implementation of the https://github.com/multiformats/multicodec project.
//!
//! This crate also can be used to encode and decode an object.
//! Currently only JSON is supported

extern crate integer_encoding;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::str;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub mod codec_prefix;
pub mod codec;

pub struct DecodeResult<T> {
    pub data: T,
    pub codec: codec::CodecType,
}

/// Returns the encoded object with a prefix of the given codec.
/// Note that the object must implement Serialize trait from serde's lib
///
/// # Arguments
///
/// * `codec` - The codec type, eg. CodecType::JSON
/// * `object` - the object reference to be encoded
///
/// # Example
///
/// ```
/// extern crate rust_multicodec;
/// #[macro_use]
/// extern crate serde_derive;
/// use rust_multicodec::{codec, encode};
/// #[derive(Serialize)]
/// #[derive(Debug)]
/// struct Person {
///     name: String
/// }
///
/// fn main(){
///     let to_be_encoded = Person{ name: String::from("sanyi") };
///     println!("{:?}", encode(codec::CodecType::JSON, &to_be_encoded));
///     // it will print: Ok([129, 30, 123, 34, 110, 97, 109, 101, 34, 58, 34, 115, 97, 110, ...])
/// }
/// ```
///
pub fn encode<T: Serialize>(codec: codec::CodecType, object: &T) -> Result<Vec<u8>, String> {
    match codec {
        codec::CodecType::JSON => {
            match serde_json::to_string(&object) {
                Ok(json) => {
                    codec_prefix::add(codec, json.as_bytes()).map_err(|io_error| -> String {
                        format!(
                            "Could not add prefix to the given object. Error: {:?}",
                            io_error.kind()
                        )
                    })
                }
                Err(err) => Err(format!(
                    "Could not serialize the given object. Serde error: {:?}",
                    err
                )),
            }
        }
    }
}

/// Decodes a byte array back into an Object.
/// Note that the object must implement Deserialize trait from serde's lib
///
/// # Arguments
///
/// * `encoded_data` - The data that was encoded with the encode method
///
/// # Example
///
/// ```
/// extern crate rust_multicodec;
/// #[macro_use]
/// extern crate serde_derive;
/// use rust_multicodec::{codec, encode, decode};
/// #[derive(Serialize, Deserialize)]
/// #[derive(Debug)]
/// struct Person {
///     name: String
/// }
///
/// fn main(){
///     let to_be_encoded = Person { name: String::from("sanyi") };
///     let encoded = encode(codec::CodecType::JSON, &to_be_encoded).unwrap();
///     let decoded: Person = decode(encoded.as_ref()).unwrap().data;
///     println!("{:?}", decoded);
///     // it will print: Person { name: "sanyi" }
/// }
/// ```
///
pub fn decode<T: DeserializeOwned>(encoded_data: &[u8]) -> Result<DecodeResult<T>, String> {
    match codec_prefix::get(encoded_data) {
        Some(codec::CodecType::JSON) => {
            let wout_prefix = codec_prefix::remove(encoded_data);

            match serde_json::from_slice(wout_prefix) {
                Ok(data) => Ok(DecodeResult {
                    codec: codec::CodecType::JSON,
                    data,
                }),
                Err(err) => Err(format!(
                    "Could not deserialize the given data. Serde error: {:?}",
                    err
                )),
            }
        }
        None => Err(String::from(
            "Could not deserialize the given data, the codec at the beginning is unknown",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct TestObject {
        message: String,
    }

    #[test]
    fn encoding_decoding_works() {
        let test_object = TestObject { message: String::from("Live long and prosper") };
        let encoded = encode(codec::CodecType::JSON, &test_object);
        assert_eq!(encoded.is_ok(), true);

        let decoded: DecodeResult<TestObject> = decode(encoded.unwrap().as_ref()).unwrap();
        assert_eq!(decoded.data.message, test_object.message);
        assert_eq!(decoded.codec, codec::CodecType::JSON);
    }
}
