use programming_bitcoin::utils::rng;

#[test]
fn random_u256() {
    let random_number = rng::get_random_u256();
    println!("Random U256: {random_number}");
}
