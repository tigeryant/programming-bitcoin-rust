use programming_bitcoin::spv::utils::{merkle_parent, merkle_parent_level};

#[test]
fn test_merkle_parent() {
    let hash_0 = hex::decode("c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5").unwrap();
    let hash_1 = hex::decode("c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5").unwrap();
    let parent = merkle_parent(hash_0, hash_1);
    let expected = hex::decode("8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd").unwrap();
    assert_eq!(parent, expected);
}

// MOVE THIS TO THE METHOD DEFINITION - just declare hashes and pass it to the method
#[test]
fn test_merkle_parent_level() {
    let hashes = [
        "c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5",
        "c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5",
        "f391da6ecfeed1814efae39e7fcb3838ae0b02c02ae7d0a5848a66947c0727b0",
        "3d238a92a94532b946c90e19c49351c763696cff3db400485b813aecb8a13181",
        "10092f2633be5f3ce349bf9ddbde36caa3dd10dfa0ec8106bce23acbff637dae",
    ];

    let hashes = hashes
        .into_iter()
        .map(|hash| hex::decode(hash).unwrap())
        .collect::<Vec<Vec<u8>>>();

    let parent_level = merkle_parent_level(hashes);

    for node in &parent_level {
        dbg!(hex::encode(node));
    }

    let node_a_expected = hex::decode("8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd").unwrap();
    let node_b_expected = hex::decode("7f4e6f9e224e20fda0ae4c44114237f97cd35aca38d83081c9bfd41feb907800").unwrap();
    let node_c_expected = hex::decode("3ecf6115380c77e8aae56660f5634982ee897351ba906a6837d15ebc3a225df0").unwrap();
    assert_eq!(node_a_expected, parent_level[0]);
    assert_eq!(node_b_expected, parent_level[1]);
    assert_eq!(node_c_expected, parent_level[2]);
}
