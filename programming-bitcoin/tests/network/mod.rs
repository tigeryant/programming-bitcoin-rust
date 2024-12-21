use std::io::Cursor;

use programming_bitcoin::network::network_envelope::NetworkEnvolope;
use programming_bitcoin::network::network_envelope::{TESTNET_NETWORK_MAGIC, MAINNET_NETWORK_MAGIC};


#[test]
fn test_new_network_message() {
    let command: [u8; 12] = NetworkEnvolope::command_as_array("verack");
    let payload = hex::decode("f9beb4d976657273696f6e0000000000650000005f1a69d2721101000100000000000000bc8f5e5400000000010000000000000000000000000000000000ffffc61b6409208d010000000000000000000000000000000000ffffcb0071c0208d128035cbc97953f80f2f5361746f7368693a302e392e332fcf05050001").unwrap();
    let testnet = true;
    let network_message = NetworkEnvolope::new(command, payload.clone(), testnet);

    let message_testnet = match network_message.magic {
        TESTNET_NETWORK_MAGIC => true,
        MAINNET_NETWORK_MAGIC => false,
        _ => false
    };

    assert_eq!(message_testnet, testnet);
    assert_eq!(network_message.command, command);
    assert_eq!(network_message.payload, payload);
}

#[test]
fn test_parse_network_message() {
    let raw_message = hex::decode("f9beb4d976657261636b000000000000000000005df6e0e2").unwrap();
    let mut stream: Cursor<Vec<u8>> =  Cursor::new(raw_message);
    let output_message = NetworkEnvolope::parse(&mut stream);
    assert!(output_message.is_ok());
    println!("{}", output_message.unwrap());
}

// test VersionMessage::new, VersionMessage::serialize
#[test]
fn test_new_version_message() {

}

#[test]
fn test_serialize_version_message() {

}
