extern crate rust_multicodec;
extern crate serde;
extern crate serde_json;

use rust_multicodec::encoding;
use rust_multicodec::encoding::Codec;
use std::str;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Person {
    name: String
}

/// Working demo
fn main(){
    let to_be_encoded=Person{name:String::from("sanyi")};
    let encode_result= encoding::encode(Codec::JSON, &to_be_encoded);
    println!("encoded: {:?}",encode_result);

    let decoded:Person=encoding::decode(encode_result.unwrap().as_ref()).unwrap().data;
    println!("decoded: {:?}",decoded);
}