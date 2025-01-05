use std::io::Cursor;
use programming_bitcoin::{ecc::{point::Point, signature::Signature}, script::script::Script, transactions::{tx::Tx, tx_fetcher::TxFetcher, tx_input::TxInput, tx_output::TxOutput}, utils::{base58::decode_base58, hash256::hash256, sig_hash_type::SigHashType}};
use tokio::task;

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
    
    // Add sleep to avoid rate limiting
    std::thread::sleep(std::time::Duration::from_millis(1750));
    
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

    let tx_in = TxInput::new(prev_tx_id, prev_index, empty_script_sig, sequence, witness, height);

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

    let unsigned_input = TxInput::new(prev_tx_id, prev_index, empty_script_sig, sequence, witness, height);
    
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

    // Add sleep to avoid rate limiting
    std::thread::sleep(std::time::Duration::from_millis(1750));

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
    // let raw_tx = hex::decode("020000000001010000000000000000000000000000000000000000000000000000000000000000ffffffff2603d9fe360489147267537069646572506f6f6c2f312fa521f46501039e105204000000000000ffffffff05220200000000000022512028202d4b19bfed17d9f7f9528e4b0433c9b78399bbaf81aa4df4549e8eb527f61b58830c00000000160014f65616071e14d79e45b30c5e968ae40e6ecce95f0000000000000000266a24aa21a9edd0a75241bd531c250819e3261c497957a5930ec984a03af7a35ecb87c1abb83000000000000000002f6a2d434f52450164db24a662e20bbdf72d1cc6e973dbb2d12897d596a6689031f48a857d344e1a42fdb272bb15d6210000000000000000126a10455853415401120f080304111f1200130120000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    // arbitrary tx
    // let raw_tx = hex::decode("020000000190ed1fec18af658aaa3d3e076efa3e0609f25d0030b25b0615a42d93ea0c82fb000000006b483045022100a6dc7b0fdce5aa039f3904867706dfb4342c8fad4ec71dc53f6c8c5e2e272ca7022045b26f412c1e73eb6f5840634f13306976775b24f22a3ea743d64853394bb1170121033accfa473722be4d7480ec098262506410581b5c1a57894c92d03ea1adb898f9ffffffff02838e5e08000000001976a9140640edc25754a60f54f06e27f35e163ad18a2a7588ac0000000000000000536a4c5048454d49010070a8610022dd4b1c8762992cdd6a02112dcea08cc0affaaf67d55fc77a7dd03cff20b242d00c3d80dbae2b4995493d4c6de570b5e4f11e3817131d2ea1e26e4dab74cb7d421494e63525d6fe3600").unwrap();
    let raw_tx = hex::decode("0100000001868278ed6ddfb6c1ed3ad5f8181eb0c7a385aa0836f01d5e4789e6bd304d87221a000000db00483045022100dc92655fe37036f47756db8102e0d7d5e28b3beb83a8fef4f5dc0559bddfb94e02205a36d4e4e6c7fcd16658c50783e00c341609977aed3ad00937bf4ee942a8993701483045022100da6bee3c93766232079a01639d07fa869598749729ae323eab8eef53577d611b02207bef15429dcadce2121ea07f233115c6f09034c0be68db99980b9a6c5e75402201475221022626e955ea6ea6d98850c994f9107b036b1334f18ca8830bfff1295d21cfdb702103b287eaf122eea69030a0e9feed096bed8045c8b98bec453e1ffac7fbdbd4bb7152aeffffffff04d3b11400000000001976a914904a49878c0adfc3aa05de7afad2cc15f483a56a88ac7f400900000000001976a914418327e3f3dda4cf5b9089325a4b95abdfa0334088ac722c0c00000000001976a914ba35042cfe9fc66fd35ac2224eebdafd1028ad2788acdc4ace020000000017a91474d691da1574e6b3c192ecfb52cc8984ee7b6c568700000000").unwrap();
    let mut stream = Cursor::new(raw_tx);
    let tx = Tx::parse(&mut stream, false);
    println!("{}", tx);    
}

