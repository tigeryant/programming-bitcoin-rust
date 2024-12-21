use std::io::Cursor;

use programming_bitcoin::network::network_envelope::NetworkEnvolope;

#[test]
fn test_parse_network_message() {
    let raw_message = hex::decode("f9beb4d976657261636b000000000000000000005df6e0e2").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_message);
    let output_message = NetworkEnvolope::parse(&mut stream);
    assert!(output_message.is_ok());
    println!("{}", output_message.unwrap());
}

#[test]
// test NetworkEnvelope::new, VersionMessage::new, VersionMessage::serialize
