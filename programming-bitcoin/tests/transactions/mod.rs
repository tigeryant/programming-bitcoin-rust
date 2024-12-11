use std::io::Cursor;
use primitive_types::U256;
use programming_bitcoin::{ecc::{point::Point, private_key::PrivateKey, signature::Signature}, script::script::Script, transactions::{input_signing_data::InputSigningData, tx::Tx, tx_input::TxInput, tx_output::TxOutput}, utils::{base58::decode_base58, sig_hash_type::SigHashType}};

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
    let z = tx.sig_hash(&SigHashType::SigHashAll, 0);
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

#[test]
fn test_create_p2pkh_tx() {
    // create a TxInput, passing the prev_tx_id, the prev_index, an empty script_sig and a sequence number
    // this tx id is in big endian
    let prev_tx_id: [u8; 32] = hex::decode("0d6fe5213c0b3291f208cba8bfb59b7476dffacc4e5cb66f6eb20a080843a299").unwrap().try_into().unwrap();
    let prev_index: [u8; 4] = 13u32.to_le_bytes();
    let empty_script_sig = Script::new_empty_script();
    let sequence: [u8; 4] = hex::decode("ffffffff").unwrap().try_into().unwrap();
    let witness: Option<Vec<Vec<u8>>> = None;

    let tx_in = TxInput::new(prev_tx_id, prev_index, empty_script_sig, sequence, witness);

    let change_amount: u64 = (0.33_f64 * 100_000_000.0) as u64;
    let change_h160 = decode_base58("mzx5YhAH9kNHtcN481u6WkjeHjYtVeKVh2").unwrap();
    let change_script = Script::p2pkh_script(change_h160);
    let change_output = TxOutput::new(change_amount, change_script);

    let target_amount: u64 =  (0.1_f64 * 100_000_000.0) as u64;
    let target_h160 = decode_base58("mnrVtF8DWjMu839VW3rBfgYaAfKk8983Xf").unwrap();
    let target_script = Script::p2pkh_script(target_h160);
    let target_output = TxOutput::new(target_amount, target_script);

    // transaction input is on testnet
    let tx = Tx::new(1, vec![tx_in], vec![change_output.clone(), target_output.clone()], 0, true, false);
    let output_serialized_tx1 = hex::encode(tx.serialize());
    // println!("{serialized_tx1}");
    // println!("{tx}");

    let expected_serialized_tx1 = String::from("010000000199a24308080ab26e6fb65c4eccfadf76749bb5bfa8cb08f291320b3c21e56f0d0d00000000ffffffff02408af701000000001976a914d52ad7ca9b3d096a38e752c2018e6fbc40cdf26f88ac80969800000000001976a914507b27411ccf7f16f10297de6cef3f291623eddf88ac00000000");
    assert_eq!(output_serialized_tx1, expected_serialized_tx1);

    /*
    // signing the transaction
    let z = tx.sig_hash(SigHashType::SigHashAll, 0);
    let z_u256 = U256::from_big_endian(&z);
    let private_key = PrivateKey::new(U256::from(8675309));
    let der = private_key.sign(z_u256).der();

    // the signature is the DER signature concatenated with the sighash
    // this will not actually be of type Signature
    // let sig = der.concat(SigHashType::SigHashAll)
    let sig = [der, vec![SigHashType::SigHashAll as u8]].concat(); // maybe this should be u32?
    let sec = private_key.point().sec(true); // assuming compressed is true
    let script_sig = Script::new(vec![sig, sec]);

    let modified_tx_in = TxInput::new(prev_tx_id, prev_index, script_sig, sequence, None);
    let new_tx = Tx::new(1, vec![modified_tx_in], vec![change_output, target_output], 0, true, false);
    let serialized_tx = new_tx.serialize();
    let output_tx2_hex = hex::encode(&serialized_tx);
    // println!("Serialized transaction (hex): {}", tx_hex);

    // let expected_tx2_hex = hex::encode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006a47304402207db2402a3311a3b845b038885e3dd889c08126a8570f26a844e3e4049c482a11022010178cdca4129eacbeab7c44648bf5ac1f9cac217cd609d216ec2ebc8d242c0a012103935581e52c354cd2f484fe8ed83af7a3097005b2f9c60bff71d35bd795f54b67feffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600");
    let expected_tx2_hex = String::from("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006a47304402207db2402a3311a3b845b038885e3dd889c08126a8570f26a844e3e4049c482a11022010178cdca4129eacbeab7c44648bf5ac1f9cac217cd609d216ec2ebc8d242c0a012103935581e52c354cd2f484fe8ed83af7a3097005b2f9c60bff71d35bd795f54b67feffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600");
    // assert_eq!(expected_tx2_hex, output_tx2_hex);
    // print the serialized tx as hex
    // add assertions
     */

}

#[test]
fn test_sign_tx() {

}

#[test]
fn test_construct_testnet_tx() {
    // constructing the input
    // note - this is a testnet tx - from the faucet
    let prev_tx_id: [u8; 32] = hex::decode("1c7c86d5a25414c4dfb614f8138a6aec7aca30176fca8a260c7886cb97b480b5").unwrap().try_into().unwrap();
    let prev_index: [u8; 4] = 1u32.to_le_bytes(); // prev index 1
    let empty_script_sig = Script::new_empty_script();
    let sequence: [u8; 4] = hex::decode("ffffffff").unwrap().try_into().unwrap();

    // UTXO value is 0.00016214
    let unsigned_input = TxInput::new(prev_tx_id, prev_index, empty_script_sig, sequence, None);
    
    // constructing the unsigned transaction
    // works with 0.0007, 8, 9
    let target_amount: u64 =  (0.00009_f64 * 100_000_000.0) as u64;
    let target_h160 = decode_base58("mwmPBaschd3ukQzVkwfL1sHcBBJSUcmb8L").unwrap();
    let target_script = Script::p2pkh_script(target_h160);
    let target_output = TxOutput::new(target_amount, target_script);
    
    // transaction is on testnet
    let unsigned_tx = Tx::new(1, vec![unsigned_input.clone()], vec![target_output.clone()], 0, true, false);
    // let signing_data = vec![InputSigningData::new(0, String::from("ee0b031ef58f9014c5b4c641dbc29c0ca086926eebd00be7b8df2c4e13a15e23"), SigHashType::SigHashAll, unsigned_input)];
    let signed_input = unsigned_tx.sign_input(0, "ee0b031ef58f9014c5b4c641dbc29c0ca086926eebd00be7b8df2c4e13a15e23", SigHashType::SigHashAll, unsigned_input);

    let signed_tx = Tx::new(1, vec![signed_input], vec![target_output.clone()], 0, true, false);
    println!("Signed tx: {}", hex::encode(signed_tx.serialize()));

    // verify the input
    assert!(signed_tx.verify_input(SigHashType::SigHashAll, 0));
    // verify the whole transaction
    assert!(signed_tx.verify());
    // testmempoolaccept
    // assertions
}

// add tests for multiple inputs and outputs