#[test]
fn test_fetch_tx() {
    let tx_id = "422495f7f5617a292fb0f57ea80907fc9b274006ec6b917cfd490c8a36bc4698"; // not working with this (coinbase) tx
    // let tx_id = "1fde1c2867578910d1e1478ac7a991492aab3782b381572b3ccc41ef0acf878c"; // (coinbase) tx
    // let tx_id = "56bf2aa92ea6ed860cdee803bcb0f132648ce3b00844b0d965a2a330d44a9391";
    let testnet = true;
    let fresh = true;
    let fetcher = TxFetcher::build();
    assert!(TxFetcher::fetch(&fetcher, tx_id, testnet, fresh).is_ok());
}

#[test]
fn test_parse_hash() {
    let raw_tx = hex::decode("020000000001010000000000000000000000000000000000000000000000000000000000000000ffffffff26033b093704ffa07467537069646572506f6f6c2f312f398032a80103b606ed19000000000000ffffffff05220200000000000022512028202d4b19bfed17d9f7f9528e4b0433c9b78399bbaf81aa4df4549e8eb527f68598c82100000000160014f65616071e14d79e45b30c5e968ae40e6ecce95f0000000000000000266a24aa21a9edd8c74c43129f200d0bedbe6fe75a161f28ca6b8ce9fa48fb0752f8ec950907b400000000000000002f6a2d434f52450164db24a662e20bbdf72d1cc6e973dbb2d12897d596a6689031f48a857d344e1a42fdb272bb15d6210000000000000000126a10455853415401120f080304111f1200130120000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let mut stream = Cursor::new(raw_tx);
    let tx = Tx::parse(&mut stream, false);
    println!("{}", tx);
    let txid = tx.id();
    println!("TXID: {}", txid);
}

#[tokio::test]
async fn test_identify_p2wpkh() {
    // mainnet tx
    let raw_tx = hex::decode("020000000001016972546966be990440a0665b73d0f4c3c942592d1f64d1033717aaa3e2c2ec910000000000fdffffff01610a0200000000001976a91476c6195adcbea5c8656d33e8af0567833e63b8c988ac024730440220424c69a855dc79b1f34d9a2ae88b4269988f4dc1dff697fc0d32b4bcfb70a36d022058c359af022f0db3bd37cbe8a426e5218ce61c761b161668883312f1055745550121022a263d5273494ce9247387770ae66e6989b665aaf8fade4403fd1b06601b9cdf9d640a00").unwrap();
    let mut stream = Cursor::new(raw_tx);
    let testnet = false;
    let tx = Tx::parse(&mut stream, testnet);

    for input in tx.tx_ins.iter() {
        // guard againt coinbase tx input
        if input.prev_tx_id == [0u8; 32] {
            println!("coinbase tx input - no script type");
            continue;
        }

        let input_clone = input.clone();
        let script_pubkey = task::spawn_blocking(move || input_clone.script_pubkey(testnet))
            .await
            .unwrap();
        
        if script_pubkey.is_p2wpkh_script_pubkey() {
            println!("Found P2WPKH tx: {}", tx.id());
            println!("{}", tx);
            println!("Script_pubkey:");
            println!("{}", script_pubkey);
            return;
        }
    }
}

#[test]
fn test_verify_p2wpkh_input() {
    // mainnet tx - debug further
    // let raw_tx = hex::decode("020000000001016972546966be990440a0665b73d0f4c3c942592d1f64d1033717aaa3e2c2ec910000000000fdffffff01610a0200000000001976a91476c6195adcbea5c8656d33e8af0567833e63b8c988ac024730440220424c69a855dc79b1f34d9a2ae88b4269988f4dc1dff697fc0d32b4bcfb70a36d022058c359af022f0db3bd37cbe8a426e5218ce61c761b161668883312f1055745550121022a263d5273494ce9247387770ae66e6989b665aaf8fade4403fd1b06601b9cdf9d640a00").unwrap();
    // testnet tx
    let raw_tx = hex::decode("0100000000010115e180dc28a2327e687facc33f10f2a20da717e5548406f7ae8b4c811072f8560100000000ffffffff0100b4f505000000001976a9141d7cd6c75c2e86f4cbf98eaed221b30bd9a0b92888ac02483045022100df7b7e5cda14ddf91290e02ea10786e03eb11ee36ec02dd862fe9a326bbcb7fd02203f5b4496b667e6e281cc654a2da9e4f08660c620a1051337fa8965f727eb19190121038262a6c6cec93c2d3ecd6c6072efea86d02ff8e3328bbd0242b20af3425990ac00000000").unwrap();
    let mut stream = Cursor::new(raw_tx);
    let tx = Tx::parse(&mut stream, true);
    println!("{}", tx);
    let result = tx.verify_input(SigHashType::SigHashAll, 0);
    assert!(result);
}

// verify:
// p2sh - see above
// p2sh_p2wsh - fails (signature checking)
// p2wpkh - fails on mainnet (signature checking)

#[test]
fn test_verify_p2sh_p2wpkh_input() {
    // mainnet tx - id: c586389e5e4b3acb9d6c8be1c19ae8ab2795397633176f5a6442a261bbdefc3a
    let raw_tx = hex::decode("0200000000010140d43a99926d43eb0e619bf0b3d83b4a31f60c176beecfb9d35bf45e54d0f7420100000017160014a4b4ca48de0b3fffc15404a1acdc8dbaae226955ffffffff0100e1f5050000000017a9144a1154d50b03292b3024370901711946cb7cccc387024830450221008604ef8f6d8afa892dee0f31259b6ce02dd70c545cfcfed8148179971876c54a022076d771d6e91bed212783c9b06e0de600fab2d518fad6f15a2b191d7fbd262a3e0121039d25ab79f41f75ceaf882411fd41fa670a4c672c23ffaf0e361a969cde0692e800000000").unwrap();
    let mut stream = Cursor::new(raw_tx);
    let tx = Tx::parse(&mut stream, false);
    println!("{}", tx);
    let result = tx.verify_input(SigHashType::SigHashAll, 0);
    assert!(result);
}

#[test]
fn test_verify_p2wsh_input() {
    // testnet tx - id: 78457666f82c28aa37b74b506745a7c7684dc7842a52a457b09f09446721e11c
    let raw_tx = hex::decode("0100000000010115e180dc28a2327e687facc33f10f2a20da717e5548406f7ae8b4c811072f8560200000000ffffffff0188b3f505000000001976a9141d7cd6c75c2e86f4cbf98eaed221b30bd9a0b92888ac02483045022100f9d3fe35f5ec8ceb07d3db95adcedac446f3b19a8f3174e7e8f904b1594d5b43022074d995d89a278bd874d45d0aea835d3936140397392698b7b5bbcdef8d08f2fd012321038262a6c6cec93c2d3ecd6c6072efea86d02ff8e3328bbd0242b20af3425990acac00000000").unwrap();
    let mut stream = Cursor::new(raw_tx);
    let tx = Tx::parse(&mut stream, true);
    println!("{}", tx);
    let result = tx.verify_input(SigHashType::SigHashAll, 0);
    assert!(result);
}

#[test]
#[ignore]
fn test_verify_p2sh_p2wsh_input() {
    // todo!("update the input tx for this tx - find a valid p2sh_p2wsh tx to test. op_checksig failing");
    // testnet tx - id: 954f43dbb30ad8024981c07d1f5eb6c9fd461e2cf1760dd1283f052af746fc88
    let raw_tx = hex::decode("0100000000010115e180dc28a2327e687facc33f10f2a20da717e5548406f7ae8b4c811072f856040000002322002001d5d92effa6ffba3efa379f9830d0f75618b13393827152d26e4309000e88b1ffffffff0188b3f505000000001976a9141d7cd6c75c2e86f4cbf98eaed221b30bd9a0b92888ac02473044022038421164c6468c63dc7bf724aa9d48d8e5abe3935564d38182addf733ad4cd81022076362326b22dd7bfaf211d5b17220723659e4fe3359740ced5762d0e497b7dcc012321038262a6c6cec93c2d3ecd6c6072efea86d02ff8e3328bbd0242b20af3425990acac00000000").unwrap();
    let mut stream = Cursor::new(raw_tx);
    let tx = Tx::parse(&mut stream, true);
    println!("{}", tx);
    let result = tx.verify_input(SigHashType::SigHashAll, 0);
    assert!(result);
}
