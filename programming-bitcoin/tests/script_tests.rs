use programming_bitcoin::{ecc::signature::Signature, script::script::Script};
// use std::io::Cursor;

// TODO test the parse (and serialize?) methods

#[test]
fn evaluate_p2pk() {
    let z = hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap();
    // let mut stream = Cursor::new(z);
    let raw_sec = hex::decode("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap();
    let pubkey_commands = vec![raw_sec, vec![0xac]]; // SEC + OP_CHECKSIG (172)
    let script_pubkey = Script::new(pubkey_commands);

    let raw_sig = hex::decode("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601").unwrap();
    let script_sig = Script::new(vec![raw_sig]);
    let combined_script = script_sig.concat(script_pubkey);

    let result = combined_script.evaluate(z);
    assert!(result);
}

#[test]
fn test_script_display() {
    let raw_sec = hex::decode("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap();
    let pubkey_commands = vec![raw_sec, vec![0xac]]; // SEC + OP_CHECKSIG (172)
    let script_pubkey = Script::new(pubkey_commands);

    let raw_sig = hex::decode("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601").unwrap();
    let script_sig = Script::new(vec![raw_sig]);
    let combined_script = script_sig.concat(script_pubkey);
    println!("{}", combined_script);
}

// TODO write script evaluation tests that include bad ops or non existent ops
// TODO write test cases for every standard transaction type

#[test]
fn test_sig_from_bytes() {
    // r goes from ef to 2c. s goes from c7 to b6 (the 01 byte is ommited)
    // 33 bytes for the s value
    // example values from p115 - coding script evaluation
    // let raw_sig = hex::decode("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601").unwrap();
    // Signature::sig_from_bytes(raw_sig);
    // should go from ed to 8f, then 7a to ed
    // 32 bytes for s value
    let example_sig = hex::decode("3045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed").unwrap();
    Signature::parse(example_sig);
}

#[test]
fn evaluate_basic_script() {
    let pubkey_commands = vec![
        vec![0x76], // 118 - OP_DUP
        vec![0x76], // 118 - OP_DUP
        vec![0x95], // 149 - OP_MUL
        vec![0x93], // 147 - OP_ADD
        vec![0x56], // 86 - OP_6
        vec![0x87], // 136 - OP_EQUAL
    ];

    let dummy_z = vec![0];
    let script_pubkey = Script::new(pubkey_commands);
    let script_sig_commands: Vec<Vec<u8>> = vec![vec![0x52]];
    let script_sig = Script::new(script_sig_commands);
    let combined_script = script_sig.concat(script_pubkey.clone());
    let result = combined_script.evaluate(dummy_z);
    assert!(result);
}
