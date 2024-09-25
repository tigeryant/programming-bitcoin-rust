use ch_1::*;

#[test]
fn a_equals_itself() {
    let element_a = FieldElement::new(3, 13);
    assert!(&element_a == &element_a);
}

#[test]
fn a_not_equals_b() {
    let element_a = FieldElement::new(3, 13);
    let element_b = FieldElement::new(12, 13);
    assert!(!(&element_a == &element_b));
}

#[test]
fn a_add_b_eq_c() {
    let element_a = FieldElement::new(7, 13);
    let element_b = FieldElement::new(12, 13);
    let element_c = FieldElement::new(6, 13);
    assert!(&(&element_a + &element_b) == &element_c);
}

#[test]
fn a_sub_b_eq_c() {
    let element_a = FieldElement::new(12, 13);
    let element_b = FieldElement::new(7, 13);
    let element_c = FieldElement::new(5, 13);
    assert!(&(&element_a - &element_b) == &element_c);
}

#[test]
fn a_mul_b_eq_c() {
    let element_a = FieldElement::new(3, 13);
    let element_b = FieldElement::new(12, 13);
    let element_c = FieldElement::new(10, 13);
    assert!(&(&element_a * &element_b) == &element_c);
}

#[test]
fn a_pow_3_eq_b() {
    let element_a = FieldElement::new(3, 13);
    let element_b = FieldElement::new(1, 13);
    assert!(&element_a.pow(3) == &element_b);
}

#[test]
fn a_div_b_eq_c() {
    let element_a = FieldElement::new(7, 19);
    let element_b = FieldElement::new(5, 19);
    let element_c = FieldElement::new(9, 19);
    assert!(&(&element_a / &element_b) == &element_c);
}

#[test]
fn a_pow_negative_exponent() {
    let element_a = FieldElement::new(7, 13);
    let element_b = FieldElement::new(8, 13);
    assert!(&element_a.pow(-3) == &element_b);
}
