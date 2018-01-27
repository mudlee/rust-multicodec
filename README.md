# rust_multicodec

Rust implementation of the multicodec specification.

## Install

First add this to your `Cargo.toml`

```toml
[dependencies]
rust_multicodec = "0.2.0"
```

Then run `cargo build`.

## Usage

### Encode / Decode

```Rust
extern crate rust_multicodec;

#[macro_use]
extern crate serde_derive;

use rust_multicodec::{codec, encode, decode};

#[derive(Serialize)]
#[derive(Debug)]
struct Person {
    name: String
}

fn main(){
    let to_be_encoded = Person{ name: String::from("sanyi") };
    let encoded = encode(codec::CodecType::JSON, &to_be_encoded).unwrap();
    println!("{:?}", encoded);
    // it will print: Ok([129, 30, 123, 34, 110, 97, 109, 101, 34, 58, 34, 115, 97, 110, 121, 105, 34, 125])
    
    let decoded: Person = decode(encoded.as_ref()).unwrap().data;
    println!("{:?}", decoded);
    // it will print: Person { name: "sanyi" }
}
```

## Contribute

Contributions are welcome :)

## More reading
* Multicodec: https://github.com/multiformats/multicodec
* VarInt: https://developers.google.com/protocol-buffers/docs/encoding
* Multiformats VarInt: https://github.com/multiformats/unsigned-varint

## License

Licensed under the Apache License, Version 2.0