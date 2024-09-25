use ch_2_elliptic_curves::*;

#[test]
fn creates_valid_point() {
    let point = Point::new(-1, -1, 5, 7);
    assert!(&point == &point);
}

#[test]
#[should_panic]
fn creates_invalid_point() {
    let _invalid_point = Point::new(-1, -2, 5, 7);
}