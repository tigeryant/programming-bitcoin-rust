use ch_1::*;

fn main() {
    let element_a = FieldElement::new(1, 97);
    let element_a_clone = FieldElement::new(1, 97);
    let element_b = FieldElement::new(29, 97);
    println!("element_a: {:?}", element_a);
    
    let a_equals_b = element_a == element_b;
    let a_equals_a = element_a == element_a_clone;
    let a_not_equals_b = element_a != element_b;
    let a_not_equals_a = element_a != element_a_clone;

    println!("element_a == element_b: {}", a_equals_b);
    println!("element_a == element_a: {}", a_equals_a);
    println!("element_a != element_b: {}", a_not_equals_b);
    println!("element_a != element_a: {}", a_not_equals_a);
}
