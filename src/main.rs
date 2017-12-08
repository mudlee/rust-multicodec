/*
Van egy block data, mondjuk {"mySecret":"This is my secret"}. Ezt el akarjuk tárolni ilyesmi formában: db.store(key, value), ahol a key egy hash, a value egy encoded formája a secret-nek.
Kell két module. 1: egy multiencode/decode modul, aminek beadod a block data-t, megmondod, hogy milyen encode-al kodolja el (json, xml, stb.).
Ő ebből fog csinálni egy ilyesmi formát: <xml><BINARIS_FORMAJA_AZ_XML_E_ALAKITOTT_JSONNAK>.
A decode része pedig fogja ezt az <xml><...> adatot, és visszaadja egy az egybe és csak kizárólag azt a json-t, amit az első mondatban írtam.
 Tehát két inputja van, egy encode tipus (json, xml, stb.), es maga az adat. Az outputja egy sime string, a fent irt formaban.
 A decode reszenel az input maga ez a string, az outputja pedig az eredeti json.

a kimenetnek nem sztringnek, hanem binarisnak kell lennie, ami szoveges formatum eseten siman a sztring utf-8-al, de lehet bson vagy mas binaris is.
Az <xml> prefix is egy int ertek, 1 vagy 2 bajton
*/

mod multicodec {
    pub struct DecodeResult {
        pub codec: Encoding,
        pub data: Vec<u8>
    }

    impl DecodeResult{
        pub fn rawData(&self) -> &Object {
            // TODO returns the original object that was encoded
        }
    }

    pub enum Encoding {
        json=1
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
    pub fn encode(encode: Encoding, object: Object) -> Result<Vec<u8>,&'static str>{
        // TODO

    }

    /// Decodes the given data
    ///
    /// # Arguments
    ///
    /// * `data` - the data that was encoded before
    ///
    /// # Returns
    /// Returns a DecodeResult
    pub fn decode(data: Vec<u8>) -> Result<DecodeResult,&'static str> {

    }
}

fn main(){
    // sample usage:
    let encoded:Vec<u8>=multicodec::encode(multicodec::Encoding::json, "any kind of data. bson, json, xml, anything");
    let decoded:multicodec::DecodeResult=multicodec::decode(encoded);

    decoded.codec; // json
    decoded.data; // binary version of the "any kind of data. bson, json, xml, anything"
    decoded.rawData(); // "any kind of data. bson, json, xml, anything"

}