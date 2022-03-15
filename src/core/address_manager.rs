pub fn decode(bin: Vec<u8>) -> Result<String, String> {
    let mut res: Vec<u8> = Vec::new();
    for i in bin { res.push(i >> 1) }

    match String::from_utf8(res) {
        Ok(n) => Ok(n),
        Err(_) => Err(String::from("String decoding error"))
    }
}

pub fn encode(text: String) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for i in text.into_bytes() { res.push(i << 1) }
    res
}