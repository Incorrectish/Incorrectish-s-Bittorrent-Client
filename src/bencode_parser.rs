use std::{vec::IntoIter, collections::BTreeMap};

use crate::bencode::BencodeData;

pub struct BencodeParser {
    chars: IntoIter<char>,
}

pub enum BencodeToken {
    Data(BencodeData),
    End,
}

impl BencodeToken {
    pub fn unwrap(self) -> BencodeData {
        match self {
            Self::Data(data) => data,
            Self::End => panic!("No data to release")
        }
    }
}

impl BencodeParser {
    pub fn new(data: String) -> Self {
        let chars: IntoIter<char> = data.chars().collect::<Vec<_>>().into_iter();
        BencodeParser { chars }
    }

    pub fn next_element(&mut self) -> Option<BencodeToken> {
        let curr_char = self.chars.next();
        match curr_char {
            Some(mut c) if c.is_ascii_digit() => {
                let mut num_str = format!("{c}");
                c = self.chars.next().expect("Invalid Bencode Format, ends on a number");
                while c.is_ascii_digit() || c.is_whitespace() {
                    if c.is_whitespace() {
                        c = self.chars.next().expect("Invalid Bencode Format, ends on a number");
                    } else {
                        // C is a digit
                        num_str.push(c);
                        c = self.chars.next().expect("Invalid Bencode Format, ends on a number");
                    }
                }
                while c.is_whitespace() {
                    c = self.chars.next().expect("Invalid Bencode Format, ends on a number");
                }
                if c != ':' {
                    panic!("Invalid Bencode Format");
                }
                // c == ':' and num is a valid number
                println!("`{num_str}`");
                let num = num_str.parse::<usize>().unwrap();
                let mut string = String::with_capacity(num);
                for i in 0..num {
                    string.push(self.chars.next().expect(&format!("Invalid Bencode Format, string is not long enough, expected {num} characters, got {i} characters")));
                }
                Some(BencodeToken::Data(BencodeData::BencodeString(string)))
            }
            Some(mut c) if c == 'i' => {
                let mut num_str = String::new();
                loop {
                    c = self.chars.next().expect("Invalid Format, number specified, but ended without information");
                    if c == 'e' {
                        break;
                    }
                    if c != '-' &&!c.is_ascii_digit() {
                        panic!("Invalid format, expected digit or '-', got {}", c);
                    }
                    num_str.push(c);
                }
                Some(BencodeToken::Data(BencodeData::BencodeInteger(num_str.parse::<i64>().expect("Cannot have a - sign within the integer, no expressions"))))

            }
            Some(c) if c == 'l' => {
                let mut bencode_elem_list = Vec::new();
                loop {
                    let next_token = self.next_element().expect("Invalid bencode format, expected elements after a list");
                    match next_token {
                        BencodeToken::Data(data) => {bencode_elem_list.push(data)}
                        BencodeToken::End => {break}
                    }
                }
                Some(BencodeToken::Data(BencodeData::BencodeList(bencode_elem_list)))
            }
            Some(c) if c == 'd' => {
                // while loop, get next token, make sure it's a string, get next token for value make a hashmap
                // TODO: this is just copy pasted list code
                let mut bencode_elem_dict = BTreeMap::new();
                loop {
                    let key = self.next_element().expect("Invalid bencode format, expected elements after a list");
                    match key {
                        BencodeToken::Data(data) => {
                            match data {
                                BencodeData::BencodeString(str) => {
                                    let value = self.next_element().expect("Invalid bencode format, expected elements after a list");
                                    match value {
                                        BencodeToken::Data(inner_data) => bencode_elem_dict.insert(str, inner_data),
                                        BencodeToken::End => panic!("Invalid Bencode Dictionary Format, every key must have a corresponding value"),
                                    };
                                }
                                _ => panic!("Invalid Bencode Dictionary Format, every key must be a string")
                            }
                        }
                        BencodeToken::End => {break}
                    }
                }
                Some(BencodeToken::Data(BencodeData::BencodeDict(bencode_elem_dict)))
            }
            Some(c) if c == 'e' => {
                Some(BencodeToken::End)
            }
            Some(c) if c.is_whitespace() => {
                self.next_element()
            }
            Some(c) => {println!("Invalid format bencode, expected one of [\\d, l, i, e, d], got `{c}`"); std::process::exit(1)}
            None => None
        }
    }
}
