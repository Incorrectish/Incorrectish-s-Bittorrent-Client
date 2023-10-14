use serde_json;
use std::{fs, io, error::Error, env};

use bencode_parser::{BencodeParser, BencodeToken};

mod bencode;
mod bencode_parser;
mod test;
// 'use strict';
// const fs = require('fs');
// const torrent = fs.readFileSync('puppy.torrent');
// console.log(torrent.toString('utf8'));

pub(crate) fn decode_bencoded_value(encoded_value: String) -> serde_json::Value {
    // println!("Argument is \n`{encoded_value}`");
    let mut bencode_parser = BencodeParser::new(encoded_value);
    bencode_parser.next_element().unwrap().unwrap().into()
}

fn main() -> Result<(), Box<dyn Error>> {
    // generate fuzz tests
    // let mut args = env::args();
    // let _ = args.next();
    // if let Some("decode") = args.next().as_deref() {
    //     let encoded_value = args.next().expect("Incorrect number of arguments");
    //     let decoded_value = decode_bencoded_value(encoded_value);
    //     dbg!(decoded_value);
    //     // println!("{}", decoded_value.to_string());
    // } else {
    //     println!("Unknown or incorrect number of arguments");
    // }
    let file_buf = fs::read("debian-12.2.0-arm64-netinst.iso.torrent").expect("Cannot read torrent file");
    // let result = String::from_utf8(file_buf.clone()).unwrap();
    let ascii_string: String = file_buf.into_iter().map(|byte| byte as char).collect();
    let value = decode_bencoded_value(ascii_string);
    dbg!(value);
    // let torrent_stff = &torrent_str[0..torrent.len()-1];
    // dbg!(torrent_stff);
    // println!("{}", decoded_str);
    Ok(())
}
