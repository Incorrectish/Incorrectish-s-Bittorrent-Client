mod test {
    use crate::{decode_bencoded_value, bencode::BencodeData};
    use serde_json;

    macro_rules! ben_list(
        {} => { Vec::<BencodeData>::new().into() };
        { $($value:expr),+ } => {
            {
                let mut list = Vec::new();
                $( list.push(BencodeData::from($value)); )+

                BencodeData::from(list)
            }
        };
    );

    #[test]
    pub fn test_integer() {
        let pairs = [
            (-10567893421i64, "i-10567893421e".to_string()),
            (3421, "i3421e".to_string()),
            (-58782, "i-58782e".to_string()),
        ];
        for pair in pairs {
            match pair {
                (a, b) => assert_eq!(
                    serde_json::Value::Number(serde_json::Number::from(a)),
                    decode_bencoded_value(b)
                ),
            };
        }
    }

    #[test]
    pub fn test_string() {
        let pairs = [
            ("", "0:"),
            ("hello world", "11:hello world"),
            ("i", "1:i"),
            ("54321", "5:54321"),
        ];
        for pair in pairs {
            match pair {
                (a, b) => assert_eq!(
                    serde_json::Value::from(BencodeData::BencodeString(a.into())),
                    decode_bencoded_value(b.to_string())
                ),
            };
        }
    }

    #[test]
    pub fn test_basic_list() {
        println!["hello"];
        let pairs = [
            (ben_list!(), "le"),
            (ben_list!["abra", "cadabra"], "l4:abra7:cadabrae"),
            (ben_list!["spam", "eggs"], "l4:spam4:eggse"),
            (
                ben_list![ben_list!["list", "of", "lists"], ben_list!["like", "omygawd!"]],
                "ll4:list2:of5:listsel4:like8:omygawd!ee",
            ),
        ];
        for pair in pairs {
            match pair {
                (a, b) => assert_eq!(
                    serde_json::Value::from(a),
                    decode_bencoded_value(b.to_string())
                ),
            };
        }
    }


}
