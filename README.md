# tungstenite simple example

## learning

- simple read/write example from tungstenite
- tungstenite::Message enum study and parsing
- Error handling from String::from_utf8(v) vs String::from_utf8_lossy(v)
- .expect("error message") vs .unwrap()
- into_owned when internally a clone on writem, apparently this might not be always necessary
ref: https://stackoverflow.com/questions/19076719/how-do-i-convert-a-vector-of-bytes-u8-to-a-string
- "sigshfdgs".to_string()
- serde_json::from_str expects a String whihc will have to be in the correct format or the parsing will panic otherwise
added exmaples returning correct defaults for Pings and Pongs