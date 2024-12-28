use programming_bitcoin::spv::utils::merkle_parent;

#[test]
fn test_merkle_parent() {
    let hash_0 = hex::decode("c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5").unwrap();
    let hash_1 = hex::decode("c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5").unwrap();
    let parent = merkle_parent(hash_0, hash_1);
    let expected = hex::decode("8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd").unwrap();
    assert_eq!(parent, expected);
}
