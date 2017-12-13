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

fn main(){
    let person=Person{name:String::from("sanyi")};
    let result= encoding::encode(Codec::JSON, &person);
    println!("encoded: {:?}",result);

    let x=encoding::decode(result.unwrap().as_ref());
    let person2:Person=serde_json::from_slice(x.as_ref())?;

    println!("decoded: {:?}",person2);

}