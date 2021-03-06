#[macro_export]
macro_rules! read_string {
    ($stream:ident) => {{
        let mut buf = [0; 1];
        $stream.read(&mut buf);
        let len = buf.first().unwrap();
        let mut vec_str = Vec::new();
        for _ in 0..*len {
            let mut buf2 = [0; 1];
            $stream.read(&mut buf2);
            vec_str.push(*buf2.first().unwrap());
        }
        String::from_utf8(vec_str).unwrap()
    }};
}

/// Checks the result and returns error if result is error.
#[macro_export]
macro_rules! check_error_f {
    ($func:expr) => {
        match $func {
            Ok(n) => n,
            Err(n) => return Err(n)
        }
    };
}

#[macro_export]
macro_rules! check_error_e {
    ($func:expr) => {
        match $func {
            Ok(0) => return Err("Unexpected EOF".to_string()),
            Ok(n) => (),
            Err(n) => return Err(n.to_string())
        }
    };
}
