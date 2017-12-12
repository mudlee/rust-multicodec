//https://developers.google.com/protocol-buffers/docs/encoding
//https://github.com/multiformats/multicodec
//https://github.com/multiformats/unsigned-varint


extern crate rust_multicodec;
extern crate serde;

use rust_multicodec::encoding;
use rust_multicodec::encoding::Codec;

#[macro_use]
extern crate serde_derive;

mod multicodec {
    /*pub struct DecodeResult {
        pub codec: Encoding,
        pub data: Vec<u8>
    }

    /*impl DecodeResult{
        pub fn rawData(&self) -> &Object {
            // TODO returns the original object that was encoded
        }
    }*/

    pub enum Encoding {
        JSON
    }

    /// Encodes a the given object using the encode parameter
    ///
    /// # Arguments
    ///
    /// * `encode` - the codec what format we will use to store the data
    /// * `object` - the object to be encoded and stored
    ///
    /// In a case of JSON, it will store the data like:
    /// {"codec":"json","data": BINARY_VERSION_OF_THE_GIVEN_DATA }
    ///
    /// And it will return
    /// # Returns
    /// It returns a u8 vector, where the first element is the codec itself, the rest is the byte verison of {"codec":"json","data": BINARY_VERSION_OF_THE_GIVEN_DATA }
    pub fn encode<T>(encode: Encoding, object: &T) -> Result<Vec<u8>,&'static str>{
        codec_prefix;
        // TODO
        Ok(Vec::new())
    }

    /*pub fn decode(data: Vec<u8>) -> Result<DecodeResult,&'static str> {

    }*/*/
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String
}

fn main(){
    let person=Person{name:String::from("sanyi")};
    let result= encoding::encode(Codec::JSON, &person);
    println!("encoded: {:?}",result);

}