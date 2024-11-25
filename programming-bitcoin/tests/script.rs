use programming_bitcoin::{ecc::signature::Signature, script::btc_script::Script};
// use std::io::Cursor;

#[test]
pub fn evaluate_script() {
    let z = hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap();
    // let mut stream = Cursor::new(z);
    let raw_sec = hex::decode("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap();
    let pubkey_commands = vec![raw_sec, vec![0xac]]; // SEC + OP_CHECKSIG (172)
    let script_pubkey = Script::new(Some(pubkey_commands));

    let raw_sig = hex::decode("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601").unwrap();
    let script_sig = Script::new(Some(vec![raw_sig]));
    let combined_script = script_sig.concat(script_pubkey);

    let result = combined_script.evaluate(z);
    assert!(result);
}

// write script evaluation tests that include bad ops or non existent ops

#[test]
fn test_sig_from_bytes() { // should go from ef to 2c, then c7 to b6 (NOT 01) - NOTE - this should finish with b6, not 01
    // note that the 01 on the end may be the sighash type
    // how many bytes for the s value? 33
    // these values are from p115 - coding script evaluation
    // let raw_sig = hex::decode("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601").unwrap();
    // Signature::sig_from_bytes(raw_sig);
    // should go from ed to 8f, then 7a to ed
    // how many bytes for the s value? 32
    let example_sig = hex::decode("3045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed").unwrap();
    Signature::sig_from_bytes(example_sig);
} 
