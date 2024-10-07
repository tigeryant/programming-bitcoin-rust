use ecc::field_element::*;
use primitive_types::U256;

#[test]
fn a_equals_itself() {
    let element_a = FieldElement::new(U256::from(3), U256::from(13));
    assert!(&element_a == &element_a);
}

#[test]
fn a_not_equals_b() {
    let element_a = FieldElement::new(U256::from(3), U256::from(13));
    let element_b = FieldElement::new(U256::from(12), U256::from(13));
    assert!(!(&element_a == &element_b));
}

#[test]
fn a_add_b_eq_c() {
    let element_a = FieldElement::new(U256::from(7), U256::from(13));
    let element_b = FieldElement::new(U256::from(12), U256::from(13));
    let element_c = FieldElement::new(U256::from(6), U256::from(13));
    assert!(&(&element_a + &element_b) == &element_c);
}

#[test]
fn a_sub_b_eq_c() {
    let element_a = FieldElement::new(U256::from(12), U256::from(13));
    let element_b = FieldElement::new(U256::from(7), U256::from(13));
    let element_c = FieldElement::new(U256::from(5), U256::from(13));
    assert_eq!(&(&element_a - &element_b), &element_c);
}

#[test]
fn a_mul_b_eq_c() {
    let element_a = FieldElement::new(U256::from(3), U256::from(13));
    let element_b = FieldElement::new(U256::from(12), U256::from(13));
    let element_c = FieldElement::new(U256::from(10), U256::from(13));
    assert_eq!(&(&element_a * &element_b), &element_c);
}

/*
#[test]
#[ignore]
fn a_div_b_eq_c() {
    let element_a = FieldElement::new(U256::from(7), U256::from(19));
    let element_b = FieldElement::new(U256::from(5), U256::from(19));
    let element_c = FieldElement::new(U256::from(9), U256::from(19));
    assert_eq!(&(&element_a / &element_b), &element_c);
}
 */
