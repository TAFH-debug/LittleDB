use crate::core::core::ErrorType;

pub fn decode(bin: Vec<u8>) -> Result<String, ErrorType> {
    let mut res: Vec<u8> = Vec::new();
    for i in bin {
        res.push(i >> 1)
    }

    match String::from_utf8(res) {
        Ok(n) => Ok(n),
        Err(_) => Err(ErrorType::InvalidArgument),
    }
}

pub fn encode(text: String) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for i in text.into_bytes() {
        res.push(i << 1)
    }
    res
}
