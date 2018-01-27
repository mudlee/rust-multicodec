#[derive(Debug)]
#[derive(PartialEq)]
pub enum CodecType {
    JSON,
}

impl CodecType {
    pub fn hex(&self) -> u64 {
        match *self {
            CodecType::JSON => 0xf01,
        }
    }

    pub fn by_hex(hex: u64) -> Option<CodecType> {
        match hex {
            0xf01 => Some(CodecType::JSON),
            _ => None,
        }
    }
}
