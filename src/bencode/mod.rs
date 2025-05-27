pub mod decode;
pub mod encode;

use std::{collections::BTreeMap, error::Error};


const INT_TOKEN: u8 = b'i';
const DICT_TOKEN: u8 = b'd';
const LIST_TOKEN: u8 = b'l';