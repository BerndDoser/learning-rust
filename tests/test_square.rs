use float_cmp::assert_approx_eq;
use learning_rust::square;

#[test]
fn test_square() {
    assert_eq!(square::square(5), 25);
    assert_eq!(square::square(-4), 16);
    assert_approx_eq!(f32, square::square(2.7), 7.29, ulps = 2);
}
