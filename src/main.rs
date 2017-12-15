extern crate rust_multicodec;
extern crate serde;
extern crate serde_json;

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
    match rust_multicodec::encode(rust_multicodec::codec::CodecType::JSON, &to_be_encoded) {
        Ok(encoded) => {
            println!("encoded: {:?}",encoded);

            match rust_multicodec::decode(encoded.as_ref()) {
                Ok(decode_result) => {
                    let decoded:Person=decode_result.data;
                    println!("decoded: {:?}",decoded);
                },
                Err(err) => panic!("An error occured: {:?}",err)
            }
        }
        Err(err) => panic!("An error occured: {:?}",err)
    }
}