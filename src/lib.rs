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
    pub codec: codec::CodecType
}

pub fn encode<T:Serialize>(codec: codec::CodecType, object: &T) -> Result<Vec<u8>,&'static str>{
    match codec {
        codec::CodecType::JSON => {
            // TODO
            match serde_json::to_string(&object){
                Ok(json) => codec_prefix::add(codec, json.as_bytes()),
                Err(_) => Err("Could not serialise the given object to json")
            }
        }
    }
}

pub fn decode<T:DeserializeOwned>(encoded_data:&[u8]) -> Result<DecodeResult<T>,&'static str>{
    match codec_prefix::get(encoded_data){
        Some(codec::CodecType::JSON)=>{
            let wout_prefix= codec_prefix::remove(encoded_data).expect("Could not remove codec prefix from the given data");
            let data=serde_json::from_slice(wout_prefix.as_ref()).expect("Could not deserialize the json object");

            Ok(DecodeResult{
                codec:codec::CodecType::JSON,
                data
            })
        },
        None=>Err("Could not extract the codec from the given data")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct TestObject {
        message: String
    }

    #[test]
    fn encoding_decoding_works(){
        let test_object=TestObject{message:String::from("Live long and prosper")};
        let encoded=encode(codec::CodecType::JSON, &test_object);
        assert_eq!(encoded.is_ok(),true);

        let decoded:DecodeResult<TestObject>=decode(encoded.unwrap().as_ref()).unwrap();
        assert_eq!(decoded.data.message, test_object.message);
        assert_eq!(decoded.codec,codec::CodecType::JSON);
    }
}
