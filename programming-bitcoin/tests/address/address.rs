#[test]
fn test_h160_to_p2sh_address() {
    let address = address::h160_to_p2sh_address("74d691da1574e6b3c192ecfb52cc8984ee7b6c56", false);
    let expected = String::from("3CLoMMyuoDQTPRD3XYZtCvgvkadrAdvdXh");
    assert_eq!(expected, address);
}

#[test]
fn test_h160_to_p2pkh_address() {
    let address = address::h160_to_p2pkh_address("74d691da1574e6b3c192ecfb52cc8984ee7b6c56", false);
    let expected = String::from("1Cdid9KFAaatwczBwBttQcwXYCpvK8h7FK");
    assert_eq!(expected, address);
}
