use calc::{eval::eval, parse::parse};

#[test]
fn round_trip_sqr() {
    let text = "3 sqr";
    let expected = 9i64;
    let parsed = parse(text).unwrap();
    let evald = eval(&parsed).unwrap();
    assert_eq!(expected, evald);
}
