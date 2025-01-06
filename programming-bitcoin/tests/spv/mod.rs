use std::io::Cursor;

use programming_bitcoin::{blocks::block::Block, spv::{merkle_tree::MerkleTree, utils::{merkle_parent, merkle_parent_level, merkle_root}}};

#[test]
fn test_merkle_parent() {
    let hash_0 =
        hex::decode("c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5").unwrap();
    let hash_1 =
        hex::decode("c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5").unwrap();
    let parent = merkle_parent(hash_0, hash_1);
    let expected =
        hex::decode("8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd").unwrap();
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

    let node_a_expected =
        hex::decode("8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd").unwrap();
    let node_b_expected =
        hex::decode("7f4e6f9e224e20fda0ae4c44114237f97cd35aca38d83081c9bfd41feb907800").unwrap();
    let node_c_expected =
        hex::decode("3ecf6115380c77e8aae56660f5634982ee897351ba906a6837d15ebc3a225df0").unwrap();
    assert_eq!(node_a_expected, parent_level[0]);
    assert_eq!(node_b_expected, parent_level[1]);
    assert_eq!(node_c_expected, parent_level[2]);
}

#[test]
fn test_merkle_root() {
    let hashes: Vec<&str> = vec![
        "c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5",
        "c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5",
        "f391da6ecfeed1814efae39e7fcb3838ae0b02c02ae7d0a5848a66947c0727b0",
        "3d238a92a94532b946c90e19c49351c763696cff3db400485b813aecb8a13181",
        "10092f2633be5f3ce349bf9ddbde36caa3dd10dfa0ec8106bce23acbff637dae",
        "7d37b3d54fa6a64869084bfd2e831309118b9e833610e6228adacdbd1b4ba161",
        "8118a77e542892fe15ae3fc771a4abfd2f5d5d5997544c3487ac36b5c85170fc",
        "dff6879848c2c9b62fe652720b8df5272093acfaa45a43cdb3696fe2466a3877",
        "b825c0745f46ac58f7d3759e6dc535a1fec7820377f24d4c2c6ad2cc55c0cb59",
        "95513952a04bd8992721e9b7e2937f1c04ba31e0469fbe615a78197f68f52b7c",
        "2e6d722e5e4dbdf2447ddecc9f7dabb8e299bae921c99ad5b0184cd9eb8e5908",
        "b13a750047bc0bdceb2473e5fe488c2596d7a7124b4e716fdd29b046ef99bbf0",
    ];

    let hashes = hashes
        .into_iter()
        .map(|hash| hex::decode(hash).unwrap())
        .collect::<Vec<Vec<u8>>>();

    let merkle_root = merkle_root(hashes);

    let expected =
        hex::decode("acbcab8bcc1af95d8d563b77d24c3d19b18f1486383d75a5085c4e86c86beed6").unwrap();
    assert_eq!(merkle_root, expected);
}

#[test]
fn test_merkle_root_endianness() {
    let hashes_le: Vec<&str> = vec![
        "42f6f52f17620653dcc909e58bb352e0bd4bd1381e2955d19c00959a22122b2e",
        "94c3af34b9667bf787e1c6a0a009201589755d01d02fe2877cc69b929d2418d4",
        "959428d7c48113cb9149d0566bde3d46e98cf028053c522b8fa8f735241aa953",
        "a9f27b99d5d108dede755710d4a1ffa2c74af70b4ca71726fa57d68454e609a2",
        "62af110031e29de1efcad103b3ad4bec7bdcf6cb9c9f4afdd586981795516577",
        "766900590ece194667e9da2984018057512887110bf54fe0aa800157aec796ba",
        "e8270fb475763bc8d855cfe45ed98060988c1bdcad2ffc8364f783c98999a208",
    ];

    // Convert each hash to Vec<u8> and switch from little to big endian
    let hashes_be = hashes_le
        .into_iter()
        .map(|hash| {
            let mut bytes = hex::decode(hash).unwrap();
            bytes.reverse();
            bytes
        })
        .collect::<Vec<Vec<u8>>>();

    let mut merkle_root = merkle_root(hashes_be);

    // Reverse byte order from big to little endian
    merkle_root.reverse();

    let expected = hex::decode("654d6181e18e4ac4368383fdc5eead11bf138f9b7ac1e15334e4411b3c4797d9").unwrap();

    assert_eq!(merkle_root, expected);
}

#[test]
fn test_validate_merkle_root() {
    // create a block (from a raw serialization)
    // call validate merkle root and assert it's true

    let raw_block = hex::decode("01000000a0d4ea3416518af0b238fef847274fc768cd39d0dc44a0ea5ec0c2dd000000007edfbf7974109f1fd628f17dfefd4915f217e0ec06e0c74e45049d36850abca4bc0eb049ffff001d27d0031e0101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0804ffff001d024f02ffffffff0100f2052a010000004341048a5294505f44683bbc2be81e0f6a91ac1a197d6050accac393aad3b86b2398387e34fedf0de5d9f185eb3f2c17f3564b9170b9c262aa3ac91f371279beca0cafac00000000").unwrap();
    let mut reader = Cursor::new(raw_block);
    let block = Block::parse(&mut reader).unwrap();

    assert!(block.validate_merkle_root());
}

#[test]
fn default_empty_merkle_tree() {
    let tree = MerkleTree::new(27);
    println!("{tree}");
    for level in tree.nodes {
        for node in level {
            assert!(node.is_none())
        }
    }
}
