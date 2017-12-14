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
    use serde::de::DeserializeOwned;

    pub mod codec;
    pub mod codec_map;

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum Codec {
        JSON
    }

    pub struct DecodeResult<T> {
        pub data: T,
        pub codec: Codec
    }

    pub fn encode<T:Serialize>(codec: Codec, object: &T) -> Result<Vec<u8>,&'static str>{
        match codec {
            Codec::JSON => {
                match serde_json::to_string(&object){
                    Ok(json) => {
                        match codec::add_prefix("json",json.as_bytes()){
                            Ok(prefixed)=>Ok(prefixed),
                            Err(err)=>Err(err)
                        }
                    },
                    Err(_) => Err("Could not serialise the given object to json")
                }
            }
        }
    }

    pub fn decode<T:DeserializeOwned>(encoded_data:&[u8]) -> Result<DecodeResult<T>,&'static str>{
        match codec::get_codec(encoded_data){
            Some("json")=>{
                let wout_prefix=codec::remove_prefix(encoded_data).expect("Could not remove codec prefix from the given data");
                let data=serde_json::from_slice(wout_prefix.as_ref()).expect("Could not deserialize the json object");

                Ok(DecodeResult{
                    codec:Codec::JSON,
                    data
                })
            },
            Some(_) => Err("The data was encoded with a codec which is not handled right now"),
            None=>Err("Could not extract the codec from the given data")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use encoding::{Codec,DecodeResult};

    #[derive(Serialize, Deserialize)]
    struct TestObject {
        message: String
    }

    #[test]
    fn encoding_decoding_works(){
        let test_object=TestObject{message:String::from("Live long and prosper")};
        let encoded=encoding::encode(Codec::JSON, &test_object);
        assert_eq!(encoded.is_ok(),true);

        let decoded:DecodeResult<TestObject>=encoding::decode(encoded.unwrap().as_ref()).unwrap();
        assert_eq!(decoded.data.message, test_object.message);
        assert_eq!(decoded.codec,Codec::JSON);
    }
}
