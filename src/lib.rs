//! This library is a Rust implementation of the https://github.com/multiformats/multicodec project.
extern crate integer_encoding;

mod codec_map;

mod codec_prefix {
    use codec_map;
    use std::io::Write;
    use integer_encoding::VarInt;

    /// Returns the data prefixed with the codec's code in a u8 buffer.
    ///
    /// # Arguments
    ///
    /// * `codec_code` - The codec code, eg. 'base1'
    /// * `data` - the data to be prefixed
    ///
    /// # Example
    ///
    /// ```
    /// extern crate rust_multicodec;
    /// extern crate hex_slice;
    ///
    /// use hex_slice::AsHex;
    /// use std::process;
    ///
    /// fn main(){
    ///     let data="Live long and prosper";
    ///
    ///     println!("{:X}",rust_multicodec::add_prefix("base1",data.as_bytes()).unwrap().as_hex());
    ///     // it will print [1 4C 69 76 65 20 6C 6F 6E 67 20 61 6E 64 20 70 72 6F 73 70 65 72]
    /// }
    /// ```
    ///
    pub fn add_prefix(codec_code: &str, data: &[u8]) -> Result<Vec<u8>, &'static str> {
        match codec_map::get_decimal_by_code(codec_code) { // getting hex code of the codec
            Some(decimal) => {
                // encoding codec's (as decimal) into a varint
                let mut target:Vec<u8>=decimal.encode_var_vec();

                match target.write(data) {
                    Err(_) => return Err("Could not write data into the result buffer"),
                    _=> ()
                }

                Ok(target)
            }
            None => Err("No implementation for the given codec")
        }
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
    /// use hex_slice::AsHex;
    /// use std::process;
    ///
    /// fn main(){
    ///     let data="Live long and prosper";
    ///
    ///     let prefixed=rust_multicodec::add_prefix("base1",data.as_bytes()).unwrap();
    ///     println!("{}",rust_multicodec::get_codec(prefixed.as_slice()).unwrap().unwrap());
    ///     // it will print "base1"
    /// }
    /// ```
    ///
    pub fn get_codec(data: &[u8]) -> Result<Option<&'static str>, &'static str> {
        let decoded:(u64,usize)=u64::decode_var_vec(&Vec::from(data));
        Ok(codec_map::get_code_by_decimal(decoded.0))
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
    /// use hex_slice::AsHex;
    /// use std::process;
    ///
    /// fn main(){
    ///     let data="Live long and prosper";
    ///
    ///     let prefixed=rust_multicodec::add_prefix("base1",data.as_bytes()).unwrap();
    ///     let raw_data=rust_multicodec::remove_prefix(prefixed.as_slice()).unwrap();
    ///     println!("Original data was {:?}", String::from_utf8(raw_data).unwrap())
    ///     // it will print return "Original data was Live long and prosper"
    /// }
    /// ```
    ///
    pub fn remove_prefix(data:&[u8]) -> Result<Vec<u8>, &'static str>{
        let decoded:(u64,usize)=u64::decode_var_vec(&Vec::from(data));
        Ok(data[decoded.1..].to_vec())
    }
}



pub mod x{
    pub fn y(){}
}

pub mod encoding {
    use codec_prefix;

    #[derive(Debug)]
    pub enum Codec {
        JSON
    }

    pub fn encode<T>(codec: Codec, object: &T) -> Result<Vec<u8>,&'static str>{
        let data="Live long and prosper";
        println!("{:?}",codec);
        //add_prefix("json",data.as_bytes());
        let prefix = match codec {
            Codec::JSON => codec_prefix::add_prefix("json",data.as_bytes()),
            _ => Err("No implementation for the given Codec")
        };

        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA:&str="Live long and prosper";

    #[test]
    fn works(){
        let result=add_prefix("utp",DATA.as_bytes());
        assert_eq!(result.is_ok(),true);

        let prefixed=result.unwrap();
        assert_eq!(get_codec(prefixed.as_slice()).unwrap().unwrap(),"utp");
        assert_eq!(remove_prefix(prefixed.as_slice()).unwrap(),DATA.as_bytes());
    }

    #[test]
    #[should_panic]
    fn fails_with_invalid_codec(){
        add_prefix("invalid_codec",DATA.as_bytes()).unwrap();
    }
}
