use ch_1::*;
use ch_1::Pow;

fn main() {
    let element_a = FieldElement::new(3, 13);
    let element_a_clone = FieldElement::new(3, 13);
    let element_b = FieldElement::new(12, 13);
    let element_c = FieldElement::new(10, 13);
    // let element_d = FieldElement::new(1, 13);
    println!("element_a: {:?}", element_a);
    
    let a_equals_b = element_a == element_b;
    let a_equals_a = element_a == element_a_clone;
    let a_not_equals_b = element_a != element_b;
    let a_not_equals_a = element_a != element_a_clone;

    println!("element_a == element_b: {}", a_equals_b); // false
    assert!(!a_equals_b);
    println!("element_a == element_a: {}", a_equals_a); // true
    assert!(a_equals_a);
    println!("element_a != element_b: {}", a_not_equals_b); // true
    assert!(a_not_equals_b);
    println!("element_a != element_a: {}", a_not_equals_a); // false
    assert!(!a_not_equals_a);

    let a_mul_b_eq_c = (element_a * element_b) == element_c;
    println!("element_a * element_b = element_c: {}", a_mul_b_eq_c); // true
    assert!(a_mul_b_eq_c);

    let element_e = FieldElement::new(3, 13);
    let element_f = FieldElement::new(1, 13);
    assert!((element_e.pow(3)) == element_f);
}
