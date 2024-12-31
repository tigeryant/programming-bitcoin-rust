use std::io::Cursor;
use programming_bitcoin::{ecc::{point::Point, signature::Signature}, script::script::Script, transactions::{tx::Tx, tx_input::TxInput, tx_output::TxOutput}, utils::{base58::decode_base58, hash256::hash256, sig_hash_type::SigHashType}};

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
    let z = tx.sig_hash(&SigHashType::SigHashAll, 0, false); // assumes not p2sh
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
    // create a TxInput
    // this tx id is in big endian
    let prev_tx_id: [u8; 32] = hex::decode("0d6fe5213c0b3291f208cba8bfb59b7476dffacc4e5cb66f6eb20a080843a299").unwrap().try_into().unwrap();
    let prev_index: [u8; 4] = 13u32.to_le_bytes();
    let empty_script_sig = Script::new_empty_script();
    let sequence: [u8; 4] = hex::decode("ffffffff").unwrap().try_into().unwrap();
    let witness: Option<Vec<Vec<u8>>> = None;
    let height = None;
    let is_coinbase = false;

    let tx_in = TxInput::new(prev_tx_id, prev_index, empty_script_sig, sequence, witness, height, is_coinbase);

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
    let expected_serialized_tx1 = String::from("010000000199a24308080ab26e6fb65c4eccfadf76749bb5bfa8cb08f291320b3c21e56f0d0d00000000ffffffff02408af701000000001976a914d52ad7ca9b3d096a38e752c2018e6fbc40cdf26f88ac80969800000000001976a914507b27411ccf7f16f10297de6cef3f291623eddf88ac00000000");
    assert_eq!(output_serialized_tx1, expected_serialized_tx1);
}

#[test]
fn test_construct_testnet_tx() {
    // constructing the input
    // this is a testnet tx - from the faucet
    let prev_tx_id: [u8; 32] = hex::decode("1c7c86d5a25414c4dfb614f8138a6aec7aca30176fca8a260c7886cb97b480b5").unwrap().try_into().unwrap();
    let prev_index: [u8; 4] = 1u32.to_le_bytes(); // prev index 1
    let empty_script_sig = Script::new_empty_script();
    let sequence: [u8; 4] = hex::decode("ffffffff").unwrap().try_into().unwrap();
    let witness: Option<Vec<Vec<u8>>> = None;
    let height = None;
    let is_coinbase = false;

    let unsigned_input = TxInput::new(prev_tx_id, prev_index, empty_script_sig, sequence, witness, height, is_coinbase);
    
    // constructing the unsigned transaction
    let target_amount: u64 =  (0.00009_f64 * 100_000_000.0) as u64;
    let target_h160 = decode_base58("mwmPBaschd3ukQzVkwfL1sHcBBJSUcmb8L").unwrap();
    let target_script = Script::p2pkh_script(target_h160);
    let target_output = TxOutput::new(target_amount, target_script);
    
    // transaction is on testnet
    let unsigned_tx = Tx::new(1, vec![unsigned_input.clone()], vec![target_output.clone()], 0, true, false);
    // this is for signing mulitple inputs
    // let signing_data = vec![InputSigningData::new(0, String::from("ee0b031ef58f9014c5b4c641dbc29c0ca086926eebd00be7b8df2c4e13a15e23"), SigHashType::SigHashAll, unsigned_input)];
    let signed_input = unsigned_tx.sign_input(0, "ee0b031ef58f9014c5b4c641dbc29c0ca086926eebd00be7b8df2c4e13a15e23", SigHashType::SigHashAll, unsigned_input);

    let signed_tx = Tx::new(1, vec![signed_input], vec![target_output.clone()], 0, true, false);
    println!("Signed tx: {}", hex::encode(signed_tx.serialize()));

    // verify the input
    assert!(signed_tx.verify_input(SigHashType::SigHashAll, 0));
    // verify the whole transaction
    assert!(signed_tx.verify());
    // can add testmempoolaccept
}

// add tests for multiple inputs and outputs

#[test]
fn verify_p2sh_tx() {
    let raw_tx = hex::decode("0100000001868278ed6ddfb6c1ed3ad5f8181eb0c7a385aa0836f01d5e4789e6bd304d87221a000000db00483045022100dc92655fe37036f47756db8102e0d7d5e28b3beb83a8fef4f5dc0559bddfb94e02205a36d4e4e6c7fcd16658c50783e00c341609977aed3ad00937bf4ee942a8993701483045022100da6bee3c93766232079a01639d07fa869598749729ae323eab8eef53577d611b02207bef15429dcadce2121ea07f233115c6f09034c0be68db99980b9a6c5e75402201475221022626e955ea6ea6d98850c994f9107b036b1334f18ca8830bfff1295d21cfdb702103b287eaf122eea69030a0e9feed096bed8045c8b98bec453e1ffac7fbdbd4bb7152aeffffffff04d3b11400000000001976a914904a49878c0adfc3aa05de7afad2cc15f483a56a88ac7f400900000000001976a914418327e3f3dda4cf5b9089325a4b95abdfa0334088ac722c0c00000000001976a914ba35042cfe9fc66fd35ac2224eebdafd1028ad2788acdc4ace020000000017a91474d691da1574e6b3c192ecfb52cc8984ee7b6c568700000000").unwrap();
    let mut stream = Cursor::new(raw_tx);
    let tx = Tx::parse(&mut stream, false);
    let result = tx.verify_input(SigHashType::SigHashAll, 0);
    assert!(result);
}

