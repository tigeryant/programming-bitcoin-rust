use programming_bitcoin::script::btc_script::Script;
use std::io::Cursor;

#[test]
pub fn evaluate_script() {
    let z = hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap();
    // let mut stream = Cursor::new(z);
    // this needs a script instance to evaluate on. Do not use the scruct name Script, but an instance
    // evaluate on a combined script_pubkey and script_sig

    let raw_sec = hex::decode("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap();
    let pubkey_commands = vec![raw_sec, vec![0xac]]; // SEC + OP_CHECKSIG (172)
    let script_pubkey = Script::new(Some(pubkey_commands));

    let raw_sig = hex::decode("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601").unwrap();
    let script_sig = Script::new(Some(vec![raw_sig]));
    let combined_script = script_sig.concat(script_pubkey);

    let result = combined_script.evaluate(z);
    dbg!(result);
    // the current order is the signature (sig), the pubkey (sec), and then the opcode (OP_CHECKSIG)
    // since evaluate pops from index 0, it will read the sig, then the pubkey, then the op_code
}

// write script evaluation tests that include bad ops or non existent ops
