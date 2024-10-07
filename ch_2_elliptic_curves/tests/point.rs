use ch_2_elliptic_curves::*;
use num_bigint::ToBigInt;

#[test]
fn creates_valid_point() {
    let x = Some(-(1_i32.to_bigint().unwrap()));
    let y = Some(-(1_i32.to_bigint().unwrap()));
    let a = 5_i32.to_bigint().unwrap();
    let b = 7_i32.to_bigint().unwrap();
    let _point = Point::new(x, y, a, b);
}

#[test]
#[should_panic]
fn creates_invalid_point() {
    let x = Some(-(1_i32.to_bigint().unwrap()));
    let y = Some(-(2_i32.to_bigint().unwrap()));
    let a = 5_i32.to_bigint().unwrap();
    let b = 7_i32.to_bigint().unwrap();
    let _point = Point::new(x, y, a, b);
}

// create infinity point
#[test]
fn creates_infinity_point() {
    let x = None;
    let y = None;
    let a = 5_i32.to_bigint().unwrap();
    let b = 7_i32.to_bigint().unwrap();
    let _point = Point::new(x, y, a, b);
}

#[test]
fn a_equals_b() {
    let x_point_a = Some(-(1_i32.to_bigint().unwrap()));
    let y_point_a = Some(-(1_i32.to_bigint().unwrap()));
    let a_point_a = 5_i32.to_bigint().unwrap();
    let b_point_a = 7_i32.to_bigint().unwrap();
    let point_a = Point::new(x_point_a, y_point_a, a_point_a, b_point_a);

    let x_point_b = Some(-(1_i32.to_bigint().unwrap()));
    let y_point_b = Some(-(1_i32.to_bigint().unwrap()));
    let a_point_b = 5_i32.to_bigint().unwrap();
    let b_point_b = 7_i32.to_bigint().unwrap();
    let point_b = Point::new(x_point_b, y_point_b, a_point_b, b_point_b);

    assert!(&point_a == &point_b);
}

#[test]
fn a_not_equals_b() {
    let x_point_a = Some(-(1_i32.to_bigint().unwrap()));
    let y_point_a = Some(-(1_i32.to_bigint().unwrap()));
    let a_point_a = 5_i32.to_bigint().unwrap();
    let b_point_a = 7_i32.to_bigint().unwrap();
    let point_a = Point::new(x_point_a, y_point_a, a_point_a, b_point_a);

    let x_point_b = Some(18_i32.to_bigint().unwrap());
    let y_point_b = Some(77_i32.to_bigint().unwrap());
    let a_point_b = 5_i32.to_bigint().unwrap();
    let b_point_b = 7_i32.to_bigint().unwrap();
    let point_b = Point::new(x_point_b, y_point_b, a_point_b, b_point_b);

    assert!(&point_a != &point_b);
}

// test addition, look at all the add for ideas on how to test
