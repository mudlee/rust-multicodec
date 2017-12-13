//! This library is a Rust implementation of the https://github.com/multiformats/multicodec project.
extern crate integer_encoding;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

/// This module can be used to encode and decode an object.
/// Currently only JSON is supported.
pub mod encoding {
    use std::str;
    use serde_json;
    use serde::Serialize;
    use serde::Deserialize;

    pub mod codec;
    pub mod codec_map;

    pub enum Codec {
        JSON
    }

    struct DecodeResult<T> {
        data: T,
        codec: Codec
    }

    pub fn encode<T:Serialize>(codec: Codec, object: &T) -> Result<Vec<u8>,&'static str>{
        match codec {
            Codec::JSON => {
                match serde_json::to_string(&object){
                    Ok(json) => {
                        match codec::add_prefix("json",json.as_bytes()){
                            Ok(prefixed)=>return Ok(prefixed),
                            Err(err)=>return Err(err)
                        }
                    },
                    _ => Err("Could not serialised the given object to json")
                }
            }
        }
    }

    // TODO: not yet finished, must return DecodeResult
    pub fn decode(encoded_data:&[u8]) -> Vec<u8>{
        match codec::get_codec(encoded_data) {
            Ok(codec_used) => {
                match codec::remove_prefix(encoded_data){
                    Ok(wout_prefix) => {
                        match codec_used.unwrap() {
                            "json" => {
                                wout_prefix
                            },
                            _ => vec![]
                        }
                    },
                    _ => vec![]
                }
            },
            _ => vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use encoding::Codec;

    #[derive(Serialize, Deserialize)]
    struct TestObject {
        message: String
    }

    #[test]
    fn encoding_decoding_works(){
        let test_object=TestObject{message:String::from("Live long and prosper")};
        let encoded=encoding::encode(Codec::JSON, &test_object);
        assert_eq!(encoded.is_ok(),true);

        // TODO: finish this
        encoding::decode(encoded.unwrap().as_ref());
    }
}
