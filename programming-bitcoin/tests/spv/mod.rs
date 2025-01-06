use std::io::Cursor;

use programming_bitcoin::{
    blocks::block::Block,
    spv::{
        merkle_tree::MerkleTree,
        utils::{merkle_parent, merkle_parent_level, merkle_root},
    },
};

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

    let expected =
        hex::decode("654d6181e18e4ac4368383fdc5eead11bf138f9b7ac1e15334e4411b3c4797d9").unwrap();

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

#[test]
fn test_tree_traversal() {
    let hashes = vec![
        "9745f7173ef14ee4155722d1cbf13304339fd00d900b759c6f9d58579b5765fb",
        "5573c8ede34936c29cdfdfe743f7f5fdfbd4f54ba0705259e62f39917065cb9b",
        "82a02ecbb6623b4274dfcab82b336dc017a27136e08521091e443e62582e8f05",
        "507ccae5ed9b340363a0e6d765af148be9cb1c8766ccc922f83e4ae681658308",
        "a7a4aec28e7162e1e9ef33dfa30f0bc0526e6cf4b11a576f6c5de58593898330",
        "bb6267664bd833fd9fc82582853ab144fece26b7a8a5bf328f8a059445b59add",
        "ea6d7ac1ee77fbacee58fc717b990c4fcccf1b19af43103c090f601677fd8836",
        "457743861de496c429912558a106b810b0507975a49773228aa788df40730d41",
        "7688029288efc9e9a0011c960a6ed9e5466581abf3e3a6c26ee317461add619a",
        "b1ae7f15836cb2286cdd4e2c37bf9bb7da0a2846d06867a429f654b2e7f383c9",
        "9b74f89fa3f93e71ff2c241f32945d877281a6a50a6bf94adac002980aafe5ab",
        "b3a92b5b255019bdaf754875633c2de9fec2ab03e6b8ce669d07cb5b18804638",
        "b5c0b915312b9bdaedd2b86aa2d0f8feffc73a2d37668fd9010179261e25e263",
        "c9d52c5cb1e557b92c84c52e7c4bfbce859408bedffc8a5560fd6e35e10b8800",
        "c555bc5fc3bc096df0a0c9532f07640bfb76bfe4fc1ace214b8b228a1297a4c2",
        "f9dbfafc3af3400954975da24eb325e326960a25b87fffe23eef3e7ed2fb610e",
    ];

    let hashes = hashes
        .into_iter()
        .map(|hash| hex::decode(hash).unwrap())
        .collect::<Vec<Vec<u8>>>();

    let mut tree = MerkleTree::new(hashes.len() as u32);

    let wrapped_hashes = hashes.into_iter().map(Some).collect();
    tree.nodes[4] = wrapped_hashes;

    while tree.root().is_none() {
        if tree.is_leaf() {
            if tree.current_depth > 0 {
                tree.up();
            }
        } else {
            let left_hash = tree.get_left_node();
            let right_hash = tree.get_right_node();
            if left_hash.is_none() {
                tree.left()
            } else if right_hash.is_none() {
                tree.right()
            } else {
                tree.set_current_node(Some(merkle_parent(left_hash.unwrap(), right_hash.unwrap())));
                if tree.current_depth > 0 {
                    tree.up();
                }
            }
            
        }
    }

    println!("{tree}");
}
