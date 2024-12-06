use std::io::Cursor;
use programming_bitcoin::{ecc::{point::Point, signature::Signature}, transactions::tx::Tx, utils::sig_hash_type::SigHashType};

// add tests here for parsing the individual components of the tx - version, inputs, outputs, locktime (and testnet?)
#[test]
fn test_parse_tx() { // run with --nocapture to see the debug
    // mainnet tx
    let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
    let mut stream = Cursor::new(raw_tx);
    let tx = Tx::parse(&mut stream, false);
    println!("{}", tx);
}

#[test]
fn test_sig_hash() {
    // mainnet tx
    let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
    let mut stream = Cursor::new(raw_tx);
    let tx = Tx::parse(&mut stream, false);

    // tests with sighash all
    let z = tx.sig_hash(SigHashType::SigHashAll, 0);
    let z_hex = hex::encode(z);
    print!("Z: 0x{}", z_hex);
    let output = z_hex;
    let expected = String::from("27e0c5994dec7824e56dec6b2fcb342eb7cdb0d0957c2fce9882f715e85d81a6");
    assert_eq!(expected, output);
}

#[test]
fn test_verify_signature() {
    let z = hex::decode("27e0c5994dec7824e56dec6b2fcb342eb7cdb0d0957c2fce9882f715e85d81a6").unwrap();
    let sec = hex::decode("0349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278a").unwrap();
    let der = hex::decode("3045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed").unwrap();

    let point: Point = Point::parse_to_s256_point(sec);
    let signature: Signature = Signature::parse(der);

    let result = point.verify(z, signature);
    assert!(result);
}

#[test]
fn test_verify_input() {
    // mainnet tx
    let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
    let mut stream = Cursor::new(raw_tx);
    let tx = Tx::parse(&mut stream, false);
    let result = tx.verify_input(SigHashType::SigHashAll, 0);
    assert!(result);
}
