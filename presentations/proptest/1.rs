// this property is false, but perhaps
// not unreasonable to expect to be true
proptest! {
  #[test]
  fn mult_and_div(ref a in any::<usize>()) {
    let result = (a * 5) / 5;
    assert_eq!(result, a);
  }
}
