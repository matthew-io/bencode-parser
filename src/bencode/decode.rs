use std::{collections::BTreeMap, error::Error};

const INT_TOKEN: u8 = b'i';
const DICT_TOKEN: u8 = b'd';
const LIST_TOKEN: u8 = b'l';

#[derive(Debug)]
pub enum Bencode {
    Int(i64),
    List(Vec<Bencode>),
    Bytes(Vec<u8>),
    Dict(BTreeMap<Vec<u8>, Bencode>)
}

// ints are formatted as "i<int>e"
// leading zeroes or negative zeroes are invalid
// params: byte slice
// returns: a bencode i64 variant and the remaining bytes of the given string
pub fn parse_int(str: &[u8]) -> Result<(Bencode, &[u8]), Box<dyn Error>>{
    let string = String::from_utf8_lossy(str);
    let e_index = string.find("e").expect("couldn't find e");

    let num_string = &string[..e_index]; 

    println!("{}", num_string);
    if num_string.starts_with("0") && num_string.len() > 2 || num_string.starts_with("-0") && num_string.len() > 2 {
        return Err("input string has leading zero".into())
    }

    let num = num_string.parse::<i64>().expect("couldn't parse num string");
    let remaining = &str[e_index + 1..];

    Ok((Bencode::Int(num), remaining))
}

// bytes are formatted as <byte_len>:<byte>
// params: byte slice
// returns: bencode byte variant and remaning bytes of the input byte slice
pub fn parse_bytes(str: &[u8]) -> Result<(Bencode, &[u8]), Box<dyn Error>> {
    let string = String::from_utf8_lossy(str);
    let colon_index = string.find(":").expect("couldn't find colon");
    let len_bytes = &str[..colon_index];
    let len = String::from_utf8_lossy(len_bytes).parse::<usize>().expect("couldn't parse length");

    let start = colon_index + 1;
    let end = start + len;

    if str.len() < end {
        return Err("input string is of incorrect length".into())
    }

    let bytes = str[start..end].to_vec();
    let remaining = &str[end..];

    Ok((Bencode::Bytes(bytes), remaining))
}

// lists are formatted as l<list>e
// a list can contain any other bencode type including lists themselves
// so some recursive parsing is necessary
// params: mutable byte slice
// returns bencode list variant and the remaining byte slice
pub fn parse_list(mut str: &[u8]) -> Result<(Bencode, &[u8]), Box<dyn Error>> {
    let mut items = Vec::new();

    while let Some(&b) = str.first() {
        if b == b'e' {
            return Ok((Bencode::List(items), &str[1..]));
        }

        let (item, remaining) = decode(str).expect("couldn't decode"); 
        items.push(item);
        str = remaining;
    }

    return Err("couldn't parse list".into())
}

// dicts in bencode are formatted as b<dict>e
// the keys in dicts are always bytes but the values,
// similarly to lists can be of any other bencode type
// params: mutable byte slice
// returns: bencode dict variant and the remaining byte slice 
pub fn parse_dict(mut str: &[u8]) -> Result<(Bencode, &[u8]), Box<dyn Error>> {
    let mut dict:BTreeMap<Vec<u8>, Bencode> = BTreeMap::new();

    while let Some(&b) = str.first() {
        if b == b'e' {
            return Ok((Bencode::Dict(dict), &str[1..]))
        }

        let (key_bencode, remaining) = parse_bytes(str).expect("couldn't parse key");
        let (value, remaining) = decode(remaining).expect("couldn't prase value");

        let key = match key_bencode {
            Bencode::Bytes(k) => k,
            _ => return Err("dict key is not a byte string".into())
        };
        dict.insert(key, value);
        str = remaining;
    }

    return Err("couldn't parse dict".into());
}


pub fn decode(str: &[u8]) -> Result<(Bencode, &[u8]), Box<dyn Error>> {
    match str.first() {
        Some(&INT_TOKEN) => parse_int(&str[1..]),
        Some(&LIST_TOKEN) => parse_list(&str[1..]),
        Some(&DICT_TOKEN) => parse_dict(&str[1..]),
        Some(b'0'..=b'9') => parse_bytes(str),
        _ => return Err("not a valid byte".into())
    }
}