#[test]
fn test_verify_p2sh_sig() {
    // input data from p160
    let raw_modified_tx = hex::decode("0100000001868278ed6ddfb6c1ed3ad5f8181eb0c7a385aa0836f01d5e4789e6bd304d87221a000000475221022626e955ea6ea6d98850c994f9107b036b1334f18ca8830bfff1295d21cfdb702103b287eaf122eea69030a0e9feed096bed8045c8b98bec453e1ffac7fbdbd4bb7152aeffffffff04d3b11400000000001976a914904a49878c0adfc3aa05de7afad2cc15f483a56a88ac7f400900000000001976a914418327e3f3dda4cf5b9089325a4b95abdfa0334088ac722c0c00000000001976a914ba35042cfe9fc66fd35ac2224eebdafd1028ad2788acdc4ace020000000017a91474d691da1574e6b3c192ecfb52cc8984ee7b6c56870000000001000000").unwrap();
    let z = hash256(&raw_modified_tx);
    let raw_sec = hex::decode("022626e955ea6ea6d98850c994f9107b036b1334f18ca8830bfff1295d21cfdb70").unwrap();
    let pubkey = Point::parse_to_s256_point(raw_sec); 
    let raw_der = hex::decode("3045022100dc92655fe37036f47756db8102e0d7d5e28b3beb83a8fef4f5dc0559bddfb94e02205a36d4e4e6c7fcd16658c50783e00c341609977aed3ad00937bf4ee942a89937").unwrap();
    let sig = Signature::parse(raw_der);
    dbg!(hex::encode(&z));
    // z: e71bfa115715d6fd33796948126f40a8cdd39f187e4afb03896795189fe1423c
    let is_valid = pubkey.verify(z, sig);
    assert!(is_valid);
}

// test p2wpkh (construction and verification)
// p2sh-p2pkh
// p2wsh
// p2sh-p2wsh

#[test]
fn test_verify_p2wsh_tx() {
    // use this structure - copied from above (invalid data for this test)
    // let raw_tx = hex::decode("0100000001868278ed6ddfb6c1ed3ad5f8181eb0c7a385aa0836f01d5e4789e6bd304d87221a000000db00483045022100dc92655fe37036f47756db8102e0d7d5e28b3beb83a8fef4f5dc0559bddfb94e02205a36d4e4e6c7fcd16658c50783e00c341609977aed3ad00937bf4ee942a8993701483045022100da6bee3c93766232079a01639d07fa869598749729ae323eab8eef53577d611b02207bef15429dcadce2121ea07f233115c6f09034c0be68db99980b9a6c5e75402201475221022626e955ea6ea6d98850c994f9107b036b1334f18ca8830bfff1295d21cfdb702103b287eaf122eea69030a0e9feed096bed8045c8b98bec453e1ffac7fbdbd4bb7152aeffffffff04d3b11400000000001976a914904a49878c0adfc3aa05de7afad2cc15f483a56a88ac7f400900000000001976a914418327e3f3dda4cf5b9089325a4b95abdfa0334088ac722c0c00000000001976a914ba35042cfe9fc66fd35ac2224eebdafd1028ad2788acdc4ace020000000017a91474d691da1574e6b3c192ecfb52cc8984ee7b6c568700000000").unwrap();
    // let mut stream = Cursor::new(raw_tx);
    // let tx = Tx::parse(&mut stream, false);
    // let result = tx.verify_input(SigHashType::SigHashAll, 0);
    // assert!(result);
}

#[test]
fn test_parse_coinbase_tx() {
    // testnet tx - id: 422495f7f5617a292fb0f57ea80907fc9b274006ec6b917cfd490c8a36bc4698
    let raw_tx = hex::decode("020000000001010000000000000000000000000000000000000000000000000000000000000000ffffffff2603d9fe360489147267537069646572506f6f6c2f312fa521f46501039e105204000000000000ffffffff05220200000000000022512028202d4b19bfed17d9f7f9528e4b0433c9b78399bbaf81aa4df4549e8eb527f61b58830c00000000160014f65616071e14d79e45b30c5e968ae40e6ecce95f0000000000000000266a24aa21a9edd0a75241bd531c250819e3261c497957a5930ec984a03af7a35ecb87c1abb83000000000000000002f6a2d434f52450164db24a662e20bbdf72d1cc6e973dbb2d12897d596a6689031f48a857d344e1a42fdb272bb15d6210000000000000000126a10455853415401120f080304111f1200130120000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    // arbitrary tx
    // let raw_tx = hex::decode("020000000190ed1fec18af658aaa3d3e076efa3e0609f25d0030b25b0615a42d93ea0c82fb000000006b483045022100a6dc7b0fdce5aa039f3904867706dfb4342c8fad4ec71dc53f6c8c5e2e272ca7022045b26f412c1e73eb6f5840634f13306976775b24f22a3ea743d64853394bb1170121033accfa473722be4d7480ec098262506410581b5c1a57894c92d03ea1adb898f9ffffffff02838e5e08000000001976a9140640edc25754a60f54f06e27f35e163ad18a2a7588ac0000000000000000536a4c5048454d49010070a8610022dd4b1c8762992cdd6a02112dcea08cc0affaaf67d55fc77a7dd03cff20b242d00c3d80dbae2b4995493d4c6de570b5e4f11e3817131d2ea1e26e4dab74cb7d421494e63525d6fe3600").unwrap();
    let mut stream = Cursor::new(raw_tx);
    let tx = Tx::parse(&mut stream, false);
    println!("{}", tx);    
}
