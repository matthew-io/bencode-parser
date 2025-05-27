use std::{collections::BTreeMap, error::Error};

use super::decode::Bencode;

pub fn encode(data: &Bencode) -> Vec<u8> {
    match data {
        Bencode::Int(i) => format!("i{}e", i).into_bytes(),
        Bencode::Bytes(b) => [b.len().to_string().into_bytes(), vec![b':'], b.clone()].concat(),
        Bencode::List(items) => {
            let mut v = vec![b'l'];
            for item in items {
                v.extend(encode(item))
            }
            v.push(b'e');
            v
        },
        Bencode::Dict(map) => {
            let mut v = vec![b'd'];
            for (key, value) in map {
                v.extend(encode(&Bencode::Bytes(key.clone())));
                v.extend(encode(value));
            }
            v.push(b'e');
            v
        }
    }
}

