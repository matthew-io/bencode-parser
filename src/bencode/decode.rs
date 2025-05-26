use std::error::Error;

const END_TOKEN: &str = "e";

pub fn parse_int(str: &[u8]) -> Result<u64, Box<dyn Error>>{
    let string = String::from_utf8_lossy(str);
    let e_index = string.find(END_TOKEN).expect("couldn't find e");

    let num_string = &string[1..e_index]; 

    if num_string.starts_with("0") && num_string.len() > 2 || num_string.starts_with("-0") && num_string.len() > 2 {
        return Err("num string is in an invalid format".into())
    }

    let num = num_string.parse::<u64>().expect("couldn't parse num string");
    Ok(num)
}

pub fn parse_bytes(str: &[u8]) {
}