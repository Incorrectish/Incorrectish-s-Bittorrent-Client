use std::collections::BTreeMap;
use serde_json::{Value, Map};

#[derive(Debug)]
pub enum BencodeData {
    BencodeInteger(i64),
    BencodeString(String),
    BencodeList(Vec<BencodeData>),
    BencodeDict(BTreeMap<String, BencodeData>)
}

impl From<i64> for BencodeData {
    fn from(value: i64) -> Self {
        BencodeData::BencodeInteger(value)
    }
}

impl From<String> for BencodeData {
    fn from(value: String) -> Self {
        BencodeData::BencodeString(value)
    }
}

impl<'a> From<&'a str> for BencodeData {
    fn from(value: &'a str) -> Self {
        BencodeData::BencodeString(value.to_string())
    }
}

impl From<Vec<BencodeData>> for BencodeData {
    fn from(value: Vec<BencodeData>) -> Self {
        BencodeData::BencodeList(value)
    }
}

impl From<BTreeMap<String, BencodeData>> for BencodeData {
    fn from(value: BTreeMap<String, BencodeData>) -> Self {
        BencodeData::BencodeDict(value)
    }
}

impl From<BencodeData> for Value {
    fn from(bencode: BencodeData) -> Value {
        match bencode {
            BencodeData::BencodeInteger(i) => Value::Number(serde_json::Number::from(i)),
            BencodeData::BencodeString(s) => Value::String(s),
            BencodeData::BencodeList(list) => {
                let json_array: Vec<Value> = list.into_iter().map(|item| item.into()).collect();
                Value::Array(json_array)
            }
            BencodeData::BencodeDict(dict) => {
                let mut json_dict = Map::new();
                for (key, value) in dict {
                    json_dict.insert(key, value.into());
                }
                Value::Object(json_dict)
            }
        }
    }
}
